use regex::Regex;
use crate::formatting::traits::FormatterRule;
use crate::formatting::metadata::{RuleMetadata, RuleCategory, RuleCapabilities};
use crate::pipeline::models::{TransformationRequest, TransformationState};
use crate::pipeline::models::Diagnostic;
use std::time::Instant;

const METADATA: RuleMetadata = RuleMetadata {
    name: "NumbersRule",
    version: "1.0.0",
    category: RuleCategory::Formatting,
    priority: 820,
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


pub struct NumbersRule;

impl FormatterRule for NumbersRule {
    fn metadata(&self) -> &'static RuleMetadata { &METADATA }

    fn applies(&self, _request: &TransformationRequest) -> bool { true }

    fn apply(&self, state: &mut TransformationState, _request: &TransformationRequest) {
        let start_time = Instant::now();
        let original_text = state.current_text.clone();
        let mut result = state.current_text.clone();
        

        // 1. Currency (basic English)
        // E.g. "100 dollars" -> "$100"
        let dollars_re = Regex::new(r"(?i)\b(\d+(?:\.\d+)?)\s+dollars\b").unwrap();
        result = dollars_re.replace_all(&result, "$$$1").to_string();

        let euros_re = Regex::new(r"(?i)\b(\d+(?:\.\d+)?)\s+euros\b").unwrap();
        result = euros_re.replace_all(&result, "€$1").to_string();

        let pounds_re = Regex::new(r"(?i)\b(\d+(?:\.\d+)?)\s+pounds\b").unwrap();
        result = pounds_re.replace_all(&result, "£$1").to_string();

        // 2. Units (basic English)
        let percent_re = Regex::new(r"(?i)\b(\d+(?:\.\d+)?)\s+percent\b").unwrap();
        result = percent_re.replace_all(&result, "$1%").to_string();

        let km_re = Regex::new(r"(?i)\b(\d+(?:\.\d+)?)\s+kilometers\b").unwrap();
        result = km_re.replace_all(&result, "$1 km").to_string();

        let kg_re = Regex::new(r"(?i)\b(\d+(?:\.\d+)?)\s+kilograms\b").unwrap();
        result = kg_re.replace_all(&result, "$1 kg").to_string();

        // 3. Ordinals
        // Simple regex replace for digit followed by ordinal word. This needs word to number translation in reality.
        // e.g. "first" -> "1st"
        let ordinals = vec![
            (r"(?i)\bfirst\b", "1st"),
            (r"(?i)\bsecond\b", "2nd"),
            (r"(?i)\bthird\b", "3rd"),
            (r"(?i)\bfourth\b", "4th"),
            (r"(?i)\bfifth\b", "5th"),
            (r"(?i)\bsixth\b", "6th"),
            (r"(?i)\bseventh\b", "7th"),
            (r"(?i)\beighth\b", "8th"),
            (r"(?i)\bninth\b", "9th"),
            (r"(?i)\btenth\b", "10th"),
        ];

        for (pattern, replacement) in ordinals {
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
