use crate::formatting::context::FormattingContext;
use crate::formatting::pipeline;
use crate::formatting::registry::RuleRegistry;

use crate::pipeline::models::TransformationRequest;
use crate::formatting::context::{AppContext, UserContext, DocumentContext, FormattingProfile};

pub fn format(text: &str, ctx: &FormattingContext) -> String {
    let registry = RuleRegistry::default();
    let pipeline = pipeline::TransformationPipeline::new(registry);
    
    let mut request = TransformationRequest::new(text.to_string());
    request.mode = ctx.mode.clone();
    request.user_context.language = ctx.language.clone();
    request.app_context.locale = ctx.locale.clone();
    request.user_context.markdown_enabled = ctx.markdown_enabled;
    request.user_context.vocabulary = ctx.vocabulary.clone();
    
    let result = pipeline.transform(&request);
    result.output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier4_pipeline_integration() {
        let input = "hello comma here is my email john dot doe at gmail dot com period here are the items we need colon apples comma bananas comma and oranges period please convert to bullet list";
        
        let expected = "Hello, here is my email john.doe@gmail.com. Here are the items we need:\n- apples\n- bananas\n- oranges.";
        
        let context = FormattingContext::default();
        let formatted = format(input, &context);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_tier4_url_and_punctuation() {
        let input = "go to open ai dot com slash pricing today comma and check the plans period";
        let expected = "Go to openai.com/pricing today, and check the plans.";
        let context = FormattingContext::default();
        let formatted = format(input, &context);
        assert_eq!(formatted, expected);
    }
}
