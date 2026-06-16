use std::path::Path;
use parakeet_rs::{ParakeetEOU, ParakeetEOUHandle};
use crate::stt::SpeechRecognizer;

pub struct ParakeetRecognizer {
    handle: ParakeetEOUHandle,
    session: Option<ParakeetEOU>,
    partial_text: String,
}

impl ParakeetRecognizer {
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self, String> {
        let handle = ParakeetEOUHandle::load(model_path, None)
            .map_err(|e| format!("Failed to load Parakeet model: {:?}", e))?;
        
        Ok(Self {
            handle,
            session: None,
            partial_text: String::new(),
        })
    }
}

impl SpeechRecognizer for ParakeetRecognizer {
    fn start_stream(&mut self) {
        self.session = Some(ParakeetEOU::from_shared(&self.handle));
        self.partial_text.clear();
    }

    fn process_audio(&mut self, audio: &[f32]) {
        if let Some(session) = &mut self.session {
            // Process chunk. We'll use chunk sizes of 2560 (160ms) if possible, but 
            // the wrapper will handle any size. We just push the incoming audio
            // in chunks to the transcribe function.
            // But ParakeetEOU::transcribe expects typical chunks. Let's feed it directly.
            match session.transcribe(audio, false) {
                Ok(text) => {
                    if !text.is_empty() {
                        self.partial_text.push_str(&text);
                    }
                }
                Err(e) => {
                    eprintln!("Parakeet transcription error: {:?}", e);
                }
            }
        }
    }

    fn partial_result(&mut self) -> Option<String> {
        if self.partial_text.is_empty() {
            None
        } else {
            Some(self.partial_text.clone())
        }
    }

    fn final_result(&mut self) -> String {
        let result = self.partial_text.clone();
        self.partial_text.clear();
        result
    }
}
