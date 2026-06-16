use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub audio_device: Option<String>,
    pub enable_auto_punctuation: bool,
    pub language: String, // Keeping it simple for Phase 1 (en)
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            audio_device: None,
            enable_auto_punctuation: true,
            language: "en".to_string(),
        }
    }
}
