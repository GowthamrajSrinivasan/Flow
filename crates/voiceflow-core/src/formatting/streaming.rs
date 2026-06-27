#[derive(Debug, Clone, Copy)]
pub enum WindowPolicy {
    /// A fixed number of words to look back
    Fixed(usize),
    /// Look back to the start of the current sentence
    Sentence,
    /// Determine window size dynamically (e.g. based on pauses)
    Adaptive,
}

pub struct WindowBuilder {
    pub policy: WindowPolicy,
}

impl WindowBuilder {
    pub fn new(policy: WindowPolicy) -> Self {
        Self { policy }
    }

    /// Constructs a processing window from the previous buffer and the incoming delta.
    /// Returns a tuple of (start_index, combined_window).
    pub fn build_window(&self, previous_buffer: &str, delta: &str) -> (usize, String) {
        if previous_buffer.is_empty() {
            return (0, delta.to_string());
        }

        match self.policy {
            WindowPolicy::Fixed(words) => {
                let mut prev_words = previous_buffer.split_whitespace().collect::<Vec<_>>();
                let take_count = prev_words.len().min(words);
                
                if take_count == 0 {
                    return (previous_buffer.len(), delta.to_string());
                }
                
                let start_idx_words = prev_words.len() - take_count;
                let first_word_to_take = prev_words[start_idx_words];
                
                // Find the byte offset of `first_word_to_take` matching this occurrence.
                // Since we know it's the `start_idx_words`-th word, we can just split and track length.
                let mut byte_offset = 0;
                let mut word_idx = 0;
                
                let mut in_word = false;
                for (i, c) in previous_buffer.char_indices() {
                    if c.is_whitespace() {
                        in_word = false;
                    } else {
                        if !in_word {
                            if word_idx == start_idx_words {
                                byte_offset = i;
                                break;
                            }
                            word_idx += 1;
                        }
                        in_word = true;
                    }
                }
                
                let window_prefix = &previous_buffer[byte_offset..];
                
                // Determine if we need to insert a space between the prefix and delta
                let combined = if !window_prefix.is_empty() && !window_prefix.ends_with(char::is_whitespace) && !delta.starts_with(char::is_whitespace) {
                    format!("{} {}", window_prefix, delta)
                } else {
                    format!("{}{}", window_prefix, delta)
                };
                
                (byte_offset, combined)
            },
            WindowPolicy::Sentence => {
                // Find the last sentence boundary
                let mut byte_offset = 0;
                for (i, c) in previous_buffer.char_indices().rev() {
                    if c == '.' || c == '?' || c == '!' {
                        byte_offset = i + 1;
                        break;
                    }
                }
                let window_prefix = &previous_buffer[byte_offset..];
                let combined = if !window_prefix.is_empty() && !window_prefix.ends_with(char::is_whitespace) && !delta.starts_with(char::is_whitespace) {
                    format!("{} {}", window_prefix, delta)
                } else {
                    format!("{}{}", window_prefix, delta)
                };
                (byte_offset, combined)
            },
            WindowPolicy::Adaptive => {
                // For now, Adaptive behaves like Fixed(10)
                let builder = WindowBuilder::new(WindowPolicy::Fixed(10));
                builder.build_window(previous_buffer, delta)
            }
        }
    }
}

pub struct MergeEngine;

impl MergeEngine {
    pub fn new() -> Self {
        Self
    }

    /// Merges the formatted window back into the historic buffer.
    pub fn merge(&self, previous_buffer: &str, formatted_window: &str, window_start: usize) -> String {
        if window_start >= previous_buffer.len() {
            return formatted_window.to_string();
        }

        let prefix = &previous_buffer[..window_start];
        format!("{}{}", prefix, formatted_window)
    }

    /// Applies a ChangeSet to the given text buffer.
    pub fn apply_changeset(&self, text: &str, changeset: &crate::pipeline::changes::ChangeSet) -> String {
        use crate::pipeline::changes::Change;
        
        let mut result = text.to_string();
        
        // Note: For a robust implementation, changes should ideally be sorted in reverse order
        // (highest offset first) so that indices don't shift when applying multiple changes.
        // Or we use a more sophisticated rope structure. For now, we apply them sequentially.
        
        let mut offset_shift: isize = 0;
        
        for change in &changeset.changes {
            match change {
                Change::Replace { start, end, replacement } => {
                    let s = (*start as isize + offset_shift).max(0) as usize;
                    let e = (*end as isize + offset_shift).max(0) as usize;
                    
                    if s <= result.len() && e <= result.len() && s <= e {
                        let original_len = e - s;
                        let new_len = replacement.len();
                        result.replace_range(s..e, replacement);
                        offset_shift += new_len as isize - original_len as isize;
                    }
                }
                Change::Insert { offset, text: ins_text } => {
                    let o = (*offset as isize + offset_shift).max(0) as usize;
                    if o <= result.len() {
                        result.insert_str(o, ins_text);
                        offset_shift += ins_text.len() as isize;
                    }
                }
                Change::Delete { start, end } => {
                    let s = (*start as isize + offset_shift).max(0) as usize;
                    let e = (*end as isize + offset_shift).max(0) as usize;
                    
                    if s <= result.len() && e <= result.len() && s <= e {
                        result.replace_range(s..e, "");
                        offset_shift -= (e - s) as isize;
                    }
                }
                Change::Move { from_start, from_end, to_offset } => {
                    let fs = (*from_start as isize + offset_shift).max(0) as usize;
                    let fe = (*from_end as isize + offset_shift).max(0) as usize;
                    
                    if fs <= result.len() && fe <= result.len() && fs <= fe {
                        let text_to_move = result[fs..fe].to_string();
                        result.replace_range(fs..fe, "");
                        offset_shift -= (fe - fs) as isize;
                        
                        let to = (*to_offset as isize + offset_shift).max(0) as usize;
                        if to <= result.len() {
                            result.insert_str(to, &text_to_move);
                            offset_shift += text_to_move.len() as isize;
                        }
                    }
                }
            }
        }
        
        result
    }
}


pub struct StabilizationEngine {
    pub delay_ms: u64,
}

impl StabilizationEngine {
    pub fn new(delay_ms: u64) -> Self {
        Self { delay_ms }
    }

    pub fn is_stable(&self, delta: &str, age_ms: u64) -> bool {
        if delta.ends_with('.') || delta.ends_with('?') || delta.ends_with('!') {
            return true;
        }
        age_ms >= self.delay_ms
    }
}
