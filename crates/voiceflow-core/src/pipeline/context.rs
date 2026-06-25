use crate::pipeline::request::FormattingMode;
use voiceflow_shared::config::vocabulary::UserVocabulary;

#[derive(Debug, Clone)]
pub struct FormatterContext {
    pub mode: FormattingMode,
    pub vocabulary: Option<UserVocabulary>,
}

impl FormatterContext {
    pub fn new(mode: FormattingMode, vocabulary: Option<UserVocabulary>) -> Self {
        Self {
            mode,
            vocabulary,
        }
    }
}

impl Default for FormatterContext {
    fn default() -> Self {
        Self {
            mode: FormattingMode::Smart,
            vocabulary: None,
        }
    }
}
