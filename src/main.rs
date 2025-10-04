mod hotkey;
mod ipc;
mod killer;
mod metrics;
mod settings;
mod ui;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "toggle" => {
                if let Err(e) = ipc::send_toggle_command() {
                    eprintln!("Failed to send toggle command: {}. Is memwatch running?", e);
                    std::process::exit(1);
                }
                return;
            }
            "--help" | "-h" => {
                println!("memwatch - macOS process monitor");
                println!();
                println!("USAGE:");
                println!("  memwatch          Launch the GUI");
                println!("  memwatch toggle   Toggle window visibility");
                println!();
                println!("HOTKEY:");
                println!("  ⌥⌘M              Toggle window from anywhere (Option+Command+M)");
                println!();
                println!("NOTE:");
                println!("  You can also use 'memwatch toggle' from terminal");
                return;
            }
            _ => {
                eprintln!("Unknown command: {}", args[1]);
                eprintln!("Use 'memwatch --help' for usage information");
                std::process::exit(1);
            }
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("memwatch"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "memwatch",
        options,
        Box::new(|cc| Ok(Box::new(ui::MemwatchApp::new(cc)))),
    );
}
