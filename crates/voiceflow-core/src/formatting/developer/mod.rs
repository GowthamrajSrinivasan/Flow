use regex::Regex;
use crate::formatting::traits::FormatterRule;
use crate::formatting::metadata::{RuleMetadata, RuleId, RuleCategory, RuleCapabilities};
use crate::pipeline::models::{TransformationRequest, TransformationState};
use crate::pipeline::models::Diagnostic;
use std::time::Instant;

const METADATA: RuleMetadata = RuleMetadata {
    id: RuleId("DeveloperRule"),
    name: "DeveloperRule",
    version: "1.0.0",
    category: RuleCategory::Developer,
    priority: 870,
    capabilities: RuleCapabilities {
        streaming_safe: true,
        token_based: false,
        regex_based: true,
        locale_aware: false,
        developer_only: true,
        markdown_only: false,
        incremental_safe: true,
    },
    depends_on: &[],
};


pub struct DeveloperRule;

impl FormatterRule for DeveloperRule {
    fn metadata(&self) -> &'static RuleMetadata { &METADATA }

    fn applies(&self, request: &TransformationRequest) -> bool { request.mode == crate::pipeline::request::FormattingMode::Developer || request.user_context.language == "rust" || request.user_context.language == "python" }

    fn apply(&self, state: &mut TransformationState, _request: &TransformationRequest) {
        let start_time = Instant::now();
        let original_text = state.current_text.clone();
        let mut result = state.current_text.clone();
        

        // 1. Casing logic (snake_case, camelCase, PascalCase)
        // E.g. "snake case my variable name" -> "my_variable_name"
        let snake_case_re = Regex::new(r"(?i)snake\s+case\s+([a-zA-Z0-9\s]+?)(?:(?:\s+end\s+casing)|\b$)").unwrap();
        result = snake_case_re.replace_all(&result, |caps: &regex::Captures| {
            let content = caps.get(1).unwrap().as_str().trim();
            content.split_whitespace().map(|s| s.to_lowercase()).collect::<Vec<String>>().join("_")
        }).to_string();

        let camel_case_re = Regex::new(r"(?i)camel\s+case\s+([a-zA-Z0-9\s]+?)(?:(?:\s+end\s+casing)|\b$)").unwrap();
        result = camel_case_re.replace_all(&result, |caps: &regex::Captures| {
            let content = caps.get(1).unwrap().as_str().trim();
            let mut words = content.split_whitespace();
            if let Some(first) = words.next() {
                let mut out = first.to_lowercase();
                for word in words {
                    if let Some(c) = word.chars().next() {
                        out.push_str(&c.to_uppercase().to_string());
                        out.push_str(&word[c.len_utf8()..].to_lowercase());
                    }
                }
                out
            } else {
                String::new()
            }
        }).to_string();
        
        // 2. Simple aliases
        let aliases = vec![
            (r"(?i)\bdouble equal(?:s)?\b", "=="),
            (r"(?i)\btriple equal(?:s)?\b", "==="),
            (r"(?i)\bnot equal(?:s)?\b", "!="),
            (r"(?i)\bgreater than or equal(?:s)?\b", ">="),
            (r"(?i)\bless than or equal(?:s)?\b", "<="),
            (r"(?i)\bgreater than\b", ">"),
            (r"(?i)\bless than\b", "<"),
            (r"(?i)\bforward slash\b", "/"),
            (r"(?i)\bbackslash\b", "\\"),
        ];

        for (pattern, replacement) in aliases {
            if let Ok(re) = Regex::new(pattern) {
                result = re.replace_all(&result, replacement).to_string();
            }
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
