use regex::Regex;
use super::context::*;

pub fn resolve_all_tier1(text: &str) -> String {
    resolve_all_tier1_with_context(text, &mut RewriteContext::new())
}

pub fn resolve_all_tier1_with_context(text: &str, context: &mut RewriteContext) -> String {
    let mut resolved = text.to_string();
    
    resolved = resolve_false_starts_with_context(&resolved, context);
    resolved = resolve_inline_corrections_with_context(&resolved, context);
    resolved = resolve_undo_commands_with_context(&resolved, context);
    resolved = resolve_basic_replace_with_context(&resolved, context);
    resolved = resolve_replace_last_word_with_context(&resolved, context);
    resolved = resolve_delete_last_sentence_with_context(&resolved, context);
    resolved = resolve_delete_last_word_with_context(&resolved, context);
    
    resolved
}

fn classify_entity(text: &str) -> EntityType {
    if Regex::new(r"(?i)^\d+(?:\:\d+)?\s*(?:AM|PM|am|pm)$").unwrap().is_match(text) {
        return EntityType::Time;
    }
    if Regex::new(r"(?i)^(Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday)$").unwrap().is_match(text) {
        return EntityType::Date;
    }
    if Regex::new(r"(?i)^[A-Za-z]+\s+\d+(?:st|nd|rd|th)?$").unwrap().is_match(text) {
        return EntityType::Date;
    }
    if Regex::new(r"(?i)^\d+$").unwrap().is_match(text) {
        return EntityType::Number;
    }
    if Regex::new(r"(?i)^\d+\s+[a-z]+$").unwrap().is_match(text) {
        return EntityType::Number;
    }
    if Regex::new(r"^[A-Z][a-z]+$").unwrap().is_match(text) {
        return EntityType::Location; // Location/Person are treated similarly for now
    }
    EntityType::Unknown
}

fn get_confidence(old_str: &str, new_str: &str) -> CorrectionConfidence {
    let old_type = classify_entity(old_str);
    let new_type = classify_entity(new_str);
    
    if old_type != EntityType::Unknown && old_type == new_type {
        return CorrectionConfidence::High;
    }
    
    if old_type == EntityType::Unknown && new_type == EntityType::Unknown {
        let old_capitalized = old_str.chars().next().map_or(false, |c| c.is_uppercase());
        let new_capitalized = new_str.chars().next().map_or(false, |c| c.is_uppercase());
        if old_capitalized && new_capitalized {
            return CorrectionConfidence::Medium;
        }
        return CorrectionConfidence::Low;
    }
    
    CorrectionConfidence::Low
}

pub fn resolve_inline_corrections(text: &str) -> String {
    resolve_inline_corrections_with_context(text, &mut RewriteContext::new())
}

