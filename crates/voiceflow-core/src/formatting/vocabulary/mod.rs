use regex::Regex;
use voiceflow_shared::config::vocabulary::VocabularyEntry;
use crate::formatting::traits::FormatterRule;
use crate::formatting::metadata::{RuleMetadata, RuleCategory, RuleCapabilities};
use crate::pipeline::models::{TransformationRequest, TransformationState, Diagnostic};
use std::time::Instant;

const METADATA: RuleMetadata = RuleMetadata {
    name: "Vocabulary",
    version: "1.0.0",
    category: RuleCategory::Normalization,
    priority: 800,
    capabilities: RuleCapabilities {
        streaming_safe: true,
        token_based: false,
        regex_based: true,
        locale_aware: true,
        developer_only: false,
        markdown_only: false,
        incremental_safe: true,
    },
};

pub struct VocabularyRule;

impl FormatterRule for VocabularyRule {
    fn metadata(&self) -> &'static RuleMetadata {
        &METADATA
    }

    fn applies(&self, _request: &TransformationRequest) -> bool {
        true
    }

    fn apply(&self, state: &mut TransformationState, request: &TransformationRequest) {
        let start_time = Instant::now();
        let original_text = state.current_text.clone();
        
        state.current_text = expand_vocabulary(&state.current_text, request);
        
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

pub fn expand_vocabulary(text: &str, request: &TransformationRequest) -> String {
    let mut result = text.to_string();
    
    let mut entries = Vec::new();

    // 1. Load User Vocabulary if available
    if let Some(vocab) = &request.user_context.vocabulary {
        for entry in &vocab.entries {
            if entry.enabled {
                entries.push(entry.clone());
            }
        }
    }
    
    // 2. Fallback / default built-in vocabulary
    if entries.is_empty() {
        entries.push(VocabularyEntry {
            spoken: "cmg".to_string(),
            output: "CMG".to_string(),
            language: "en".to_string(),
            enabled: true,
            case_sensitive: false,
        });
        entries.push(VocabularyEntry {
            spoken: "petro signs".to_string(),
            output: "PetroSigns".to_string(),
            language: "en".to_string(),
            enabled: true,
            case_sensitive: false,
        });
        entries.push(VocabularyEntry {
            spoken: "qwen".to_string(),
            output: "Qwen3".to_string(),
            language: "en".to_string(),
            enabled: true,
            case_sensitive: false,
        });
    }

    // 3. Normalize spoken phrases and count words for sorting
    let mut processed_entries: Vec<(String, String, bool, usize)> = entries.into_iter().map(|entry| {
        let normalized = entry.spoken.trim().to_lowercase();
        // Collapse whitespace
        let re_ws = Regex::new(r"\s+").unwrap();
        let normalized = re_ws.replace_all(&normalized, " ").to_string();
        let word_count = normalized.split_whitespace().count();
        (normalized, entry.output, entry.case_sensitive, word_count)
    }).collect();

    // 4. Sort by word count descending (longest phrase first)
    processed_entries.sort_by(|a, b| b.3.cmp(&a.3));

    // 5. Apply regex replacements
    for (spoken, output, case_sensitive, _) in processed_entries {
        let escaped_spoken = regex::escape(&spoken);
        // Replace spaces with \s+ to handle varying whitespace in the input text
        let flexible_spaces = escaped_spoken.replace(" ", r"\s+");
        let pattern = if case_sensitive {
            format!(r"\b{}\b", flexible_spaces)
        } else {
            format!(r"(?i)\b{}\b", flexible_spaces)
        };
        
        if let Ok(re) = Regex::new(&pattern) {
            result = re.replace_all(&result, output.as_str()).to_string();
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use voiceflow_shared::config::vocabulary::UserVocabulary;
    use crate::pipeline::models::TransformationRequest;

    #[test]
    fn test_expand_vocabulary_longest_phrase_first() {
        let vocab = UserVocabulary {
            entries: vec![
                VocabularyEntry {
                    spoken: "computer".to_string(),
                    output: "PC".to_string(),
                    language: "en".to_string(),
                    enabled: true,
                    case_sensitive: false,
                },
                VocabularyEntry {
                    spoken: "computer modeling group".to_string(),
                    output: "CMG".to_string(),
                    language: "en".to_string(),
                    enabled: true,
                    case_sensitive: false,
                },
            ],
        };
        let mut request = TransformationRequest::new("We work at the computer modeling group today".to_string());
        request.user_context.vocabulary = Some(vocab);
        let result = expand_vocabulary("We work at the computer modeling group today", &request);
        assert_eq!(result, "We work at the CMG today");
    }

    #[test]
    fn test_expand_vocabulary_case_sensitivity() {
        let vocab = UserVocabulary {
            entries: vec![
                VocabularyEntry {
                    spoken: "apple".to_string(),
                    output: "AppleInc".to_string(),
                    language: "en".to_string(),
                    enabled: true,
                    case_sensitive: true,
                },
                VocabularyEntry {
                    spoken: "orange".to_string(),
                    output: "OrangeCorp".to_string(),
                    language: "en".to_string(),
                    enabled: true,
                    case_sensitive: false,
                },
            ],
        };
        let mut request = TransformationRequest::new("".to_string());
        request.user_context.vocabulary = Some(vocab);
        let result1 = expand_vocabulary("I have an apple and an Apple", &request);
        // "apple" is case-sensitive, so only exact matches should be replaced. Wait, let's see how our regex matching is done.
        // It does let pattern = format!(r"\b{}\b", flexible_spaces) which matches lowercase "apple", but not "Apple" (since it's capital A).
        // Let's verify:
        assert_eq!(result1, "I have an AppleInc and an Apple");

        let result2 = expand_vocabulary("I have an Orange and an orange", &request);
        // "orange" is not case sensitive, so both should be replaced:
        assert_eq!(result2, "I have an OrangeCorp and an OrangeCorp");
    }

    #[test]
    fn test_expand_vocabulary_whitespace_collapsing() {
        let vocab = UserVocabulary {
            entries: vec![
                VocabularyEntry {
                    spoken: "petro  signs".to_string(), // multiple spaces
                    output: "PetroSigns".to_string(),
                    language: "en".to_string(),
                    enabled: true,
                    case_sensitive: false,
                },
            ],
        };
        let mut request = TransformationRequest::new("Let's look at petro   signs".to_string());
        request.user_context.vocabulary = Some(vocab);
        let result = expand_vocabulary("Let's look at petro   signs", &request);
        assert_eq!(result, "Let's look at PetroSigns");
    }
}
