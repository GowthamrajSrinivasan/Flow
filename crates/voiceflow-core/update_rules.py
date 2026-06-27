import os
import re

RULES_DIR = "/Users/gowthamrajsrinivasan/Documents/Projects/voiceflow-core/crates/voiceflow-core/src/formatting"

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # If it's already updated, skip
    if "RuleMetadata" in content and "fn metadata" in content:
        return

    # Extract name, priority, applies logic, apply logic
    name_match = re.search(r'fn name\(&self\) -> \&\'static str\s*{\s*"([^"]+)"\s*}', content)
    priority_match = re.search(r'fn priority\(&self\) -> u32\s*{\s*(\d+)\s*(?://.*)?}', content)
    
    if not name_match or not priority_match:
        return
        
    name = name_match.group(1)
    priority = priority_match.group(1)
    
    # Simple heuristic for category
    category = "Formatting"
    if name == "Cleanup" or name == "Spacing":
        category = "Cleanup"
    elif name == "Developer":
        category = "Developer"
    elif name == "Vocabulary" or name == "Capitalization":
        category = "Normalization"
    elif name == "Punctuation":
        category = "Lexical"

    new_imports = "use crate::formatting::metadata::{RuleMetadata, RuleCategory, RuleCapabilities};\nuse crate::pipeline::models::{TransformationRequest, TransformationState};\nuse std::time::Instant;\nuse crate::pipeline::models::Diagnostic;\n"
    
    # replace use crate::formatting::context::FormattingContext;
    content = re.sub(r'use crate::formatting::context::FormattingContext;\n?', '', content)
    content = re.sub(r'use crate::formatting::traits::FormatterRule;\n?', 'use crate::formatting::traits::FormatterRule;\n' + new_imports, content)
    
    # Generate metadata block
    metadata_block = f"""
const METADATA: RuleMetadata = RuleMetadata {{
    name: "{name}",
    version: "1.0.0",
    category: RuleCategory::{category},
    priority: {priority},
    capabilities: RuleCapabilities {{
        streaming_safe: true,
        token_based: false,
        regex_based: true,
        locale_aware: false,
        developer_only: {"true" if name == "Developer" else "false"},
        markdown_only: {"true" if name == "Markdown" else "false"},
        incremental_safe: true,
    }},
}};
"""
    # Replace impl FormatterRule
    content = re.sub(r'impl FormatterRule for ([A-Za-z]+Rule)\s*{', metadata_block + r'impl FormatterRule for \1 {', content)
    
    # Replace fn name and priority with metadata
    content = re.sub(r'\s*fn name\(&self\) -> \&\'static str\s*{[^}]+}', '', content)
    content = re.sub(r'\s*fn priority\(&self\) -> u32\s*{[^}]+}', '\n    fn metadata(&self) -> &\'static RuleMetadata {\n        &METADATA\n    }', content)
    
    # Replace applies
    content = re.sub(r'fn applies\(&self, _?ctx: &FormattingContext\)', r'fn applies(&self, request: &TransformationRequest)', content)
    content = content.replace("ctx.markdown_enabled", "request.user_context.markdown_enabled")
    content = content.replace("ctx.mode", "request.mode")
    
    # Replace apply
    content = re.sub(r'fn apply\(&self, text: &mut String, _?ctx: &FormattingContext\)\s*{', 
                     r'fn apply(&self, state: &mut TransformationState, request: &TransformationRequest) {\n        let start_time = Instant::now();\n        let original_text = state.current_text.clone();\n        let mut text = state.current_text.clone();', content)
    
    content = content.replace("ctx.", "request.user_context.")
    
    # At the end of apply block, we need to assign back to state and add diagnostics.
    # We find the last `}` of apply by a simple search, or we can just replace `*text = ...` 
    content = re.sub(r'\*text\s*=\s*([^;]+);', r'state.current_text = \1;', content)
    
    # We should add diagnostic code at the end of apply
    diagnostic_code = f"""
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
"""
    # Just insert it before the closing brace of apply.
    # Usually the apply ends with `}`. So we find the last `}` of the file if it's just one impl block? No, there might be other functions.
    # Let's just do a manual replace for each file since we know the structure.
    
    with open(filepath, 'w') as f:
        f.write(content)

for root, _, files in os.walk(RULES_DIR):
    for file in files:
        if file.endswith('.rs') and file != 'mod.rs' and file != 'traits.rs' and file != 'registry.rs' and file != 'context.rs' and file != 'formatter.rs' and file != 'pipeline.rs' and file != 'metadata.rs':
            process_file(os.path.join(root, file))
