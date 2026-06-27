use voiceflow_core::formatting::registry::PipelineBuilder;
use voiceflow_core::formatting::metadata::{RuleCategory, RuleId, RuleCapabilities, RuleMetadata};
use voiceflow_core::formatting::traits::FormatterRule;
use voiceflow_core::pipeline::models::{TransformationRequest, TransformationState};

// Mock rules for testing
struct MockRule(&'static RuleMetadata);

impl FormatterRule for MockRule {
    fn metadata(&self) -> &'static RuleMetadata {
        self.0
    }
    
    fn applies(&self, _: &TransformationRequest) -> bool { true }
    fn apply(&self, _: &mut TransformationState, _: &TransformationRequest) {}
}

const META_A: RuleMetadata = RuleMetadata {
    id: RuleId("A"),
    name: "A",
    version: "1.0",
    category: RuleCategory::Lexical,
    priority: 10,
    capabilities: RuleCapabilities { streaming_safe: true, token_based: false, regex_based: true, locale_aware: false, developer_only: false, markdown_only: false, incremental_safe: true },
    depends_on: &[],
};

const META_B: RuleMetadata = RuleMetadata {
    id: RuleId("B"),
    name: "B",
    version: "1.0",
    category: RuleCategory::Lexical,
    priority: 10,
    capabilities: RuleCapabilities { streaming_safe: true, token_based: false, regex_based: true, locale_aware: false, developer_only: false, markdown_only: false, incremental_safe: true },
    depends_on: &[RuleId("A")],
};

const META_C: RuleMetadata = RuleMetadata {
    id: RuleId("C"),
    name: "C",
    version: "1.0",
    category: RuleCategory::Lexical,
    priority: 20, // High priority, but depends on B
    capabilities: RuleCapabilities { streaming_safe: true, token_based: false, regex_based: true, locale_aware: false, developer_only: false, markdown_only: false, incremental_safe: true },
    depends_on: &[RuleId("B")],
};

const META_D: RuleMetadata = RuleMetadata {
    id: RuleId("D"),
    name: "D",
    version: "1.0",
    category: RuleCategory::Lexical,
    priority: 5, // Low priority, no deps, runs after C
    capabilities: RuleCapabilities { streaming_safe: true, token_based: false, regex_based: true, locale_aware: false, developer_only: false, markdown_only: false, incremental_safe: true },
    depends_on: &[],
};

#[test]
fn test_pipeline_execution_order() {
    // Even if added in reverse order, topological sort should resolve it
    let registry = PipelineBuilder::new()
        .add_rule(Box::new(MockRule(&META_D)))
        .add_rule(Box::new(MockRule(&META_C)))
        .add_rule(Box::new(MockRule(&META_B)))
        .add_rule(Box::new(MockRule(&META_A)))
        .add_pass(RuleCategory::Lexical)
        .build();
    
    let ordered = registry.ordered_rule_names();
    assert_eq!(ordered, vec!["A", "B", "C", "D"]);
}
