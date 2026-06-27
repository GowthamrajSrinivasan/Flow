use crate::formatting::metadata::RuleMetadata;
use crate::pipeline::models::{TransformationRequest, TransformationState};

pub trait FormatterRule: Send + Sync {
    fn metadata(&self) -> &'static RuleMetadata;
    
    // Check if the rule should run given the read-only request
    fn applies(&self, request: &TransformationRequest) -> bool;
    
    // Mutate the transformation state (contains text, tokens, and diagnostics)
    fn apply(&self, state: &mut TransformationState, request: &TransformationRequest);
}
