// Simplified hotkey manager - global hotkey disabled due to thread safety complexity
// with cocoa crate. Can be re-enabled with objc2 crate in future.

pub struct HotkeyManager;

impl HotkeyManager {
    pub fn new() -> Option<Self> {
        // Global hotkey disabled for now
        None
    }

    pub fn check_triggered(&self) -> bool {
        false
    }
}