pub fn resolve_inline_corrections_with_context(text: &str, context: &mut RewriteContext) -> String {
    let triggers = r"(?:no wait|wait|no|actually|sorry|correction|i mean|rather)";
    
    // Check if the current text ends with a trigger (streaming safety)
    let pending_trigger_re = Regex::new(&format!(r"(?i)\b{}\s*$", triggers)).unwrap();
    if pending_trigger_re.is_match(text) {
        if context.pending_correction.is_none() {
            context.pending_correction = Some(PendingCorrection {
                trigger: "pending".to_string(),
                target_span: (0, 0),
            });
        }
        return text.to_string(); // Do not process yet
    } else if context.pending_correction.is_some() {
        context.pending_correction = None; // Reset once complete
    }

    let full_trigger_pattern = format!(r"\s*[\.,]?\s*{}\s*[\.,]?\s+", triggers);

    // To implement the confidence checks, we iterate manually
    let patterns = vec![
        (Regex::new(&format!(r"(?i)\b(?P<old>\d+(?:\:\d+)?\s*(?:AM|PM|am|pm))\b{}\b(?P<new>\d+(?:\:\d+)?\s*(?:AM|PM|am|pm))\b", full_trigger_pattern)).unwrap(), "$new"),
        (Regex::new(&format!(r"(?i)\b(?P<old>\d+\s+[a-z]+)\b{}\b(?P<new>\d+\s+[a-z]+)\b", full_trigger_pattern)).unwrap(), "$new"),
        (Regex::new(&format!(r"(?i)\b(?P<old>[A-Za-z]+\s+\d+(?:st|nd|rd|th)?)\b{}\b(?P<new>[A-Za-z]+\s+\d+(?:st|nd|rd|th)?)\b", full_trigger_pattern)).unwrap(), "$new"),
        (Regex::new(&format!(r"(?i)\b(?P<old>Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday)\b{}\b(?P<new>Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday)\b", full_trigger_pattern)).unwrap(), "$new"),
        (Regex::new(&format!(r"(?i)\b(?P<old>[A-Z][a-z]+)\b{}\b(?P<new>[A-Z][a-z]+)\b", full_trigger_pattern)).unwrap(), "$new"),
        (Regex::new(&format!(r"(?i)\b(?P<old>\w+)\b{}\b(?P<new>\w+)\b", full_trigger_pattern)).unwrap(), "$new"),
    ];

    let mut resolved = text.to_string();
    let mut changed = true;
    while changed {
        changed = false;
        for (pattern, _) in &patterns {
            if let Some(caps) = pattern.captures(&resolved) {
                let old_str = caps.name("old").map(|m| m.as_str()).unwrap_or("");
                let new_str = caps.name("new").map(|m| m.as_str()).unwrap_or("");
                
                let confidence = get_confidence(old_str, new_str);
                if confidence != CorrectionConfidence::Low {
                    let before = resolved.clone();
                    
                    let mut new_resolved = String::new();
                    let match_start = caps.get(0).unwrap().start();
                    let match_end = caps.get(0).unwrap().end();
                    new_resolved.push_str(&resolved[..match_start]);
                    new_resolved.push_str(new_str);
                    new_resolved.push_str(&resolved[match_end..]);
                    
                    resolved = new_resolved;
                    changed = true;
                    
                    context.history.push(RewriteOperation::InlineCorrection {
                        before,
                        after: resolved.clone(),
                    });
                    break;
                }
            }
        }
    }
    
    // Complex patterns
    let pattern_complex_1 = Regex::new(&format!(r"(?i)\b(?P<month>[A-Za-z]+)\s+(?P<old>\d+(?:st|nd|rd|th)?)\b{}\b(?P<new>\d+(?:st|nd|rd|th)?)\b", full_trigger_pattern)).unwrap();
    if let Some(caps) = pattern_complex_1.captures(&resolved) {
        let before = resolved.clone();
        let month = caps.name("month").unwrap().as_str();
        let new_str = caps.name("new").unwrap().as_str();
        let replacement = format!("{} {}", month, new_str);
        
        let mut new_resolved = String::new();
        new_resolved.push_str(&resolved[..caps.get(0).unwrap().start()]);
        new_resolved.push_str(&replacement);
        new_resolved.push_str(&resolved[caps.get(0).unwrap().end()..]);
        resolved = new_resolved;
        context.history.push(RewriteOperation::InlineCorrection { before, after: resolved.clone() });
    }
    
    let pattern_complex_2 = Regex::new(&format!(r"(?i)\b(?P<old>\d+)(?P<noun>\s+[a-z]+){}\b(?P<new>\d+)\b", full_trigger_pattern)).unwrap();
    if let Some(caps) = pattern_complex_2.captures(&resolved) {
        let before = resolved.clone();
        let noun = caps.name("noun").unwrap().as_str();
        let new_str = caps.name("new").unwrap().as_str();
        let replacement = format!("{}{}", new_str, noun);
        
        let mut new_resolved = String::new();
        new_resolved.push_str(&resolved[..caps.get(0).unwrap().start()]);
        new_resolved.push_str(&replacement);
        new_resolved.push_str(&resolved[caps.get(0).unwrap().end()..]);
        resolved = new_resolved;
        context.history.push(RewriteOperation::InlineCorrection { before, after: resolved.clone() });
    }

    resolved
}

