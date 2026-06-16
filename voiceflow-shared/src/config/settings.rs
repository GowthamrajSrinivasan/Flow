use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub version: u32,
    
    // General
    pub launch_at_login: bool,
    pub overlay_enabled: bool,
    pub theme: String,
    
    // Dictation
    pub hotkey: String,
    pub language: String,
    pub vocabulary_count: u32,
    
    // Injection
    pub auto_paste: bool,
    pub clipboard_fallback: bool,
    pub show_notifications: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            version: 1,
            launch_at_login: false,
            overlay_enabled: true,
            theme: "dark".to_string(),
            hotkey: "Alt+Space".to_string(),
            language: "en".to_string(),
            vocabulary_count: 0,
            auto_paste: true,
            clipboard_fallback: true,
            show_notifications: true,
        }
    }
}

impl AppSettings {
    pub fn config_dir() -> Option<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            std::env::var("HOME").ok().map(|h| PathBuf::from(h).join("Library/Application Support/VoiceFlow"))
        }
        #[cfg(not(target_os = "macos"))]
        {
            directories::ProjectDirs::from("com", "voiceflow", "voiceflow").map(|dirs| dirs.config_dir().to_path_buf())
        }
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

