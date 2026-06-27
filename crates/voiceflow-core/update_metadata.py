import os
import glob
import re

search_path = '/Users/gowthamrajsrinivasan/Documents/Projects/voiceflow-core/crates/voiceflow-core/src/formatting/**/*.rs'
files = glob.glob(search_path, recursive=True)

for file in files:
    if not os.path.isfile(file):
        continue
    with open(file, 'r') as f:
        content = f.read()
    
    if 'const METADATA: RuleMetadata = RuleMetadata {' in content:
        if 'id: RuleId(' in content:
            continue # already updated
        
        # Add import if missing
        if 'RuleId' not in content:
            content = re.sub(r'(use crate::formatting::metadata::.*?RuleMetadata)', r'\1, RuleId', content)
            if 'RuleId' not in content:
                 content = re.sub(r'(use crate::formatting::metadata::.*?RuleCategory)', r'\1, RuleId', content)
            if 'RuleId' not in content:
                 content = content.replace('use crate::formatting::metadata::{RuleCategory, RuleMetadata};', 'use crate::formatting::metadata::{RuleCategory, RuleMetadata, RuleId};')
            if 'RuleId' not in content:
                 content = content.replace('use crate::formatting::metadata::{RuleCapabilities, RuleCategory, RuleMetadata};', 'use crate::formatting::metadata::{RuleCapabilities, RuleCategory, RuleMetadata, RuleId};')
            
        
        # Extract rule name to use as RuleId
        name_match = re.search(r'name:\s*"([^"]+)"', content)
        if name_match:
            rule_name = name_match.group(1)
            new_metadata = f'''const METADATA: RuleMetadata = RuleMetadata {{
    id: RuleId("{rule_name}"),
    name: "{rule_name}",'''
            content = re.sub(r'const METADATA: RuleMetadata = RuleMetadata \{\n\s*name:\s*"[^"]+",', new_metadata, content)
            
            # Add depends_on
            # Make sure we don't add double commas
            content = re.sub(r'capabilities:(.*?)(,?\s*)\n\};', r'capabilities:\1,\n    depends_on: &[],\n};', content, flags=re.DOTALL)
            
            with open(file, 'w') as f:
                f.write(content)
            print(f"Updated {file}")
