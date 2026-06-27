#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleId(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleCategory {
    Lexical,
    Normalization,
    Developer,
    Formatting,
    Cleanup,
    Validation,
}

#[derive(Debug, Clone)]
pub struct RuleCapabilities {
    pub streaming_safe: bool,
    pub token_based: bool,
    pub regex_based: bool,
    pub locale_aware: bool,
    pub developer_only: bool,
    pub markdown_only: bool,
    pub incremental_safe: bool,
}

impl Default for RuleCapabilities {
    fn default() -> Self {
        Self {
            streaming_safe: true,
            token_based: false,
            regex_based: true,
            locale_aware: false,
            developer_only: false,
            markdown_only: false,
            incremental_safe: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuleMetadata {
    pub id: RuleId,
    pub name: &'static str,
    pub version: &'static str,
    pub category: RuleCategory,
    pub priority: u16,
    pub capabilities: RuleCapabilities,
    pub depends_on: &'static [RuleId],
}
