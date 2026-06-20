#[derive(Debug, Clone)]
pub enum VoiceFlowEvent {
    ListeningStarted,
    ListeningStopped,
    PartialTranscript(String),
    FinalTranscript(String),
    ModelDownloading(u8),
    ModelDownloadComplete,
    Error(String),
    EngineInitializing,
    ModelDownloadStarted,
    ModelLoading,
    EngineReady,
    EngineNotReady,
}
