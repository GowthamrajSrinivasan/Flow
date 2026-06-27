use crate::formatting::traits::FormatterRule;
use crate::formatting::metadata::RuleCategory;
use crate::pipeline::models::{TransformationRequest, TransformationState};

pub struct PipelineBuilder {
    rules: Vec<Box<dyn FormatterRule>>,
    passes: Vec<RuleCategory>,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            passes: Vec::new(),
        }
    }

    pub fn add_rule(mut self, rule: Box<dyn FormatterRule>) -> Self {
        self.rules.push(rule);
        // Sort rules by priority descending globally
        self.rules.sort_by(|a, b| b.metadata().priority.cmp(&a.metadata().priority));
        self
    }

    pub fn add_pass(mut self, category: RuleCategory) -> Self {
        self.passes.push(category);
        self
    }

    pub fn build(self) -> RuleRegistry {
        RuleRegistry {
            rules: self.rules,
            passes: self.passes,
        }
    }
}

pub struct RuleRegistry {
    rules: Vec<Box<dyn FormatterRule>>,
    passes: Vec<RuleCategory>,
}

impl RuleRegistry {
    pub fn apply_all(&self, state: &mut TransformationState, request: &TransformationRequest) {
        // Multi-pass execution
        for pass in &self.passes {
            for rule in &self.rules {
                if rule.metadata().category == *pass && rule.applies(request) {
                    let text_before = state.current_text.clone();
                    
                    rule.apply(state, request);
                    
                    if text_before != state.current_text {
                        state.changes.add(crate::pipeline::changes::Change::Replace {
                            start: 0,
                            end: text_before.len(),
                            replacement: state.current_text.clone(),
                        });
                    }
                }
            }
        }
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        PipelineBuilder::new()
            .add_rule(Box::new(crate::formatting::punctuation::PunctuationRule))
            .add_rule(Box::new(crate::formatting::markdown::MarkdownRule))
            .add_rule(Box::new(crate::formatting::developer::DeveloperRule))
            .add_rule(Box::new(crate::formatting::numbers::NumbersRule))
            .add_rule(Box::new(crate::formatting::vocabulary::VocabularyRule))
            .add_rule(Box::new(crate::formatting::email_url::EmailUrlRule))
            .add_rule(Box::new(crate::formatting::lists::ListsRule))
            .add_rule(Box::new(crate::formatting::spacing::SpacingRule))
            .add_rule(Box::new(crate::formatting::capitalization::CapitalizationRule))
            .add_rule(Box::new(crate::formatting::cleanup::CleanupRule))
            .add_pass(RuleCategory::Lexical)
            .add_pass(RuleCategory::Normalization)
            .add_pass(RuleCategory::Developer)
            .add_pass(RuleCategory::Formatting)
            .add_pass(RuleCategory::Cleanup)
            .build()
    }
}
