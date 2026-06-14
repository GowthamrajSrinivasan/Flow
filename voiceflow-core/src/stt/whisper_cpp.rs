use crate::stt::SpeechRecognizer;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct WhisperCppRecognizer {
    ctx: WhisperContext,
    audio_buffer: Vec<f32>,
}

impl WhisperCppRecognizer {
    pub fn new(model_path: &str) -> Result<Self, String> {
        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(model_path, params)
            .map_err(|e| format!("Failed to load model: {}", e))?;
        Ok(Self { 
            ctx,
            audio_buffer: Vec::new(),
        })
    }
}

impl SpeechRecognizer for WhisperCppRecognizer {
    fn start_stream(&mut self) {
        self.audio_buffer.clear();
    }

    fn process_audio(&mut self, audio: &[f32]) {
        self.audio_buffer.extend_from_slice(audio);
    }

    fn partial_result(&self) -> Option<String> {
        None
    }

    fn final_result(&mut self) -> String {
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
        result.trim().to_string()
    }
}
