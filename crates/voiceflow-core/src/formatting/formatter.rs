use crate::pipeline::request::FormattingMode;
use super::{punctuation, spacing, cleanup, vocabulary, capitalization, email_url, lists};

pub fn format(text: &str, mode: FormattingMode) -> String {
    if mode == FormattingMode::Raw {
        return text.to_string();
    }
    
    // 1. Spoken Punctuation
    let mut current = punctuation::convert(text);
    
    // 2. Email/URL Formatting (Tier 4 Phase 2)
    current = email_url::format(&current);

    // 3. Lists (Tier 4 Phase 3)
    current = lists::format(&current);

    // 4. Spacing Fixes
    current = spacing::fix_spacing(&current);
    
    // 4. Cleanup Artifacts
    current = cleanup::cleanup(&current);
    
    // 5. User Vocabulary
    // (In a real app, enabled state comes from processing options, passing true for now)
    current = vocabulary::expand_vocabulary(&current, true);
    
    // 6. Capitalization
    current = capitalization::capitalize(&current);
    
    // 7. Context Formatting
    match mode {
        FormattingMode::Email => {
            // Very simple deterministic fix as example
            if current.to_lowercase().starts_with("thanks,") && !current.contains('\n') {
                current = current.replacen(" ", "\n", 1);
            }
        },
        FormattingMode::Chat => {
            // E.g., make it shorter or remove trailing periods, just returning for now
        },
        _ => {}
    }
    
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier4_pipeline_integration() {
        let input = "hello comma here is my email john dot doe at gmail dot com period here are the items we need colon apples comma bananas comma and oranges period please convert to bullet list";
        
        let expected = "Hello, here is my email john.doe@gmail.com. Here are the items we need:\n- apples\n- bananas\n- oranges.";
        
        let formatted = format(input, FormattingMode::Smart);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_tier4_url_and_punctuation() {
        let input = "go to open ai dot com slash pricing today comma and check the plans period";
        let expected = "Go to openai.com/pricing today, and check the plans.";
        let formatted = format(input, FormattingMode::Smart);
        assert_eq!(formatted, expected);
    }
}
