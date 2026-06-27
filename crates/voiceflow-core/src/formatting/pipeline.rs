use crate::formatting::context::FormattingContext;
use crate::formatting::registry::RuleRegistry;
use crate::pipeline::models::{TransformationRequest, TransformationResult, TransformationState, TransformationMetrics};
use std::time::Instant;

pub struct TransformationPipeline {
    registry: RuleRegistry,
}

impl TransformationPipeline {
    pub fn new(registry: RuleRegistry) -> Self {
        Self { registry }
    }

    pub fn transform(&self, request: &TransformationRequest) -> TransformationResult {
        let start_time = Instant::now();
        let chars_in = request.input.len();
        
        let mut state = TransformationState::new(request.input.clone());

        self.registry.apply_all(&mut state, request);

        let elapsed = start_time.elapsed().as_millis();
        let chars_out = state.current_text.len();
        
        let metrics = TransformationMetrics {
            duration_ms: elapsed,
            rules_executed: state.diagnostics.len(),
            characters_in: chars_in,
            characters_out: chars_out,
            allocations: 0, // Placeholder
        };

        TransformationResult::new(state.current_text, state.changes, state.diagnostics, metrics)
    }
}
