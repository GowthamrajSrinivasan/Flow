use regex::Regex;

fn resolve_inline_corrections(text: &str) -> String {
    let mut resolved = text.to_string();
    
    let patterns = vec![
        r"(?i)(?P<old>\b\d+\w*\s+\S+)\s+(?:no wait|actually|i mean|scratch that)[^\w\s]*\s+(?P<new>\b\d+\w*\s+\S+)",
        r"(?i)(?P<old>\S+\s+\d+\w*)\s+(?:no wait|actually|i mean|scratch that)[^\w\s]*\s+(?P<new>\S+\s+\d+\w*)",
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

fn main() {
    let input = "Let's meet on June 1st no wait June 2nd.";
    println!("Input: {}", input);
    println!("Output: {}", resolve_inline_corrections(input));
}
