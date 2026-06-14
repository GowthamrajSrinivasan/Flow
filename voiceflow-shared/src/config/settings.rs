use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub hotkey: String,
    pub language: String,
    pub overlay: bool,
    pub auto_inject: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            hotkey: "Ctrl+Shift+Space".to_string(),
            language: "en".to_string(),
            overlay: true,
            auto_inject: true,
        }
    }
}

impl AppSettings {
    pub fn config_dir() -> Option<PathBuf> {
        directories::ProjectDirs::from("com", "voiceflow", "voiceflow").map(|dirs| dirs.config_dir().to_path_buf())
    }

    pub fn config_file() -> Option<PathBuf> {
        Self::config_dir().map(|dir| dir.join("settings.json"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_file() {
            if path.exists() {
                if let Ok(json) = std::fs::read_to_string(path) {
                    if let Ok(settings) = serde_json::from_str(&json) {
                        return settings;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(dir) = Self::config_dir() {
            std::fs::create_dir_all(&dir)?;
            let path = dir.join("settings.json");
            let json = serde_json::to_string_pretty(self)?;
            std::fs::write(path, json)?;
        }
        Ok(())
    }
}
