use regex::Regex;
use crate::formatting::metadata::{RuleMetadata, RuleCategory, RuleCapabilities};
use crate::pipeline::models::{TransformationRequest, TransformationState};
use crate::pipeline::models::Diagnostic;
use std::time::Instant;

const METADATA: RuleMetadata = RuleMetadata {
    name: "SpacingRule",
    version: "1.0.0",
    category: RuleCategory::Cleanup,
    priority: 500,
    capabilities: RuleCapabilities {
        streaming_safe: true,
        token_based: false,
        regex_based: true,
        locale_aware: false,
        developer_only: false,
        markdown_only: false,
        incremental_safe: true,
    },
};

pub struct SpacingRule;

impl crate::formatting::traits::FormatterRule for SpacingRule {
    fn metadata(&self) -> &'static RuleMetadata {
        &METADATA
    }

    fn applies(&self, _request: &TransformationRequest) -> bool {
        true
    }

    fn apply(&self, state: &mut TransformationState, _request: &TransformationRequest) {
        let start_time = Instant::now();
        let original_text = state.current_text.clone();
        
        state.current_text = fix_spacing(&state.current_text);
        
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

pub fn fix_spacing(text: &str) -> String {
    let mut result = text.to_string();
    
    // Remove space before punctuation
    let re_before = Regex::new(r"\s+([,\.\?\!\:\;])").unwrap();
    result = re_before.replace_all(&result, "$1").to_string();
    
    // Ensure single space after punctuation (except if followed by newline or another punctuation)
    // Note: Period (.) is excluded to prevent mangling URLs (e.g. openai.com) and Emails.
    let re_after = Regex::new(r"([,\?\!\:\;])([^\s\n\.\,\!\?])").unwrap();
    result = re_after.replace_all(&result, "$1 $2").to_string();
    
    result
}
