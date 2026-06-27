use regex::Regex;
use crate::formatting::metadata::{RuleMetadata, RuleCategory, RuleCapabilities};
use crate::pipeline::models::{TransformationRequest, TransformationState};
use crate::pipeline::models::Diagnostic;
use std::time::Instant;

const METADATA: RuleMetadata = RuleMetadata {
    name: "CapitalizationRule",
    version: "1.0.0",
    category: RuleCategory::Formatting,
    priority: 200,
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

pub struct CapitalizationRule;

impl crate::formatting::traits::FormatterRule for CapitalizationRule {
    fn metadata(&self) -> &'static RuleMetadata {
        &METADATA
    }

    fn applies(&self, _request: &TransformationRequest) -> bool {
        true
    }

    fn apply(&self, state: &mut TransformationState, _request: &TransformationRequest) {
        let start_time = Instant::now();
        let original_text = state.current_text.clone();
        
        state.current_text = capitalize(&state.current_text);
        
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

pub fn capitalize(text: &str) -> String {
    let mut result = text.to_string();
    
    // Capitalize first letter of the text
    if let Some(first_char) = result.chars().next() {
        if first_char.is_lowercase() {
            let mut c_iter = result.chars();
            let first = c_iter.next().unwrap().to_uppercase().to_string();
            result = first + c_iter.as_str();
        }
    }
    
    // Capitalize after period, question mark, or exclamation mark followed by a space
    let re_sentence = Regex::new(r"([\.!\?]\s+)([a-z])").unwrap();
    result = re_sentence.replace_all(&result, |caps: &regex::Captures| {
        format!("{}{}", &caps[1], caps[2].to_uppercase())
    }).to_string();
    
    // Capitalize after newlines
    let re_newline = Regex::new(r"(\n+)([a-z])").unwrap();
    result = re_newline.replace_all(&result, |caps: &regex::Captures| {
        format!("{}{}", &caps[1], caps[2].to_uppercase())
    }).to_string();

    result
}
