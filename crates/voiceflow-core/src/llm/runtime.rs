use crate::profile::RuntimeProfile;
use super::provider::LlamaProvider;

pub struct LlmRuntime {
    provider: LlamaProvider,
}

impl LlmRuntime {
    pub fn new() -> Self {
        Self {
            provider: LlamaProvider::new(),
        }
    }
    
    pub fn run_inference(&self, text: &str, system_prompt: &str, profile: &dyn RuntimeProfile) -> String {
        self.provider.generate(text, system_prompt, profile.rewrite_limit_tokens(), text)
    }
}
