use regex::Regex;
use crate::formatting::metadata::{RuleMetadata, RuleId, RuleCategory, RuleCapabilities};
use crate::pipeline::models::{TransformationRequest, TransformationState};
use crate::pipeline::models::Diagnostic;
use std::time::Instant;

const METADATA: RuleMetadata = RuleMetadata {
    id: RuleId("PunctuationRule"),
    name: "PunctuationRule",
    version: "1.0.0",
    category: RuleCategory::Lexical,
    priority: 1000,
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

pub struct PunctuationRule;

impl crate::formatting::traits::FormatterRule for PunctuationRule {
    fn metadata(&self) -> &'static RuleMetadata {
        &METADATA
    }

    fn applies(&self, _request: &TransformationRequest) -> bool {
        true
    }

    fn apply(&self, state: &mut TransformationState, _request: &TransformationRequest) {
        let start_time = Instant::now();
        let original_text = state.current_text.clone();
        
        state.current_text = convert(&state.current_text);
        
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

pub fn convert(text: &str) -> String {
    let mut result = text.to_string();
    
    let mappings = [
        ("comma", ","),
        ("period", "."),
        ("full stop", "."),
        ("question mark", "?"),
        ("exclamation mark", "!"),
        ("colon", ":"),
        ("semicolon", ";"),
        ("dash", "-"),
        ("hyphen", "-"),
        ("new line", "\n"),
        ("new paragraph", "\n\n"),
        ("open quote", "\""),
        ("close quote", "\""),
        ("open bracket", "("),
        ("close bracket", ")"),
    ];
    
    // Note: In a robust implementation, this would use word boundaries
    // to avoid matching "command" as "comma" + "nd".
    for (word, punc) in mappings.iter() {
        // Simple case-insensitive replacement with word boundaries
        let pattern = format!(r"(?i)\b{}\b", word);
        if let Ok(re) = Regex::new(&pattern) {
            result = re.replace_all(&result, *punc).to_string();
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spoken_punctuation() {
        assert_eq!(convert("hello comma world period"), "hello , world .");
        assert_eq!(convert("this is a test new paragraph wait new line yes"), "this is a test \n\n wait \n yes");
        assert_eq!(convert("what is this question mark"), "what is this ?");
        assert_eq!(convert("stop exclamation mark"), "stop !");
    }

    #[test]
    fn test_case_insensitivity() {
        assert_eq!(convert("HELLO COMMA WORLD"), "HELLO , WORLD");
        assert_eq!(convert("Next New Paragraph Okay"), "Next \n\n Okay");
    }

    #[test]
    fn test_word_boundaries() {
        assert_eq!(convert("commander"), "commander");
        assert_eq!(convert("new liner"), "new liner");
    }
}
