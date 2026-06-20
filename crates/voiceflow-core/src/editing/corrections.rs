use regex::Regex;

pub fn resolve_inline_corrections(text: &str) -> String {
    let mut resolved = text.to_string();
    
    let patterns = vec![
        r"(?i)(?P<old>\b\d+\S*\s+\S+)\s+(?:no wait|actually|i mean|scratch that)[^\w\s]*\s+(?P<new>\b\d+\S*\s+\S+)",
        r"(?i)(?P<old>\S+\s+\d+\S*)\s+(?:no wait|actually|i mean|scratch that)[^\w\s]*\s+(?P<new>\S+\s+\d+\S*)",
        r"(?i)(?P<old>\S+)\s+(?:no wait|actually|i mean|scratch that)[^\w\s]*\s+(?P<new>\S+)",
    ];

    for pattern in patterns {
        if let Ok(re) = Regex::new(pattern) {
            let mut previous = String::new();
            while resolved != previous {
                previous = resolved.clone();
                resolved = re.replace_all(&resolved, "$new").to_string();
            }
        }
    }

    resolved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordinals() {
        let input = "Let's meet on June 1st no wait June 2nd.";
        let output = resolve_inline_corrections(input);
        assert_eq!(output, "Let's meet on June 2nd.");
    }

    #[test]
    fn test_ordinals_with_commas() {
        let input = "Let's meet on June 1st, no wait June 2nd.";
        let output = resolve_inline_corrections(input);
        assert_eq!(output, "Let's meet on June 2nd.");
    }
}
