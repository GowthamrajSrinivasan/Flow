use regex::Regex;
fn main() {
    let input = "Let's meet on June 1st no wait June 2nd.";
    let pattern = r"(?i)(?P<old>\S+\s+\d+\w*)\s+(?:no wait|actually|i mean|scratch that)[^\w\s]*\s+(?P<new>\S+\s+\d+\w*)";
    let re = Regex::new(pattern).unwrap();
    if let Some(caps) = re.captures(input) {
        println!("Matched!");
        println!("Old: {}", &caps["old"]);
        println!("New: {}", &caps["new"]);
    } else {
        println!("Did not match.");
    }
}
