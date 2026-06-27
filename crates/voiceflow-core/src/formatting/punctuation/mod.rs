use regex::Regex;

pub fn convert(text: &str) -> String {
    let mut result = text.to_string();
    
    let mappings = [
        ("comma", ","),
        ("period", "."),
        ("full stop", "."),
        ("question mark", "?"),
        ("exclamation mark", "!"),
        ("colon", ":"),
        ("semicolon", ";"),
        ("dash", "-"),
        ("hyphen", "-"),
        ("new line", "\n"),
        ("new paragraph", "\n\n"),
        ("open quote", "\""),
        ("close quote", "\""),
        ("open bracket", "("),
        ("close bracket", ")"),
    ];
    
    // Note: In a robust implementation, this would use word boundaries
    // to avoid matching "command" as "comma" + "nd".
    for (word, punc) in mappings.iter() {
        // Simple case-insensitive replacement with word boundaries
        let pattern = format!(r"(?i)\b{}\b", word);
        if let Ok(re) = Regex::new(&pattern) {
            result = re.replace_all(&result, *punc).to_string();
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spoken_punctuation() {
        assert_eq!(convert("hello comma world period"), "hello , world .");
        assert_eq!(convert("this is a test new paragraph wait new line yes"), "this is a test \n\n wait \n yes");
        assert_eq!(convert("what is this question mark"), "what is this ?");
        assert_eq!(convert("stop exclamation mark"), "stop !");
    }

    #[test]
    fn test_case_insensitivity() {
        assert_eq!(convert("HELLO COMMA WORLD"), "HELLO , WORLD");
        assert_eq!(convert("Next New Paragraph Okay"), "Next \n\n Okay");
    }

    #[test]
    fn test_word_boundaries() {
        assert_eq!(convert("commander"), "commander");
        assert_eq!(convert("new liner"), "new liner");
    }
}
