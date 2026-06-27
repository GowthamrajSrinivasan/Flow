#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormattingMode {
    Raw,
    Smart, // Equivalent to Writing
    Email,
    Chat,
    Document,
    Developer,
    Markdown,
}

impl Default for FormattingMode {
    fn default() -> Self {
        FormattingMode::Smart
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteMode {
    Off,
    Light,
    Standard,
    Aggressive,
}

impl Default for RewriteMode {
    fn default() -> Self {
        RewriteMode::Off
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProcessingOptions {
    pub formatting_mode: FormattingMode,
    pub rewrite_mode: RewriteMode,
    pub auto_capitalize: bool,
    pub auto_punctuation: bool,
    pub vocabulary_enabled: bool,
    pub llm_enabled: bool,
}