pub fn resolve_false_starts(text: &str) -> String {
    resolve_false_starts_with_context(text, &mut RewriteContext::new())
}

pub fn resolve_false_starts_with_context(text: &str, context: &mut RewriteContext) -> String {
    let word_regex = Regex::new(r"(\w+)").unwrap();
    let mut words = Vec::new();
    let mut matches = Vec::new();
    
    for m in word_regex.find_iter(text) {
        words.push(m.as_str().to_lowercase());
        matches.push((m.start(), m.end()));
    }

    if words.is_empty() {
        return text.to_string();
    }

    let mut deleted_ranges: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while i < words.len() {
        let mut best_match = None;
        let mut best_score = 0.0;

        let max_n = words.len() - i;
        for n in (1..=max_n).rev() {
            let mut j = i + n;
            let max_j = std::cmp::min(words.len() - n, i + n + 10); 
            
            while j <= max_j {
                let mut is_match = true;
                for k in 0..n {
                    if words[i + k] != words[j + k] {
                        is_match = false;
                        break;
                    }
                }
                
                if is_match {
                    let d = j - i;
                    let score = n as f32 / d as f32;
                    
                    if score >= 0.5 && (n >= 2 || (n == 1 && d == 1)) { 
                        if score > best_score || (score == best_score && n > best_match.unwrap_or((0,0,0)).2) {
                            best_match = Some((i, j, n));
                            best_score = score;
                        }
                    }
                }
                j += 1;
            }
        }

        if let Some((best_i, best_j, _)) = best_match {
            deleted_ranges.push((best_i, best_j));
            i = best_j; 
        } else {
            i += 1;
        }
    }

    if deleted_ranges.is_empty() {
        return text.to_string();
    }

    let mut result = String::new();
    let mut last_end = 0;
    
    for (start_word, end_word) in deleted_ranges {
        let start_char = matches[start_word].0;
        let end_char = matches[end_word].0;
        
        result.push_str(&text[last_end..start_char]);
        last_end = end_char;
    }
    
    result.push_str(&text[last_end..]);

    let mut chars: Vec<char> = result.trim_start().chars().collect();
    if let Some(first) = chars.first_mut() {
        *first = first.to_ascii_uppercase();
    }
    
    let final_result = chars.into_iter().collect::<String>().trim_end().to_string();
    
    if final_result != text {
        context.history.push(RewriteOperation::FalseStartRecovery {
            before: text.to_string(),
            after: final_result.clone(),
        });
    }

    final_result
}

pub fn resolve_undo_commands(text: &str) -> String {
    resolve_undo_commands_with_context(text, &mut RewriteContext::new())
}

pub fn resolve_undo_commands_with_context(text: &str, context: &mut RewriteContext) -> String {
    let mut resolved = text.to_string();
    let command_re = Regex::new(r"(?i)(^|\s+|[.?!,]\s*)(?:(?:please|can\s+you|could\s+you)\s+)?(cancel that|never mind|ignore that|scratch that)(?:\s*[.?!]+(?:\s+|$)|$)").unwrap();

    let mut changed = true;
    while changed {
        changed = false;
        
        if let Some(caps) = command_re.captures(&resolved) {
            let before_state = resolved.clone();
            
            let full_match = caps.get(0).unwrap();
            let prefix_boundary = caps.get(1).unwrap();
            let command_start = prefix_boundary.end();
            let full_match_end = full_match.end();
            
            let mut search_end = command_start;
            while search_end > 0 && resolved[..search_end].ends_with(char::is_whitespace) {
                search_end -= 1;
            }
            
            let mut prev_sentence_start = 0;
            if search_end > 0 {
                let text_before = &resolved[..search_end];
                let sentence_terminator_re = Regex::new(r"[.?!]+\s+").unwrap();
                let matches: Vec<_> = sentence_terminator_re.find_iter(text_before).collect();
                if let Some(last_match) = matches.last() {
                    prev_sentence_start = last_match.end();
                }
            }
            
            let mut new_resolved = String::new();
            new_resolved.push_str(&resolved[..prev_sentence_start]);
            new_resolved.push_str(&resolved[full_match_end..]);
            
            if new_resolved != resolved {
                resolved = new_resolved;
                changed = true;
                context.history.push(RewriteOperation::UndoCommand {
                    before: before_state,
                    after: resolved.clone(),
                });
            }
        }
    }
    
    resolved.trim().to_string()
}

