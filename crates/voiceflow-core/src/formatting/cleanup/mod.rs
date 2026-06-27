use regex::Regex;
use crate::formatting::metadata::{RuleMetadata, RuleId, RuleCategory, RuleCapabilities};
use crate::pipeline::models::{TransformationRequest, TransformationState};
use crate::pipeline::models::Diagnostic;
use std::time::Instant;

const METADATA: RuleMetadata = RuleMetadata {
    id: RuleId("CleanupRule"),
    name: "CleanupRule",
    version: "1.0.0",
    category: RuleCategory::Cleanup,
    priority: 100,
    capabilities: RuleCapabilities {
        streaming_safe: true,
        token_based: false,
        regex_based: true,
        locale_aware: false,
        developer_only: false,
        markdown_only: false,
        incremental_safe: true,
    },
    depends_on: &[],
};

pub struct CleanupRule;

impl crate::formatting::traits::FormatterRule for CleanupRule {
    fn metadata(&self) -> &'static RuleMetadata {
        &METADATA
    }

    fn applies(&self, _request: &TransformationRequest) -> bool {
        true
    }

    fn apply(&self, state: &mut TransformationState, _request: &TransformationRequest) {
        let start_time = Instant::now();
        let original_text = state.current_text.clone();
        
        state.current_text = cleanup(&state.current_text);
        
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

pub fn cleanup(text: &str) -> String {
    let mut result = text.to_string();
    
    // Remove duplicate spaces
    let re_spaces = Regex::new(r" {2,}").unwrap();
    result = re_spaces.replace_all(&result, " ").to_string();
    
    // Remove duplicate newlines (limit to 2 max)
    let re_newlines = Regex::new(r"\n{3,}").unwrap();
    result = re_newlines.replace_all(&result, "\n\n").to_string();
    
    // Remove duplicate punctuation
    let mut new_result = String::with_capacity(result.len());
    let mut last_char = '\0';
    for c in result.chars() {
        if ".,?!:;".contains(c) && c == last_char {
            continue;
        }
        new_result.push(c);
        last_char = c;
    }
    result = new_result;
    
    result.trim().to_string()
}
