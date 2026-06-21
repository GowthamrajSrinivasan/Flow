use regex::Regex;

pub fn resolve_false_starts(text: &str) -> String {
    // Basic tokenizer that keeps track of word boundaries and original text
    let word_regex = Regex::new(r"(\w+)").unwrap();
    
    let mut words = Vec::new();
    let mut matches = Vec::new();
    
    for m in word_regex.find_iter(text) {
        words.push(m.as_str().to_lowercase());
        matches.push((m.start(), m.end()));
    }

    if words.is_empty() {
        return text.to_string();
    }

    let mut deleted_ranges: Vec<(usize, usize)> = Vec::new();

    let mut i = 0;
    while i < words.len() {
        let mut best_match = None;
        let mut best_score = 0.0;

        // Try to find the largest N that matches a future sequence
        let max_n = words.len() - i;
        for n in (1..=max_n).rev() {
            let mut j = i + n;
            let max_j = std::cmp::min(words.len() - n, i + n + 10); // Limit lookahead to 10 words
            
            while j <= max_j {
                let mut is_match = true;
                for k in 0..n {
                    if words[i + k] != words[j + k] {
                        is_match = false;
                        break;
                    }
                }
                
                if is_match {
                    let d = j - i;
                    let score = n as f32 / d as f32;
                    
                    if score >= 0.5 && (n >= 2 || d <= 2) { // false start condition
                        if score > best_score || (score == best_score && n > best_match.unwrap_or((0,0,0)).2) {
                            best_match = Some((i, j, n));
                            best_score = score;
                        }
                    }
                }
                j += 1;
            }
        }

        if let Some((best_i, best_j, _)) = best_match {
            deleted_ranges.push((best_i, best_j));
            i = best_j; // Skip over the aborted part
        } else {
            i += 1;
        }
    }

    if deleted_ranges.is_empty() {
        return text.to_string();
    }

    // Reconstruct the string
    let mut result = String::new();
    let mut last_end = 0;
    
    // We want to remove the text corresponding to words[i..j].
    // This includes trailing whitespace/punctuation after the aborted part,
    // so we delete from the start of word `i` to the start of word `j`.
    
    for (start_word, end_word) in deleted_ranges {
        let start_char = matches[start_word].0;
        let end_char = matches[end_word].0;
        
        result.push_str(&text[last_end..start_char]);
        last_end = end_char;
    }
    
    result.push_str(&text[last_end..]);

    // Capitalize the first letter if needed (basic fix)
    let mut chars: Vec<char> = result.trim_start().chars().collect();
    if let Some(first) = chars.first_mut() {
        *first = first.to_ascii_uppercase();
    }
    
    result.trim().to_string()
}

fn main() {
    let cases = vec![
        ("Let's schedule a meeting. Let's schedule a call with Rahul.", "Let's schedule a call with Rahul."),
        ("The issue is. The issue is in production.", "The issue is in production."),
        ("Please send. Please send the report.", "Please send the report."),
        ("I will be available tomorrow. I will be available next week.", "I will be available next week."),
        ("Let's schedule a meeting let's schedule a call with Rahul.", "Let's schedule a call with Rahul."),
        ("I went to the store. I went to the bank.", "I went to the bank."),
    ];

    for (input, expected) in cases {
        let output = resolve_false_starts(input);
        println!("Input: {}", input);
        println!("Output: {}", output);
        println!("Expected: {}", expected);
        println!("Match: {}\n", output == expected);
    }
}
