import os
import re

RULES = [
    ("markdown", "MarkdownRule", "Formatting", 850, "ctx.markdown_enabled || ctx.mode == crate::pipeline::request::FormattingMode::Markdown"),
    ("developer", "DeveloperRule", "Developer", 870, "ctx.mode == crate::pipeline::request::FormattingMode::Developer || ctx.language == \"rust\" || ctx.language == \"python\""),
    ("numbers", "NumbersRule", "Formatting", 820, "true"),
]

for folder, rule_name, category, priority, applies_expr in RULES:
    filepath = f"src/formatting/{folder}/mod.rs"
    with open(filepath, "r") as f:
        content = f.read()
    
    # We want to replace the `impl FormatterRule for ...` block and the imports.
    # The apply method contains the logic directly, so we just capture the body of apply.
    apply_match = re.search(r'fn apply\(&self, text: &mut String, _ctx: &FormattingContext\)\s*{([^}]+)}', content)
    
    # Because there might be multiple blocks, let's use a simpler approach.
    # Just replace the top part and adjust `apply` and `applies`.
    # Wait, the apply body for developer rule is huge and contains nested braces! Regex won't easily work.
    
    # We can just do simple string replacements!
    
    new_imports = f"""use crate::formatting::metadata::{{RuleMetadata, RuleCategory, RuleCapabilities}};
use crate::pipeline::models::{{TransformationRequest, TransformationState}};
use crate::pipeline::models::Diagnostic;
use std::time::Instant;

const METADATA: RuleMetadata = RuleMetadata {{
    name: "{rule_name}",
    version: "1.0.0",
    category: RuleCategory::{category},
    priority: {priority},
    capabilities: RuleCapabilities {{
        streaming_safe: true,
        token_based: false,
        regex_based: true,
        locale_aware: false,
        developer_only: {"true" if rule_name == "DeveloperRule" else "false"},
        markdown_only: {"true" if rule_name == "MarkdownRule" else "false"},
        incremental_safe: true,
    }},
}};
"""
    
    content = content.replace("use crate::formatting::traits::FormatterRule;\nuse crate::formatting::context::FormattingContext;", f"use crate::formatting::traits::FormatterRule;\n{new_imports}")
    
    content = re.sub(r'fn name\(&self\) -> &\'static str\s*{\s*"[^"]+"\s*}', r'fn metadata(&self) -> &\'static RuleMetadata { &METADATA }', content)
    content = re.sub(r'fn priority\(&self\) -> u32\s*{\s*\d+\s*(?://.*)?}', '', content)
    
    applies_new = "request.user_context.markdown_enabled || request.mode == crate::pipeline::request::FormattingMode::Markdown"
    if rule_name == "DeveloperRule":
        applies_new = "request.mode == crate::pipeline::request::FormattingMode::Developer || request.user_context.language == \"rust\" || request.user_context.language == \"python\""
    elif rule_name == "NumbersRule":
        applies_new = "true"
        
    content = re.sub(r'fn applies\(&self, ctx: &FormattingContext\) -> bool\s*{[^}]+}', f'fn applies(&self, request: &TransformationRequest) -> bool {{ {applies_new} }}', content)
    content = re.sub(r'fn applies\(&self, _ctx: &FormattingContext\) -> bool\s*{[^}]+}', f'fn applies(&self, request: &TransformationRequest) -> bool {{ {applies_new} }}', content)
    
    
    content = content.replace("fn apply(&self, text: &mut String, _ctx: &FormattingContext) {", "fn apply(&self, state: &mut TransformationState, _request: &TransformationRequest) {\n        let start_time = Instant::now();\n        let original_text = state.current_text.clone();\n        let mut result = state.current_text.clone();")
    
    content = content.replace("let mut result = text.to_string();", "")
    content = content.replace("*text = result;", "state.current_text = result;\n        let duration = start_time.elapsed().as_millis();\n        if original_text != state.current_text {\n            state.diagnostics.push(Diagnostic {\n                rule: METADATA.name,\n                severity: \"info\".to_string(),\n                before: original_text,\n                after: state.current_text.clone(),\n                duration_ms: duration,\n            });\n        }")
    
    with open(filepath, "w") as f:
        f.write(content)

print("Remaining rules updated.")
