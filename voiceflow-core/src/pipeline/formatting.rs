use regex::Regex;

pub struct FormattingEngine {
    filler_re: Regex,
    multi_space_re: Regex,
}

impl FormattingEngine {
    pub fn new() -> Self {
        // Basic filler removal (case insensitive, bounded by word boundaries)
        let filler_re = Regex::new(r"(?i)\b(um|uh|like|you know)\b").unwrap();
        let multi_space_re = Regex::new(r"\s+").unwrap();
        
        Self {
            filler_re,
            multi_space_re,
        }
    }

    pub fn apply(&self, text: &str) -> String {
        let mut result = text.replace("[BLANK_AUDIO]", "").replace("[_BLANK_AUDIO_]", "");

        // 1. Remove fillers
        result = self.filler_re.replace_all(&result, "").to_string();
        
        // 2. Clean up multiple spaces
        result = self.multi_space_re.replace_all(&result, " ").to_string();
        result = result.trim().to_string();

        // 3. Capitalization and Basic Punctuation
        if !result.is_empty() {
            let mut chars: Vec<char> = result.chars().collect();
            
            // Capitalize first letter
            if chars[0].is_ascii_lowercase() {
                chars[0] = chars[0].to_ascii_uppercase();
            }

            // Ensure ending punctuation
            let last_char = chars.last().unwrap();
            if !['.', '!', '?'].contains(last_char) {
                chars.push('.');
            }
            
            result = chars.into_iter().collect();
        }

        result
    }
}

impl Default for FormattingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatting_engine() {
        let engine = FormattingEngine::new();
        
        // Test basic formatting
        let text = " um i think like we should go now ";
        let formatted = engine.apply(text);
        assert_eq!(formatted, "I think we should go now.");

        // Test punctuation existing
        let text2 = "hello world!";
        assert_eq!(engine.apply(text2), "Hello world!");
    }
}
