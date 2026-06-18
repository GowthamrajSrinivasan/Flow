use regex::Regex;

pub fn cleanup(text: &str) -> String {
    let mut result = text.to_string();
    
    // Remove duplicate spaces
    let re_spaces = Regex::new(r" {2,}").unwrap();
    result = re_spaces.replace_all(&result, " ").to_string();
    
    // Remove duplicate newlines (limit to 2 max)
    let re_newlines = Regex::new(r"\n{3,}").unwrap();
    result = re_newlines.replace_all(&result, "\n\n").to_string();
    
    // Remove duplicate punctuation
    let re_punct = Regex::new(r"([,\.\?\!\:\;])\1+").unwrap();
    result = re_punct.replace_all(&result, "$1").to_string();
    
    result.trim().to_string()
}
