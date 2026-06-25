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
    let mut new_result = String::with_capacity(result.len());
    let mut last_char = '\0';
    for c in result.chars() {
        if ".,?!:;".contains(c) && c == last_char {
            continue;
        }
        new_result.push(c);
        last_char = c;
    }
    result = new_result;
    
    result.trim().to_string()
}