pub fn resolve_basic_replace(text: &str) -> String {
    resolve_basic_replace_with_context(text, &mut RewriteContext::new())
}

pub fn resolve_basic_replace_with_context(text: &str, context: &mut RewriteContext) -> String {
    let command_re = Regex::new(r"(?i)(^|[.?!,]\s*)(?:(?:please|can\s+you|could\s+you)\s+)?replace\s+([^.?!]+)\s+(?:with|to)\s+([^.?!]+)(\s*[.?!]+(?:\s+|$)|$)").unwrap();
    let mut resolved = text.to_string();
    
    let mut changed = true;
    while changed {
        changed = false;
        if let Some(caps) = command_re.captures(&resolved) {
            let before_state = resolved.clone();
            let command_start = caps.get(0).unwrap().start();
            let command_end = caps.get(0).unwrap().end();
            let prefix = caps.get(1).unwrap().as_str(); 
            let old_str = caps.get(2).unwrap().as_str();
            let new_str = caps.get(3).unwrap().as_str();
            let suffix = caps.get(4).unwrap().as_str();
            
            let before_command = &resolved[..command_start + prefix.len()];
            
            let escaped_old = regex::escape(old_str);
            let search_re = Regex::new(&format!(r"(?i)\b{}\b", escaped_old)).unwrap();
            let matches: Vec<_> = search_re.find_iter(before_command).collect();
            
            if let Some(last_match) = matches.last() {
                let mut new_resolved = String::new();
                new_resolved.push_str(&before_command[..last_match.start()]);
                
                // Match case of the replaced word
                let mut final_new = new_str.to_string();
                if let Some(first_char) = last_match.as_str().chars().next() {
                    if first_char.is_uppercase() {
                        let mut c = final_new.chars();
                        if let Some(f) = c.next() {
                            final_new = f.to_uppercase().collect::<String>() + c.as_str();
                        }
                    }
                }
                
                new_resolved.push_str(&final_new);
                let p = before_command[last_match.end()..].trim_end();
                let s = suffix.trim_start();
                if !p.is_empty() {
                    new_resolved.push_str(p);
                } else {
                    new_resolved.push_str(s);
                }
                new_resolved.push_str(&resolved[command_end..]);
                
                resolved = new_resolved;
                changed = true;
                context.history.push(RewriteOperation::ReplaceCommand {
                    before: before_state,
                    after: resolved.clone(),
                });
            } else {
                break;
            }
        }
    }
    
    resolved.trim().to_string()
}

pub fn resolve_delete_last_word(text: &str) -> String {
    resolve_delete_last_word_with_context(text, &mut RewriteContext::new())
}

