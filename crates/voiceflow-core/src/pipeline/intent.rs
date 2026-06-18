#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserIntent {
    Dictation,
    Rewrite,
    Summarize,
    Translate,
    CorrectGrammar,
}

pub struct IntentDetector;

impl IntentDetector {
    pub fn detect(transcript: &str) -> UserIntent {
        let lower = transcript.to_lowercase();
        
        // Fast path for short transcripts which are definitely dictation
        if lower.len() < 10 {
            return UserIntent::Dictation;
        }

        // Rule-based and keyword matching
        if lower.starts_with("rewrite") || lower.contains("rewrite professionally") {
            return UserIntent::Rewrite;
        }
        
        if lower.starts_with("summarize") || lower.contains("summarize this") {
            return UserIntent::Summarize;
        }
        
        if lower.starts_with("translate") || lower.contains("translate to") {
            return UserIntent::Translate;
        }
        
        if lower.starts_with("correct grammar") || lower.contains("fix grammar") {
            return UserIntent::CorrectGrammar;
        }
        
        // Default to dictation
        UserIntent::Dictation
    }
}
