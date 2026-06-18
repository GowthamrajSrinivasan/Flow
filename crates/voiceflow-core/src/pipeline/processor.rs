use std::time::Instant;

use crate::profile::RuntimeProfile;
use super::request::{ProcessingOptions, RewriteMode};
use super::response::ProcessingResult;
use super::intent::{IntentDetector, UserIntent};

pub struct VoiceFlowProcessor {
    profile: Box<dyn RuntimeProfile>,
}

impl VoiceFlowProcessor {
    pub fn new(profile: impl RuntimeProfile + 'static) -> Self {
        Self {
            profile: Box::new(profile),
        }
    }

    pub fn process(&self, transcript: &str, options: ProcessingOptions) -> ProcessingResult {
        let start_time = Instant::now();
        
        // 1. Intent Detection
        let intent = IntentDetector::detect(transcript);
        
        // 2. Formatter Engine
        // TODO: Call formatting pipeline
        let formatted_text = crate::formatting::formatter::format(transcript, options.formatting_mode);
        
        // 3. LLM Rewrite Layer
        let (final_text, used_llm) = if options.llm_enabled && options.rewrite_mode != RewriteMode::Off && intent != UserIntent::Dictation {
            // TODO: Call LLM Rewrite Engine
            let rewritten = crate::llm::rewrite::rewrite(&formatted_text, intent, options.rewrite_mode, self.profile.as_ref());
            (rewritten, true)
        } else {
            (formatted_text.clone(), false)
        };
        
        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        ProcessingResult {
            raw_text: transcript.to_string(),
            formatted_text,
            final_text,
            intent,
            used_llm,
            processing_time_ms,
        }
    }
}
