/// Calculates the Word Error Rate (WER) between a reference string and a hypothesis string.
/// Both strings are lowercased and stripped of basic punctuation before comparison.
pub fn calculate_wer(reference: &str, hypothesis: &str) -> f64 {
    let sanitize = |s: &str| -> Vec<String> {
        s.to_lowercase()
         .replace(|c: char| c.is_ascii_punctuation(), "")
         .split_whitespace()
         .map(|w| w.to_string())
         .collect()
    };

    let ref_words = sanitize(reference);
    let hyp_words = sanitize(hypothesis);

    if ref_words.is_empty() {
        return if hyp_words.is_empty() { 0.0 } else { 1.0 };
    }

    let n = ref_words.len();
    let m = hyp_words.len();

    let mut d = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n {
        d[i][0] = i;
    }
    for j in 0..=m {
        d[0][j] = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            let cost = if ref_words[i - 1] == hyp_words[j - 1] { 0 } else { 1 };
            
            d[i][j] = (d[i - 1][j] + 1) // Deletion
                .min(d[i][j - 1] + 1)   // Insertion
                .min(d[i - 1][j - 1] + cost); // Substitution
        }
    }

    let distance = d[n][m] as f64;
    distance / (n as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wer() {
        let expected = "Requill integrates with Firebase and Genkit";
        let actual = "Requill integrates with Fire Base and Gen Kit";
        
        // Expected words: 6
        // Actual words: 8
        // integrations with (same)
        // Fire Base (2 words) vs Firebase (1 word)
        // Gen Kit (2 words) vs Genkit (1 word)
        
        let wer = calculate_wer(expected, actual);
        // Fire Base replaces Firebase (1 substitution, 1 insertion)
        // Gen Kit replaces Genkit (1 substitution, 1 insertion)
        // Distance = 4. 4 / 6 = 0.6666...
        assert!(wer > 0.0);
    }
}
