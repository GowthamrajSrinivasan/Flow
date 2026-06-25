use regex::Regex;

pub fn format(text: &str) -> String {
    let mut result = text.to_string();

    // Blacklist of words that usually precede a dictated URL or Email, 
    // but are NOT part of it.
    let blacklist = vec![
        "go", "to", "visit", "is", "at", "contact", "email", "website", 
        "on", "in", "for", "and", "the", "a", "an", "my", "your"
    ];

    let is_blacklisted = |w: &str| {
        let lower = w.to_lowercase();
        blacklist.contains(&lower.as_str())
    };

    // 1. Spoken Emails
    let email_regex = Regex::new(r"(?i)\b([a-z0-9\s]+)\s+at\s+([a-z0-9\s]+)\s+dot\s+(com|net|org|io|co|edu)\b").unwrap();
    result = email_regex.replace_all(&result, |caps: &regex::Captures| {
        let left_raw = caps[1].to_string();
        let domain_raw = caps[2].to_string();
        let tld = caps[3].to_string();

        let mut words: Vec<&str> = left_raw.split_whitespace().collect();
        let mut email_left_words = Vec::new();
        for w in words.iter().rev() {
            if is_blacklisted(w) && w.to_lowercase() != "dot" {
                break;
            }
            email_left_words.push(*w);
        }
        email_left_words.reverse();
        
        let keep_count = words.len() - email_left_words.len();
        words.truncate(keep_count);
        let prefix = if words.is_empty() {
            String::new()
        } else {
            format!("{} ", words.join(" "))
        };

        let formatted_left = email_left_words.join(" ").replace(" dot ", ".").replace(" ", "");
        let formatted_domain = domain_raw.replace(" dot ", ".").replace(" ", "");

        format!("{}{}@{}.{}", prefix, formatted_left, formatted_domain, tld)
    }).to_string();

    // 2. Spoken URLs
    let url_regex = Regex::new(r"(?i)\b([a-z0-9\s]+)\s+dot\s+(com|net|org|io|co|edu)(?:\s+slash\s+([a-z0-9]+))?\b").unwrap();
    result = url_regex.replace_all(&result, |caps: &regex::Captures| {
        let left_raw = caps[1].to_string();
        let tld = caps[2].to_string();
        let path = caps.get(3).map(|m| m.as_str().to_string());

        let mut words: Vec<&str> = left_raw.split_whitespace().collect();
        let mut url_left_words = Vec::new();
        
        let mut www_index = None;
        for (i, w) in words.iter().enumerate().rev() {
            if w.to_lowercase() == "www" {
                www_index = Some(i);
                break;
            }
        }
        
        if let Some(idx) = www_index {
            for w in &words[idx..] {
                url_left_words.push(*w);
            }
            words.truncate(idx);
        } else {
            for w in words.iter().rev() {
                if is_blacklisted(w) && w.to_lowercase() != "dot" {
                    break;
                }
                url_left_words.push(*w);
            }
            url_left_words.reverse();
            let keep_count = words.len() - url_left_words.len();
            words.truncate(keep_count);
        }
        
        let prefix = if words.is_empty() {
            String::new()
        } else {
            format!("{} ", words.join(" "))
        };
        
        let formatted_left = url_left_words.join(" ").replace(" dot ", ".").replace(" ", "");
        let formatted_path = match path {
            Some(p) => format!("/{}", p.replace(" ", "")),
            None => String::new()
        };
        
        format!("{}{}.{}{}", prefix, formatted_left, tld, formatted_path)
    }).to_string();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_emails() {
        assert_eq!(format("my email is john dot doe at gmail dot com"), "my email is john.doe@gmail.com");
        assert_eq!(format("contact admin at open ai dot org please"), "contact admin@openai.org please");
    }

    #[test]
    fn test_format_urls() {
        assert_eq!(format("go to open ai dot com slash pricing today"), "go to openai.com/pricing today");
        assert_eq!(format("visit www dot my site dot net"), "visit www.mysite.net");
    }
}
