use crate::pipeline::request::RewriteMode;
use crate::pipeline::intent::UserIntent;
use crate::profile::RuntimeProfile;

use super::runtime::LlmRuntime;
use super::prompts::build_system_prompt;
use super::safety::strip_filler;

pub fn rewrite(text: &str, intent: UserIntent, mode: RewriteMode, profile: &dyn RuntimeProfile) -> String {
    if mode == RewriteMode::Off || intent == UserIntent::Dictation {
        return text.to_string();
    }
    
    let runtime = LlmRuntime::new();
    let system_prompt = build_system_prompt(intent, mode);
    
    let raw_output = runtime.run_inference(text, &system_prompt, profile);
    
    strip_filler(&raw_output)
}