pub fn resolve_delete_last_word_with_context(text: &str, context: &mut RewriteContext) -> String {
    let command_re = Regex::new(r"(?i)(^|\s+|[.?!,]\s*)(?:(?:please|can\s+you|could\s+you)\s+)?(?:delete|remove)\s+(?:the\s+)?(?:last|previous)\s+word(\s*[.?!]+(?:\s+|$)|$)").unwrap();
    let word_re = Regex::new(r"\w+").unwrap();
    
    let mut resolved = text.to_string();
    let mut changed = true;
    while changed {
        changed = false;
        
        if let Some(caps) = command_re.captures(&resolved) {
            let before_state = resolved.clone();
            let command_start = caps.get(0).unwrap().start();
            let command_end = caps.get(0).unwrap().end();
            let prefix = caps.get(1).unwrap().as_str();
            let suffix = caps.get(2).unwrap().as_str();
            
            let before_command = &resolved[..command_start + prefix.len()];
            
            let matches: Vec<_> = word_re.find_iter(before_command).collect();
            if let Some(last_match) = matches.last() {
                let mut new_resolved = String::new();
                let mut cut_index = last_match.start();
                
                if let Some(space_start) = before_command[..cut_index].rfind(|c: char| !c.is_whitespace()) {
                    let ch = before_command[..cut_index][space_start..].chars().next().unwrap();
                    cut_index = space_start + ch.len_utf8();
                } else {
                    cut_index = 0;
                }
                
                new_resolved.push_str(&before_command[..cut_index]);
                let p = before_command[last_match.end()..].trim_end();
                let trailing_spaces = suffix.trim_start_matches(|c: char| !c.is_whitespace());
                let s_punct = suffix.trim();
                
                if !p.is_empty() {
                    new_resolved.push_str(p);
                } else {
                    new_resolved.push_str(s_punct);
                }
                new_resolved.push_str(trailing_spaces);
                new_resolved.push_str(&resolved[command_end..]);
                
                resolved = new_resolved;
                changed = true;
                context.history.push(RewriteOperation::DeleteCommand {
                    before: before_state,
                    after: resolved.clone(),
                });
            } else {
                let mut new_resolved = String::new();
                new_resolved.push_str(before_command);
                new_resolved.push_str(suffix.trim_start());
                new_resolved.push_str(&resolved[command_end..]);
                resolved = new_resolved;
                changed = true;
            }
        }
    }
    resolved.trim().to_string()
}

pub fn resolve_delete_last_sentence(text: &str) -> String {
    resolve_delete_last_sentence_with_context(text, &mut RewriteContext::new())
}

pub fn resolve_delete_last_sentence_with_context(text: &str, context: &mut RewriteContext) -> String {
    let command_re = Regex::new(r"(?i)(^|\s+|[.?!,]\s*)(?:(?:please|can\s+you|could\s+you)\s+)?(?:delete|remove)\s+(?:the\s+)?(?:last|previous)\s+sentence(\s*[.?!]+(?:\s+|$)|$)").unwrap();
    let mut resolved = text.to_string();
    let mut changed = true;
    
    while changed {
        changed = false;
        if let Some(caps) = command_re.captures(&resolved) {
            let before_state = resolved.clone();
            let full_match_start = caps.get(0).unwrap().start();
            let command_end = caps.get(0).unwrap().end();
            let prefix = caps.get(1).unwrap().as_str();
            let _suffix = caps.get(2).unwrap().as_str();
            
            // The actual command starts after the prefix
            let command_start = full_match_start + prefix.len();
            let before_command = &resolved[..command_start];
            
            let trimmed = before_command.trim_end_matches(|c: char| c.is_whitespace() || c == '.' || c == '?' || c == '!');
            let sentence_terminator_re = Regex::new(r"[.?!]+").unwrap();
            let matches: Vec<_> = sentence_terminator_re.find_iter(trimmed).collect();
            
            let mut cut_index = 0;
            if let Some(last_match) = matches.last() {
                cut_index = last_match.end();
                // Skip the spaces following the terminator
                while cut_index < before_command.len() && before_command[cut_index..].starts_with(|c: char| c.is_whitespace()) {
                    let next_char_len = before_command[cut_index..].chars().next().unwrap().len_utf8();
                    cut_index += next_char_len;
                }
            }
            
            let mut new_resolved = String::new();
            new_resolved.push_str(&before_command[..cut_index]);
            // Do not push suffix here, because before_command[..cut_index] already has the correct punctuation/spacing,
            // and pushing suffix (which is just command's trailing punctuation) leads to duplicated periods.
            new_resolved.push_str(&resolved[command_end..]);
            
            resolved = new_resolved;
            changed = true;
            context.history.push(RewriteOperation::DeleteLastSentence {
                before: before_state,
                after: resolved.clone(),
            });
        }
    }
    resolved.trim().to_string()
}

