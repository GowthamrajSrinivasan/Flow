pub trait RuntimeProfile {
    fn stt_model(&self) -> &str;
    fn llm_model(&self) -> &str;
    fn rewrite_limit_tokens(&self) -> usize;
    fn max_context_tokens(&self) -> usize;
    fn target_latency_ms(&self) -> u64;
}

pub struct DesktopProfile {
    pub stt_model: String,
    pub llm_model: String,
    pub rewrite_limit_tokens: usize,
    pub max_context_tokens: usize,
    pub target_latency_ms: u64,
}

impl Default for DesktopProfile {
    fn default() -> Self {
        Self {
            stt_model: "faster-whisper-small".to_string(),
            llm_model: "qwen3-8b-instruct".to_string(),
            rewrite_limit_tokens: 500,
            max_context_tokens: 4096,
            target_latency_ms: 150,
        }
    }
}

impl RuntimeProfile for DesktopProfile {
    fn stt_model(&self) -> &str {
        &self.stt_model
    }
    fn llm_model(&self) -> &str {
        &self.llm_model
    }
    fn rewrite_limit_tokens(&self) -> usize {
        self.rewrite_limit_tokens
    }
    fn max_context_tokens(&self) -> usize {
        self.max_context_tokens
    }
    fn target_latency_ms(&self) -> u64 {
        self.target_latency_ms
    }
}

pub struct MobileProfile {
    pub stt_model: String,
    pub llm_model: String,
    pub rewrite_limit_tokens: usize,
    pub max_context_tokens: usize,
    pub target_latency_ms: u64,
}

impl Default for MobileProfile {
    fn default() -> Self {
        Self {
            stt_model: "whisper-tiny".to_string(),
            llm_model: "qwen3-1.7b".to_string(),
            rewrite_limit_tokens: 250,
            max_context_tokens: 2048,
            target_latency_ms: 100,
        }
    }
}

impl RuntimeProfile for MobileProfile {
    fn stt_model(&self) -> &str {
        &self.stt_model
    }
    fn llm_model(&self) -> &str {
        &self.llm_model
    }
    fn rewrite_limit_tokens(&self) -> usize {
        self.rewrite_limit_tokens
    }
    fn max_context_tokens(&self) -> usize {
        self.max_context_tokens
    }
    fn target_latency_ms(&self) -> u64 {
        self.target_latency_ms
    }
}
