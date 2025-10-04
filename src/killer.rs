use libc::{kill, SIGKILL, SIGTERM};
use std::thread;
use std::time::Duration;
use sysinfo::{Pid, System};

#[derive(Debug, Clone, PartialEq)]
pub enum KillStatus {
    Success,
    RequiresConfirmation(u32),
    Failed(String),
    NotFound,
}

pub fn terminate_process(pid: u32) -> KillStatus {
    let pid_i32 = pid as i32;

    let mut system = System::new();
    system.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[Pid::from_u32(pid)]), true);

    if !system.processes().contains_key(&Pid::from_u32(pid)) {
        return KillStatus::NotFound;
    }

    unsafe {
        let result = kill(pid_i32, SIGTERM);
        if result != 0 {
            let error = std::io::Error::last_os_error();
            return KillStatus::Failed(format!("Failed to send SIGTERM: {}", error));
        }
    }

    thread::sleep(Duration::from_millis(1500));

    system.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[Pid::from_u32(pid)]), true);

    if !system.processes().contains_key(&Pid::from_u32(pid)) {
        return KillStatus::Success;
    }

    KillStatus::RequiresConfirmation(pid)
}

pub fn force_kill_process(pid: u32) -> KillStatus {
    let pid_i32 = pid as i32;

    let mut system = System::new();
    system.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[Pid::from_u32(pid)]), true);

    if !system.processes().contains_key(&Pid::from_u32(pid)) {
        return KillStatus::NotFound;
    }

    unsafe {
        let result = kill(pid_i32, SIGKILL);
        if result != 0 {
            let error = std::io::Error::last_os_error();
            return KillStatus::Failed(format!("Failed to send SIGKILL: {}", error));
        }
    }

    thread::sleep(Duration::from_millis(200));

    system.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[Pid::from_u32(pid)]), true);

    if !system.processes().contains_key(&Pid::from_u32(pid)) {
        KillStatus::Success
    } else {
        KillStatus::Failed("Process still running after SIGKILL".to_string())
    }
}
