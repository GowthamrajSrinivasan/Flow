use regex::Regex;

pub fn strip_filler(text: &str) -> String {
    let mut clean = text.trim().to_string();
    
    // Strip common LLM prefixes
    let prefixes = [
        "Sure! Here is the rewritten text:",
        "Sure, here's the rewritten text:",
        "Here is the rewritten text:",
        "Here's the rewritten text:",
        "I improved the grammar:",
        "Here is the text:",
        "Here you go:",
    ];
    
    for prefix in prefixes.iter() {
        if clean.to_lowercase().starts_with(&prefix.to_lowercase()) {
            clean = clean[prefix.len()..].trim().to_string();
        }
    }
    
    // Sometimes LLMs output the result in quotes
    let re_quotes = Regex::new(r#"^"([\s\S]*)"$"#).unwrap();
    if let Some(caps) = re_quotes.captures(&clean) {
        clean = caps[1].trim().to_string();
    }
    
    clean
}
