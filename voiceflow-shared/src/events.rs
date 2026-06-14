use serde::{Deserialize, Serialize};
use crate::metrics::SessionMetrics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoiceFlowEvent {
    ListeningStarted,
    ListeningStopped,
    SpeechDetected,
    SpeechEnded,
    TranscriptionPartial(String),
    TranscriptionFinal(String),
    PartialTranscript(String),
    FinalTranscript(String),
    InjectionComplete,
    MetricsUpdated(SessionMetrics),
    Error(String),
}
