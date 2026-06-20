use regex::Regex;
fn main() {
    let input1 = "Let's meet on June 1st, no wait June 2nd.";
    let pattern = r"(?i)(?P<old>\S+\s+\d+\S*)\s+(?:no wait|actually|i mean|scratch that)[^\w\s]*\s+(?P<new>\S+\s+\d+\S*)";
    let re = Regex::new(pattern).unwrap();
    if let Some(caps) = re.captures(input1) {
        println!("Matched!");
    } else {
        println!("Did not match pattern 2");
    }
}
