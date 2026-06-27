use regex::Regex;
use crate::formatting::metadata::{RuleMetadata, RuleCategory, RuleCapabilities};
use crate::pipeline::models::{TransformationRequest, TransformationState};
use crate::pipeline::models::Diagnostic;
use std::time::Instant;

const METADATA: RuleMetadata = RuleMetadata {
    name: "ListsRule",
    version: "1.0.0",
    category: RuleCategory::Formatting,
    priority: 600,
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

pub struct ListsRule;

impl crate::formatting::traits::FormatterRule for ListsRule {
    fn metadata(&self) -> &'static RuleMetadata {
        &METADATA
    }

    fn applies(&self, _request: &TransformationRequest) -> bool {
        true
    }

    fn apply(&self, state: &mut TransformationState, _request: &TransformationRequest) {
        let start_time = Instant::now();
        let original_text = state.current_text.clone();
        
        state.current_text = format(&state.current_text);
        
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

pub fn format(text: &str) -> String {
    let command_re = Regex::new(r"(?i)(^|\s+|[.?!,]\s*)(?:(?:please|can\s+you|could\s+you)\s+)?convert\s+to\s+(?:a\s+)?(?:bullet|bulleted)\s+list(\s*[.?!]+(?:\s+|$)|$)").unwrap();
    let mut resolved = text.to_string();
    let mut changed = true;

    while changed {
        changed = false;
        if let Some(caps) = command_re.captures(&resolved) {
            let command_start = caps.get(0).unwrap().start();
            let command_end = caps.get(0).unwrap().end();
            let prefix = caps.get(1).unwrap().as_str();
            
            let before_command = &resolved[..command_start + prefix.len()];
            
            // Trim trailing punctuation and whitespace to find true previous sentences
            let trimmed = before_command.trim_end_matches(|c: char| c.is_whitespace() || c == '.' || c == '?' || c == '!');
            let sentence_terminator = Regex::new(r"[.?!]+").unwrap();
            let matches: Vec<_> = sentence_terminator.find_iter(trimmed).collect();
            
            let mut cut_index = 0;
            if let Some(last_match) = matches.last() {
                cut_index = last_match.end();
                while cut_index < trimmed.len() && trimmed[cut_index..].starts_with(|c: char| c.is_whitespace()) {
                    cut_index += trimmed[cut_index..].chars().next().unwrap().len_utf8();
                }
            }
            
            let prefix_text = before_command[..cut_index].to_string();
            let target_sentence = before_command[cut_index..].trim_end();
            
            if target_sentence.contains(',') || target_sentence.contains(" and ") {
                let mut list_prefix = String::new();
                let mut list_content = target_sentence;
                
                if let Some(colon_idx) = target_sentence.find(':') {
                    list_prefix = target_sentence[..colon_idx + 1].to_string();
                    list_content = &target_sentence[colon_idx + 1..];
                }

                let split_re = Regex::new(r"(?i)(?:,\s*and\s+|,\s*|\s+and\s+)").unwrap();
                let parts: Vec<&str> = split_re.split(list_content).filter(|s| !s.trim().is_empty()).collect();
                
                let mut bulleted = String::new();
                for (i, part) in parts.iter().enumerate() {
                    bulleted.push_str(&format!("- {}", part.trim()));
                    if i < parts.len() - 1 {
                        bulleted.push('\n');
                    }
                }
                
                let mut new_resolved = prefix_text;
                if !list_prefix.is_empty() {
                    new_resolved.push_str(&list_prefix);
                    new_resolved.push('\n');
                } else {
                    // For a normal comma list (e.g. "I bought apples, bananas, and oranges.")
                    // "I bought apples" is the first bullet, but maybe the user wanted a prefix.
                    // For MVP, we just accept "I bought apples" as a bullet.
                }
                new_resolved.push_str(&bulleted);
                new_resolved.push_str(&resolved[command_end..]);
                resolved = new_resolved;
                changed = true;
            } else {
                let mut new_resolved = prefix_text;
                new_resolved.push_str(&format!("- {}", target_sentence));
                new_resolved.push_str(&resolved[command_end..]);
                resolved = new_resolved;
                changed = true;
            }
        }
    }
    
    resolved.trim_start().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comma_separated_list() {
        let text = "I need apples, bananas, and oranges. Convert to bullet list.";
        let expected = "- I need apples\n- bananas\n- oranges.";
        assert_eq!(format(text), expected);
    }

    #[test]
    fn test_colon_list() {
        let text = "Here is the plan: design, code, and test. Convert to a bullet list.";
        let expected = "Here is the plan:\n- design\n- code\n- test.";
        assert_eq!(format(text), expected);
    }
}
