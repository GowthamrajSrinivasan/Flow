use regex::Regex;

pub fn capitalize(text: &str) -> String {
    let mut result = text.to_string();
    
    // Capitalize first letter of the text
    if let Some(first_char) = result.chars().next() {
        if first_char.is_lowercase() {
            let mut c_iter = result.chars();
            let first = c_iter.next().unwrap().to_uppercase().to_string();
            result = first + c_iter.as_str();
        }
    }
    
    // Capitalize after period, question mark, or exclamation mark followed by a space
    let re_sentence = Regex::new(r"([\.!\?]\s+)([a-z])").unwrap();
    result = re_sentence.replace_all(&result, |caps: &regex::Captures| {
        format!("{}{}", &caps[1], caps[2].to_uppercase())
    }).to_string();
    
    // Capitalize after newlines
    let re_newline = Regex::new(r"(\n+)([a-z])").unwrap();
    result = re_newline.replace_all(&result, |caps: &regex::Captures| {
        format!("{}{}", &caps[1], caps[2].to_uppercase())
    }).to_string();

    result
}
