use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetrics {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub audio_capture_latency_ms: Option<u64>,
    pub vad_latency_ms: Option<u64>,
    pub stt_latency_ms: Option<u64>,
    pub formatting_latency_ms: Option<u64>,
    pub injection_latency_ms: Option<u64>,
    pub model_load_ms: Option<u64>,
    pub first_token_ms: Option<u64>,
    pub first_partial_ms: Option<u64>,
    pub final_transcript_ms: Option<u64>,
    pub memory_usage_mb: Option<u64>,
    pub cpu_usage_percent: Option<f32>,
    pub model_size_mb: Option<u64>,
    pub download_size_mb: Option<u64>,
}

impl SessionMetrics {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            start_time: Utc::now(),
            end_time: None,
            audio_capture_latency_ms: None,
            vad_latency_ms: None,
            stt_latency_ms: None,
            formatting_latency_ms: None,
            injection_latency_ms: None,
            model_load_ms: None,
            first_token_ms: None,
            first_partial_ms: None,
            final_transcript_ms: None,
            memory_usage_mb: None,
            cpu_usage_percent: None,
            model_size_mb: None,
            download_size_mb: None,
        }
    }

    pub fn total_latency_ms(&self) -> u64 {
        self.audio_capture_latency_ms.unwrap_or(0)
            + self.vad_latency_ms.unwrap_or(0)
            + self.stt_latency_ms.unwrap_or(0)
            + self.formatting_latency_ms.unwrap_or(0)
            + self.injection_latency_ms.unwrap_or(0)
    }

    pub fn complete(&mut self) {
        self.end_time = Some(Utc::now());
    }
}
