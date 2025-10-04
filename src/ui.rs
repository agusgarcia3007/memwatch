use crate::hotkey::HotkeyManager;
use crate::ipc::IpcServer;
use crate::killer::{force_kill_process, terminate_process, KillStatus};
use crate::metrics::{MetricsCollector, ProcessInfo};
use crate::settings::{Settings, SortMode};
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::time::{Duration, Instant};

pub struct MemwatchApp {
    metrics: MetricsCollector,
    processes: Vec<ProcessInfo>,
    settings: Settings,
    last_refresh: Instant,
    search_filter: String,
    kill_confirmation: Option<u32>,
    notification: Option<(String, Instant, NotificationLevel)>,
    show_settings: bool,
    hotkey_manager: Option<HotkeyManager>,
    ipc_server: Option<IpcServer>,
    window_visible: bool,
}

#[derive(Clone, Copy)]
enum NotificationLevel {
    Info,
    Success,
    Error,
}

impl MemwatchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut metrics = MetricsCollector::new();
        metrics.refresh();
        let processes = metrics.get_processes();
        let settings = Settings::load();

        let hotkey_manager = HotkeyManager::new();
        let ipc_server = IpcServer::new();

        Self {
            metrics,
            processes,
            settings,
            last_refresh: Instant::now(),
            search_filter: String::new(),
            kill_confirmation: None,
            notification: None,
            show_settings: false,
            hotkey_manager,
            ipc_server,
            window_visible: true,
        }
    }

    fn show_notification(&mut self, message: String, level: NotificationLevel) {
        self.notification = Some((message, Instant::now(), level));
    }

    fn render_notification(&mut self, ctx: &egui::Context) {
        if let Some((message, start_time, level)) = &self.notification {
            if start_time.elapsed() > Duration::from_secs(5) {
                self.notification = None;
                return;
            }

            let (bg_color, text_color) = match level {
                NotificationLevel::Info => (egui::Color32::from_rgb(70, 130, 180), egui::Color32::WHITE),
                NotificationLevel::Success => (egui::Color32::from_rgb(60, 179, 113), egui::Color32::WHITE),
                NotificationLevel::Error => (egui::Color32::from_rgb(220, 53, 69), egui::Color32::WHITE),
            };

            egui::TopBottomPanel::top("notification")
                .frame(egui::Frame::none().fill(bg_color))
                .show(ctx, |ui| {
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(message).color(text_color));
                    });
                    ui.add_space(4.0);
                });
        }
    }

    fn render_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Sort by:");
            if ui.selectable_label(self.settings.sort_mode == SortMode::Memory, "Memory").clicked() {
                self.settings.sort_mode = SortMode::Memory;
                let _ = self.settings.save();
            }
            if ui.selectable_label(self.settings.sort_mode == SortMode::Cpu, "CPU").clicked() {
                self.settings.sort_mode = SortMode::Cpu;
                let _ = self.settings.save();
            }

            ui.separator();

            ui.label("Filter:");
            ui.text_edit_singleline(&mut self.search_filter);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⚙ Settings").clicked() {
                    self.show_settings = !self.show_settings;
                }
            });
        });
    }

    fn render_process_list(&mut self, ui: &mut egui::Ui) {
        let mut sorted_processes = self.processes.clone();

        if !self.search_filter.is_empty() {
            let filter_lower = self.search_filter.to_lowercase();
            sorted_processes.retain(|p| {
                p.name.to_lowercase().contains(&filter_lower)
                    || p.pid.to_string().contains(&filter_lower)
            });
        }

        match self.settings.sort_mode {
            SortMode::Memory => {
                sorted_processes.sort_by(|a, b| {
                    b.memory_mb
                        .partial_cmp(&a.memory_mb)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            SortMode::Cpu => {
                sorted_processes.sort_by(|a, b| {
                    b.cpu_usage
                        .partial_cmp(&a.cpu_usage)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            use egui_extras::{Column, TableBuilder};

            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .column(Column::auto().at_least(250.0))
                .column(Column::auto().at_least(80.0))
                .column(Column::auto().at_least(100.0))
                .column(Column::auto().at_least(100.0))
                .column(Column::auto().at_least(100.0))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Process Name");
                    });
                    header.col(|ui| {
                        ui.strong("PID");
                    });
                    header.col(|ui| {
                        ui.strong("CPU %");
                    });
                    header.col(|ui| {
                        ui.strong("Memory (MB)");
                    });
                    header.col(|ui| {
                        ui.strong("Action");
                    });
                })
                .body(|mut body| {
                    for process in sorted_processes.iter().take(100) {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.label(&process.name);
                            });
                            row.col(|ui| {
                                ui.label(process.pid.to_string());
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.1}%", process.cpu_usage));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.1}", process.memory_mb));
                            });
                            row.col(|ui| {
                                if self.kill_confirmation == Some(process.pid) {
                                    ui.horizontal(|ui| {
                                        if ui.small_button("Confirm Kill").clicked() {
                                            let pid = process.pid;
                                            match force_kill_process(pid) {
                                                KillStatus::Success => {
                                                    self.show_notification(
                                                        format!("SIGKILL sent to process {}", pid),
                                                        NotificationLevel::Success,
                                                    );
                                                    self.kill_confirmation = None;
                                                }
                                                KillStatus::Failed(err) => {
                                                    self.show_notification(
                                                        format!("Failed to kill {}: {}", pid, err),
                                                        NotificationLevel::Error,
                                                    );
                                                    self.kill_confirmation = None;
                                                }
                                                KillStatus::NotFound => {
                                                    self.show_notification(
                                                        format!("Process {} not found", pid),
                                                        NotificationLevel::Info,
                                                    );
                                                    self.kill_confirmation = None;
                                                }
                                                _ => {}
                                            }
                                        }
                                        if ui.small_button("Cancel").clicked() {
                                            self.kill_confirmation = None;
                                        }
                                    });
                                } else if ui.small_button("Force Quit").clicked() {
                                    let pid = process.pid;
                                    match terminate_process(pid) {
                                        KillStatus::Success => {
                                            self.show_notification(
                                                format!("Process {} terminated successfully", pid),
                                                NotificationLevel::Success,
                                            );
                                        }
                                        KillStatus::RequiresConfirmation(_) => {
                                            self.show_notification(
                                                format!(
                                                    "Process {} did not respond to SIGTERM. Confirm SIGKILL?",
                                                    pid
                                                ),
                                                NotificationLevel::Info,
                                            );
                                            self.kill_confirmation = Some(pid);
                                        }
                                        KillStatus::Failed(err) => {
                                            self.show_notification(
                                                format!("Failed to terminate {}: {}", pid, err),
                                                NotificationLevel::Error,
                                            );
                                        }
                                        KillStatus::NotFound => {
                                            self.show_notification(
                                                format!("Process {} not found", pid),
                                                NotificationLevel::Info,
                                            );
                                        }
                                    }
                                }
                            });
                        });
                    }
                });
        });
    }

    fn render_chart(&self, ui: &mut egui::Ui) {
        ui.heading("Resource Usage");

        let cpu_data = self.metrics.get_cpu_history();
        let memory_data = self.metrics.get_memory_history();

        let window_seconds = self.settings.chart_window_seconds as f64;

        let current_time = if let Some(last) = cpu_data.last() {
            last.0
        } else {
            0.0
        };

        let start_time = (current_time - window_seconds).max(0.0);

        let cpu_line: PlotPoints = cpu_data
            .iter()
            .filter(|(t, _)| *t >= start_time)
            .map(|(t, cpu)| [*t, *cpu as f64])
            .collect();

        let memory_line: PlotPoints = memory_data
            .iter()
            .filter(|(t, _)| *t >= start_time)
            .map(|(t, mem)| [*t, *mem])
            .collect();

        Plot::new("resource_chart")
            .view_aspect(2.5)
            .legend(egui_plot::Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(
                    Line::new(cpu_line)
                        .name("CPU %")
                        .color(egui::Color32::from_rgb(75, 150, 220)),
                );
                plot_ui.line(
                    Line::new(memory_line)
                        .name(format!("Memory (GB) / {:.1} GB total", self.metrics.get_total_memory_gb()))
                        .color(egui::Color32::from_rgb(255, 140, 0)),
                );
            });
    }

    fn render_settings(&mut self, ctx: &egui::Context) {
        if !self.show_settings {
            return;
        }

        let mut should_close = false;
        egui::Window::new("Settings")
            .open(&mut self.show_settings)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Chart Settings");

                ui.horizontal(|ui| {
                    ui.label("Chart window (seconds):");
                    let mut window = self.settings.chart_window_seconds as i32;
                    if ui.add(egui::Slider::new(&mut window, 60..=300)).changed() {
                        self.settings.chart_window_seconds = window as u32;
                        let _ = self.settings.save();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Refresh interval (ms):");
                    let mut interval = self.settings.refresh_interval_ms as i32;
                    if ui.add(egui::Slider::new(&mut interval, 500..=2000)).changed() {
                        self.settings.refresh_interval_ms = interval as u64;
                        let _ = self.settings.save();
                    }
                });

                ui.separator();

                ui.heading("Hotkey");
                if ui.checkbox(&mut self.settings.hotkey_enabled, "Enable global hotkey (⌥⌘M)").changed() {
                    let _ = self.settings.save();
                }

                if self.hotkey_manager.is_some() {
                    ui.label("Press ⌥⌘M anywhere to toggle window");
                } else {
                    ui.label("⚠ Global hotkey not available on this platform");
                }

                ui.label("You can also use 'memwatch toggle' from CLI");

                if ui.button("Close").clicked() {
                    should_close = true;
                }
            });

        if should_close {
            self.show_settings = false;
        }
    }

    fn toggle_window(&mut self, ctx: &egui::Context) {
        self.window_visible = !self.window_visible;

        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(self.window_visible));

        if self.window_visible {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }
    }
}

impl eframe::App for MemwatchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for global hotkey: Option+Command+M
        if self.settings.hotkey_enabled {
            ctx.input(|i| {
                if i.modifiers.command && i.modifiers.alt && !i.modifiers.shift && !i.modifiers.ctrl {
                    if i.key_pressed(egui::Key::M) {
                        self.toggle_window(ctx);
                    }
                }
            });
        }

        if let Some(ref ipc) = self.ipc_server {
            if let Some(msg) = ipc.check_message() {
                if msg == "toggle" {
                    self.toggle_window(ctx);
                }
            }
        }

        if self.last_refresh.elapsed() >= Duration::from_millis(self.settings.refresh_interval_ms) {
            self.metrics.refresh();
            self.processes = self.metrics.get_processes();
            self.last_refresh = Instant::now();
        }

        self.render_notification(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_toolbar(ui);

            ui.separator();

            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), ui.available_height() * 0.65),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    self.render_process_list(ui);
                },
            );

            ui.separator();

            self.render_chart(ui);
        });

        if self.show_settings {
            self.render_settings(ctx);
        }

        ctx.request_repaint_after(Duration::from_millis(self.settings.refresh_interval_ms));
    }
}
