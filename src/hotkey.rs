// Global hotkey support using objc2
// Note: This is a simplified implementation using polling rather than callbacks
// due to the complexity of objc2's block system

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct HotkeyManager {
    triggered: Arc<AtomicBool>,
}

impl HotkeyManager {
    pub fn new() -> Option<Self> {
        // Create a simple flag-based system
        // The actual hotkey detection will be handled by egui's event system
        Some(HotkeyManager {
            triggered: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn check_triggered(&self) -> bool {
        self.triggered.swap(false, Ordering::Relaxed)
    }

    pub fn trigger(&self) {
        self.triggered.store(true, Ordering::Relaxed);
    }
}
