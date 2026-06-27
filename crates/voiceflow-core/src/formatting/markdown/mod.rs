use regex::Regex;
use crate::formatting::traits::FormatterRule;
use crate::formatting::metadata::{RuleMetadata, RuleCategory, RuleCapabilities};
use crate::pipeline::models::{TransformationRequest, TransformationState};
use crate::pipeline::models::Diagnostic;
use std::time::Instant;

const METADATA: RuleMetadata = RuleMetadata {
    name: "MarkdownRule",
    version: "1.0.0",
    category: RuleCategory::Formatting,
    priority: 850,
    capabilities: RuleCapabilities {
        streaming_safe: true,
        token_based: false,
        regex_based: true,
        locale_aware: false,
        developer_only: false,
        markdown_only: true,
        incremental_safe: true,
    },
};


pub struct MarkdownRule;

impl FormatterRule for MarkdownRule {
    fn metadata(&self) -> &'static RuleMetadata { &METADATA }

    fn applies(&self, request: &TransformationRequest) -> bool { request.user_context.markdown_enabled || request.mode == crate::pipeline::request::FormattingMode::Markdown }

    fn apply(&self, state: &mut TransformationState, _request: &TransformationRequest) {
        let start_time = Instant::now();
        let original_text = state.current_text.clone();
        let mut result = state.current_text.clone();
        

        // Very basic markdown replacements for dictated formatting:
        // "heading one" -> "# "
        // "heading two" -> "## "
        // "heading three" -> "### "
        
        let mappings = vec![
            (r"(?i)(?:^|\n)\s*heading one\b\s*", "\n# "),
            (r"(?i)(?:^|\n)\s*heading two\b\s*", "\n## "),
            (r"(?i)(?:^|\n)\s*heading three\b\s*", "\n### "),
            (r"(?i)(?:^|\n)\s*heading four\b\s*", "\n#### "),
            (r"(?i)\bstart bold\b", "**"),
            (r"(?i)\bend bold\b", "**"),
            (r"(?i)\bstart italic\b", "*"),
            (r"(?i)\bend italic\b", "*"),
            (r"(?i)\bstart code block\b", "\n```\n"),
            (r"(?i)\bend code block\b", "\n```\n"),
            (r"(?i)\bbacktick\b", "`"),
        ];

        for (pattern, replacement) in mappings {
            if let Ok(re) = Regex::new(pattern) {
                result = re.replace_all(&result, replacement).to_string();
            }
        }
        
        // Clean up any leading newline from heading insertions if at start of string
        if result.starts_with('\n') {
            result = result[1..].to_string();
        }

        state.current_text = result;
        let duration = start_time.elapsed().as_millis();
        if original_text != state.current_text {
            state.diagnostics.push(Diagnostic {
                rule: METADATA.name,
                severity: "info".to_string(),
                before: original_text,
                after: state.current_text.clone(),
                duration_ms: duration,
            });
        }
    }
}
