use crate::pipeline::request::FormattingMode;
use super::{punctuation, spacing, cleanup, vocabulary, capitalization};

pub fn format(text: &str, mode: FormattingMode) -> String {
    if mode == FormattingMode::Raw {
        return text.to_string();
    }
    
    // 1. Spoken Punctuation
    let mut current = punctuation::convert(text);
    
    // 2. Spacing Fixes
    current = spacing::fix_spacing(&current);
    
    // 3. Cleanup Artifacts
    current = cleanup::cleanup(&current);
    
    // 4. User Vocabulary
    // (In a real app, enabled state comes from processing options, passing true for now)
    current = vocabulary::expand_vocabulary(&current, true);
    
    // 5. Capitalization
    current = capitalization::capitalize(&current);
    
    // 6. Context Formatting
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
