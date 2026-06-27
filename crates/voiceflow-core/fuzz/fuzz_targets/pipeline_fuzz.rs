#![no_main]
use libfuzzer_sys::fuzz_target;
use voiceflow_core::formatting::registry::RuleRegistry;
use voiceflow_core::pipeline::models::{TransformationRequest, TransformationState};

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        let registry = RuleRegistry::default();
        
        let mut state = TransformationState::new(text.to_string());
        let request = TransformationRequest::new(text.to_string());
        
        // This shouldn't panic on any random UTF-8 string
        registry.apply_all(&mut state, &request);
    }
});
