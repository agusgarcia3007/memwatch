use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_mb: f64,
}

pub struct MetricsCollector {
    system: System,
    last_update: Instant,
    cpu_history: Vec<(f64, f32)>,
    memory_history: Vec<(f64, f64)>,
    start_time: Instant,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let now = Instant::now();
        Self {
            system,
            last_update: now,
            cpu_history: Vec::with_capacity(300),
            memory_history: Vec::with_capacity(300),
            start_time: now,
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::new()
                .with_cpu()
                .with_memory(),
        );
        self.system.refresh_cpu_all();
        self.system.refresh_memory();

        self.last_update = Instant::now();

        let elapsed = self.last_update.duration_since(self.start_time).as_secs_f64();

        let total_cpu = self.system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>()
            / self.system.cpus().len() as f32;
        self.cpu_history.push((elapsed, total_cpu));

        let used_memory = self.system.used_memory();
        let used_gb = used_memory as f64 / 1_073_741_824.0;
        self.memory_history.push((elapsed, used_gb));

        if self.cpu_history.len() > 300 {
            self.cpu_history.remove(0);
        }
        if self.memory_history.len() > 300 {
            self.memory_history.remove(0);
        }
    }

    pub fn get_processes(&self) -> Vec<ProcessInfo> {
        let mut processes: Vec<ProcessInfo> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_mb: process.memory() as f64 / 1_048_576.0,
            })
            .collect();

        processes.sort_by(|a, b| {
            b.memory_mb
                .partial_cmp(&a.memory_mb)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        processes
    }

    pub fn get_cpu_history(&self) -> &[(f64, f32)] {
        &self.cpu_history
    }

    pub fn get_memory_history(&self) -> &[(f64, f64)] {
        &self.memory_history
    }

    pub fn get_total_memory_gb(&self) -> f64 {
        self.system.total_memory() as f64 / 1_073_741_824.0
    }
}