pub fn resolve_replace_last_word(text: &str) -> String {
    resolve_replace_last_word_with_context(text, &mut RewriteContext::new())
}

pub fn resolve_replace_last_word_with_context(text: &str, context: &mut RewriteContext) -> String {
    let command_re = Regex::new(r"(?i)(^|[.?!,]\s*)(?:(?:please|can\s+you|could\s+you)\s+)?replace\s+(?:the\s+)?last\s+word\s+(?:with|to)\s+([^.?!]+)(\s*[.?!]+(?:\s+|$)|$)").unwrap();
    let word_re = Regex::new(r"\w+").unwrap();
    
    let mut resolved = text.to_string();
    let mut changed = true;
    while changed {
        changed = false;
        if let Some(caps) = command_re.captures(&resolved) {
            let before_state = resolved.clone();
            let command_start = caps.get(0).unwrap().start();
            let command_end = caps.get(0).unwrap().end();
            let prefix = caps.get(1).unwrap().as_str(); 
            let new_str = caps.get(2).unwrap().as_str();
            let suffix = caps.get(3).unwrap().as_str();
            
            let before_command = &resolved[..command_start + prefix.len()];
            
            let matches: Vec<_> = word_re.find_iter(before_command).collect();
            if let Some(last_match) = matches.last() {
                let mut new_resolved = String::new();
                new_resolved.push_str(&before_command[..last_match.start()]);
                
                let mut final_new = new_str.to_string();
                if let Some(first_char) = last_match.as_str().chars().next() {
                    if first_char.is_uppercase() {
                        let mut c = final_new.chars();
                        if let Some(f) = c.next() {
                            final_new = f.to_uppercase().collect::<String>() + c.as_str();
                        }
                    }
                }
                
                new_resolved.push_str(&final_new);
                let p = before_command[last_match.end()..].trim_end();
                
                let trailing_spaces = suffix.trim_start_matches(|c: char| !c.is_whitespace());
                let s_punct = suffix.trim();
                
                if !p.is_empty() {
                    new_resolved.push_str(p);
                } else {
                    new_resolved.push_str(s_punct);
                }
                new_resolved.push_str(trailing_spaces);
                new_resolved.push_str(&resolved[command_end..]);
                
                resolved = new_resolved;
                changed = true;
                context.history.push(RewriteOperation::ReplaceLastWord {
                    before: before_state,
                    after: resolved.clone(),
                });
            } else {
                let mut new_resolved = String::new();
                new_resolved.push_str(before_command);
                new_resolved.push_str(suffix.trim_start());
                new_resolved.push_str(&resolved[command_end..]);
                resolved = new_resolved;
                changed = true;
            }
        }
    }
    
    resolved.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_last_sentence() {
        assert_eq!(
            resolve_delete_last_sentence("Project is complete. Deployment is tomorrow. Delete last sentence"),
            "Project is complete."
        );
        assert_eq!(
            resolve_delete_last_sentence("First. Second! Third? delete previous sentence"),
            "First. Second!"
        );
    }

    #[test]
    fn test_replace_last_word() {
        assert_eq!(
            resolve_replace_last_word("Hello Rahul. Replace last word with Rajesh"),
            "Hello Rajesh."
        );
        assert_eq!(
            resolve_replace_last_word("Meeting is tomorrow. Replace the last word with Friday"),
            "Meeting is Friday."
        );
    }
    
    #[test]
    fn test_tier1_pipeline() {
        assert_eq!(
            resolve_all_tier1("The meeting is scheduled tomorrow. Delete last word"),
            "The meeting is scheduled."
        );
        assert_eq!(
            resolve_all_tier1("Project is complete. Deployment is tomorrow. Delete last sentence"),
            "Project is complete."
        );
        assert_eq!(
            resolve_all_tier1("Meeting in Chennai tomorrow. Replace Chennai with Bangalore"),
            "Meeting in Bangalore tomorrow."
        );
        assert_eq!(
            resolve_all_tier1("Hello Rahul. Replace last word with Rajesh"),
            "Hello Rajesh."
        );
    }
}
