use regex::Regex;

pub fn fix_spacing(text: &str) -> String {
    let mut result = text.to_string();
    
    // Remove space before punctuation
    let re_before = Regex::new(r"\s+([,\.\?\!\:\;])").unwrap();
    result = re_before.replace_all(&result, "$1").to_string();
    
    // Ensure single space after punctuation (except if followed by newline or another punctuation)
    let re_after = Regex::new(r"([,\.\?\!\:\;])([^\s\n\.\,\!\?])").unwrap();
    result = re_after.replace_all(&result, "$1 $2").to_string();
    
    result
}
