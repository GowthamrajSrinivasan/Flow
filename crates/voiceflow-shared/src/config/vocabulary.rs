use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::config::settings::AppSettings;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VocabularyEntry {
    pub spoken: String,
    pub output: String,
    pub language: String,
    pub enabled: bool,
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVocabulary {
    pub entries: Vec<VocabularyEntry>,
}

impl Default for UserVocabulary {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl UserVocabulary {
    pub fn config_file() -> Option<PathBuf> {
        AppSettings::config_dir().map(|dir| dir.join("vocabulary.json"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_file() {
            if path.exists() {
                if let Ok(json) = std::fs::read_to_string(path) {
                    if let Ok(vocab) = serde_json::from_str(&json) {
                        return vocab;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(dir) = AppSettings::config_dir() {
            std::fs::create_dir_all(&dir)?;
            let path = dir.join("vocabulary.json");
            let json = serde_json::to_string_pretty(self)?;
            std::fs::write(path, json)?;
        }
        Ok(())
    }
}
