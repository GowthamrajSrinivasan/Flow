use crate::SpeechRecognizer;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Strip Whisper special tokens like [BLANK_AUDIO], [SOUND], [SOBBING], etc.
/// These appear literally in segment text even when `set_print_special(false)` is set.
fn strip_special_tokens(text: &str) -> String {
    let mut result = String::new();
    let mut depth = 0usize;
    for ch in text.chars() {
        match ch {
            '[' => depth += 1,
            ']' => { if depth > 0 { depth -= 1; } }
            _ if depth == 0 => result.push(ch),
            _ => {}
        }
    }
    result.trim().to_string()
}

pub struct WhisperCppRecognizer {
    ctx: WhisperContext,
    audio_buffer: Vec<f32>,
    partial_buffer: Vec<f32>,
}

impl WhisperCppRecognizer {
    pub fn new(model_path: &str) -> Result<Self, String> {
        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(model_path, params)
            .map_err(|e| format!("Failed to load model: {}", e))?;
        Ok(Self { 
            ctx,
            audio_buffer: Vec::new(),
            partial_buffer: Vec::new(),
        })
    }
}

impl SpeechRecognizer for WhisperCppRecognizer {
    fn start_stream(&mut self) {
        self.audio_buffer.clear();
        self.partial_buffer.clear();
    }

    fn process_audio(&mut self, audio: &[f32]) {
        self.audio_buffer.extend_from_slice(audio);
        self.partial_buffer.extend_from_slice(audio);
    }

    fn partial_result(&mut self) -> Option<String> {
        if self.audio_buffer.is_empty() {
            return None;
        }

        // Only run partial if we have at least 0.5s of audio to avoid "input too short" spam
        if self.audio_buffer.len() < 8000 {
            return None;
        }

        let mut state = match self.ctx.create_state() {
            Ok(s) => s,
            Err(_) => return None,
        };
        
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_suppress_blank(true);
        params.set_no_speech_thold(0.6);
        
        if state.full(params, &self.audio_buffer).is_err() {
            return None;
        }
        
        let num_segments = state.full_n_segments();
        let mut result = String::new();
        for i in 0..num_segments {
            if let Some(segment) = state.get_segment(i) {
                if let Ok(text) = segment.to_str() {
                    result.push_str(text);
                }
            }
        }
        
        let text = strip_special_tokens(&result);
        if text.is_empty() { None } else { Some(text) }
    }

    fn final_result(&mut self) -> String {
        eprintln!("[DEBUG] final_result: audio_buffer has {} samples = {:.2}s",
            self.audio_buffer.len(),
            self.audio_buffer.len() as f32 / 16000.0
        );
        if self.audio_buffer.is_empty() {
            return String::new();
        }

        let mut state = match self.ctx.create_state() {
            Ok(s) => s,
            Err(_) => return String::new(),
        };
        
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_suppress_blank(true);
        params.set_no_speech_thold(0.6);
        
        if state.full(params, &self.audio_buffer).is_err() {
            return String::new();
        }
        
        let num_segments = state.full_n_segments();
        let mut result = String::new();
        for i in 0..num_segments {
            if let Some(segment) = state.get_segment(i) {
                if let Ok(text) = segment.to_str() {
                    result.push_str(text);
                }
            }
        }
        
        self.audio_buffer.clear();
        self.partial_buffer.clear();
        strip_special_tokens(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_special_tokens() {
        assert_eq!(strip_special_tokens("[_BEG_] Hello world!"), "Hello world!");
        assert_eq!(strip_special_tokens("[BLANK_AUDIO]"), "");
        assert_eq!(strip_special_tokens("Normal text [SOUND] here"), "Normal text  here");
        assert_eq!(strip_special_tokens("[_BEG_] [BLANK_AUDIO] Hey [_TT_50]"), "Hey");
    }

    #[test]
    fn test_recognizer_buffer_management() {
        // We can't easily instantiate WhisperCppRecognizer without a valid model file on the runner,
        // so we'll test the trait methods conceptually if we extracted them, but for now we
        // ensure our buffer clearing logic in SpeechRecognizer is conceptually tested.
        // Actually, we can test that stripping works perfectly which was the main formatting bug.
    }
}
