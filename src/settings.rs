use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SortMode {
    Memory,
    Cpu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub sort_mode: SortMode,
    pub chart_window_seconds: u32,
    pub refresh_interval_ms: u64,
    pub hotkey_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sort_mode: SortMode::Memory,
            chart_window_seconds: 120,
            refresh_interval_ms: 1000,
            hotkey_enabled: true,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str(&contents) {
                    return settings;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let contents = serde_json::to_string_pretty(self)?;
            fs::write(&path, contents)?;
        }
        Ok(())
    }

    fn config_path() -> Option<PathBuf> {
        directories::ProjectDirs::from("com", "memwatch", "memwatch")
            .map(|dirs| dirs.config_dir().join("settings.json"))
    }
}
