use serde::{Deserialize, Serialize};
use regex::{Regex, RegexBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyItem {
    pub canonical: String,
    pub aliases: Vec<String>,
}

pub struct VocabularyEngine {
    _items: Vec<VocabularyItem>,
    patterns: Vec<(Regex, String)>,
}

impl VocabularyEngine {
    pub fn new(items: Vec<VocabularyItem>) -> Self {
        let mut patterns = Vec::new();
        
        for item in &items {
            for alias in &item.aliases {
                // Word boundary matching, case insensitive
                let pattern_str = format!(r"(?i)\b{}\b", regex::escape(alias));
                if let Ok(re) = RegexBuilder::new(&pattern_str).case_insensitive(true).build() {
                    patterns.push((re, item.canonical.clone()));
                }
            }
        }
        
        // Sort patterns by length of the matched alias descending, so longer aliases match first
        patterns.sort_by(|a, b| b.0.as_str().len().cmp(&a.0.as_str().len()));

        Self { _items: items, patterns }
    }

    pub fn apply(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (re, canonical) in &self.patterns {
            result = re.replace_all(&result, canonical).to_string();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocabulary_replacement() {
        let items = vec![
            VocabularyItem {
                canonical: "Requill".to_string(),
                aliases: vec!["Requel".to_string(), "Requil".to_string(), "Re Quill".to_string()],
            },
            VocabularyItem {
                canonical: "Gowthamraj".to_string(),
                aliases: vec!["Gautham Raj".to_string(), "Gowtamraj".to_string()],
            }
        ];
        
        let engine = VocabularyEngine::new(items);
        
        let text = "I am working on requel. Gautham raj is here.";
        let result = engine.apply(text);
        assert_eq!(result, "I am working on Requill. Gowthamraj is here.");
    }
}
