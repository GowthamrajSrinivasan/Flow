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
