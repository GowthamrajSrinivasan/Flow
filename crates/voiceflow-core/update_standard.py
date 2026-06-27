import os

RULES = [
    ("capitalization", "CapitalizationRule", "Formatting", 200, "capitalize"),
    ("cleanup", "CleanupRule", "Cleanup", 100, "cleanup"),
    ("email_url", "EmailUrlRule", "Formatting", 700, "format"),
    ("lists", "ListsRule", "Formatting", 600, "format"),
    ("punctuation", "PunctuationRule", "Lexical", 1000, "convert"),
    ("spacing", "SpacingRule", "Cleanup", 500, "fix_spacing"),
]

for folder, rule_name, category, priority, func_name in RULES:
    filepath = f"src/formatting/{folder}/mod.rs"
    if folder == "email_url":
        filepath = f"src/formatting/email_url.rs"
        
    with open(filepath, "r") as f:
        content = f.read()
    
    parts = content.split(f"pub fn {func_name}")
    assert len(parts) == 2, f"Could not split {filepath}"
    
    new_imports_and_impl = f"""use regex::Regex;
use crate::formatting::metadata::{{RuleMetadata, RuleCategory, RuleCapabilities}};
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
        developer_only: false,
        markdown_only: false,
        incremental_safe: true,
    }},
}};

pub struct {rule_name};

impl crate::formatting::traits::FormatterRule for {rule_name} {{
    fn metadata(&self) -> &'static RuleMetadata {{
        &METADATA
    }}

    fn applies(&self, _request: &TransformationRequest) -> bool {{
        true
    }}

    fn apply(&self, state: &mut TransformationState, _request: &TransformationRequest) {{
        let start_time = Instant::now();
        let original_text = state.current_text.clone();
        
        state.current_text = {func_name}(&state.current_text);
        
        let duration = start_time.elapsed().as_millis();
        if original_text != state.current_text {{
            state.diagnostics.push(Diagnostic {{
                rule: METADATA.name,
                severity: "info".to_string(),
                before: original_text,
                after: state.current_text.clone(),
                duration_ms: duration,
            }});
        }}
    }}
}}

pub fn {func_name}""" + parts[1]

    with open(filepath, "w") as f:
        f.write(new_imports_and_impl)

print("Standard rules updated.")
