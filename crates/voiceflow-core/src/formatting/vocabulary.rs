use regex::Regex;

pub fn expand_vocabulary(text: &str, _enabled: bool) -> String {
    // In a real implementation, this would load user vocabulary from a config or database
    let mut result = text.to_string();
    
    // Stub mappings
    let mappings = [
        ("cmg", "CMG"),
        ("petro signs", "PetroSigns"),
        ("qwen", "Qwen3"),
    ];
    
    for (word, replacement) in mappings.iter() {
        let pattern = format!(r"(?i)\b{}\b", word);
        if let Ok(re) = Regex::new(&pattern) {
            result = re.replace_all(&result, *replacement).to_string();
        }
    }
    
    result
}
