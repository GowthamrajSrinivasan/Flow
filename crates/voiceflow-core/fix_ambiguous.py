import os

registry_path = 'src/formatting/registry.rs'
with open(registry_path, 'r') as f:
    content = f.read()

content = content.replace('''state.changes.add(crate::pipeline::changes::Change::Replace {
                            start: 0,
                            end: text_before.len(),
                            replacement: state.current_text.clone(),
                        });''', '''state.changes.add(crate::pipeline::changes::Change {
                            id: state.changes.changes.len(),
                            kind: crate::pipeline::changes::ChangeKind::Replace {
                                replacement: state.current_text.clone(),
                            },
                            range: crate::pipeline::changes::TextRange {
                                start: 0,
                                end: text_before.len(),
                            },
                            source: crate::pipeline::changes::ChangeSource::Rule(rule.metadata().name.to_string()),
                            confidence: crate::pipeline::changes::Confidence::Certain,
                        });''')
with open(registry_path, 'w') as f:
    f.write(content)

streaming_path = 'src/formatting/streaming.rs'
with open(streaming_path, 'r') as f:
    content = f.read()

content = content.replace('use crate::pipeline::changes::Change;', 'use crate::pipeline::changes::{Change, ChangeKind};')

old_match = '''match change {
                Change::Replace { start, end, replacement } => {
                    let s = (*start as isize + offset_shift).max(0) as usize;
                    let e = (*end as isize + offset_shift).max(0) as usize;
                    
                    if s <= result.len() && e <= result.len() && s <= e {
                        let original_len = e - s;
                        let new_len = replacement.len();
                        result.replace_range(s..e, replacement);
                        offset_shift += new_len as isize - original_len as isize;
                    }
                }
                Change::Insert { offset, text: ins_text } => {
                    let o = (*offset as isize + offset_shift).max(0) as usize;
                    if o <= result.len() {
                        result.insert_str(o, ins_text);
                        offset_shift += ins_text.len() as isize;
                    }
                }
                Change::Delete { start, end } => {
                    let s = (*start as isize + offset_shift).max(0) as usize;
                    let e = (*end as isize + offset_shift).max(0) as usize;
                    
                    if s <= result.len() && e <= result.len() && s <= e {
                        result.replace_range(s..e, "");
                        offset_shift -= (e - s) as isize;
                    }
                }
                Change::Move { from_start, from_end, to_offset } => {
                    let fs = (*from_start as isize + offset_shift).max(0) as usize;
                    let fe = (*from_end as isize + offset_shift).max(0) as usize;
                    
                    if fs <= result.len() && fe <= result.len() && fs <= fe {
                        let text_to_move = result[fs..fe].to_string();
                        result.replace_range(fs..fe, "");
                        offset_shift -= (fe - fs) as isize;
                        
                        let to = (*to_offset as isize + offset_shift).max(0) as usize;
                        if to <= result.len() {
                            result.insert_str(to, &text_to_move);
                            offset_shift += text_to_move.len() as isize;
                        }
                    }
                }
            }'''

new_match = '''match &change.kind {
                ChangeKind::Replace { replacement } => {
                    let s = (change.range.start as isize + offset_shift).max(0) as usize;
                    let e = (change.range.end as isize + offset_shift).max(0) as usize;
                    
                    if s <= result.len() && e <= result.len() && s <= e {
                        let original_len = e - s;
                        let new_len = replacement.len();
                        result.replace_range(s..e, replacement);
                        offset_shift += new_len as isize - original_len as isize;
                    }
                }
                ChangeKind::Insert { text: ins_text } => {
                    let o = (change.range.start as isize + offset_shift).max(0) as usize;
                    if o <= result.len() {
                        result.insert_str(o, ins_text);
                        offset_shift += ins_text.len() as isize;
                    }
                }
                ChangeKind::Delete => {
                    let s = (change.range.start as isize + offset_shift).max(0) as usize;
                    let e = (change.range.end as isize + offset_shift).max(0) as usize;
                    
                    if s <= result.len() && e <= result.len() && s <= e {
                        result.replace_range(s..e, "");
                        offset_shift -= (e - s) as isize;
                    }
                }
                ChangeKind::Move { to_offset } => {
                    let fs = (change.range.start as isize + offset_shift).max(0) as usize;
                    let fe = (change.range.end as isize + offset_shift).max(0) as usize;
                    
                    if fs <= result.len() && fe <= result.len() && fs <= fe {
                        let text_to_move = result[fs..fe].to_string();
                        result.replace_range(fs..fe, "");
                        offset_shift -= (fe - fs) as isize;
                        
                        let to = (*to_offset as isize + offset_shift).max(0) as usize;
                        if to <= result.len() {
                            result.insert_str(to, &text_to_move);
                            offset_shift += text_to_move.len() as isize;
                        }
                    }
                }
            }'''

content = content.replace(old_match, new_match)
with open(streaming_path, 'w') as f:
    f.write(content)
