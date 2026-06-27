use crate::pipeline::request::FormattingMode;
use voiceflow_shared::config::vocabulary::UserVocabulary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormattingProfile {
    General,
    Developer,
    Markdown,
    Email,
    Slack,
    Medical,
    Legal,
}

#[derive(Debug, Clone, Default)]
pub struct AppContext {
    pub active_application: String,
    pub platform: String,
    pub locale: String,
}

#[derive(Debug, Clone)]
pub struct UserContext {
    pub developer_mode: bool,
    pub markdown_enabled: bool,
    pub language: String,
    pub vocabulary: Option<UserVocabulary>,
}

impl Default for UserContext {
    fn default() -> Self {
        Self {
            developer_mode: false,
            markdown_enabled: false,
            language: "en".to_string(),
            vocabulary: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DocumentContext {
    pub current_buffer: String,
    pub cursor_position: Option<usize>,
    pub list_state: Option<String>,
}

// -----------------------------------------------------------------------------
// Legacy Types for Backward Compatibility (To be deprecated in Phase 4)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct FormattingContext {
    pub mode: FormattingMode,
    pub language: String,
    pub locale: String,
    pub markdown_enabled: bool,
    pub vocabulary: Option<UserVocabulary>,
}

impl FormattingContext {
    pub fn new(mode: FormattingMode, vocabulary: Option<UserVocabulary>) -> Self {
        Self {
            mode,
            vocabulary,
            ..Default::default()
        }
    }
}

impl Default for FormattingContext {
    fn default() -> Self {
        Self {
            mode: FormattingMode::Smart,
            language: "en".to_string(),
            locale: "en-US".to_string(),
            markdown_enabled: false,
            vocabulary: None,
        }
    }
}
