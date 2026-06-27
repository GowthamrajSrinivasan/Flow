use regex::Regex;
use crate::pipeline::context::FormatterContext;
use voiceflow_shared::config::vocabulary::VocabularyEntry;

pub fn expand_vocabulary(text: &str, context: &FormatterContext) -> String {
    let mut result = text.to_string();
    
    let mut entries = Vec::new();

    // 1. Load User Vocabulary if available
    if let Some(vocab) = &context.vocabulary {
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
        // Replace escaped spaces with \s+ to handle varying whitespace in the input text
        let flexible_spaces = escaped_spoken.replace("\\ ", r"\s+");
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
