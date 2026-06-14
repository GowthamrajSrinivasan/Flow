#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranscriptState {
    Idle,
    Listening,
    Transcribing,
    Formatting,
    Injecting,
    Complete,
}

pub struct TranscriptStateMachine {
    pub state: TranscriptState,
}

impl TranscriptStateMachine {
    pub fn new() -> Self {
        Self {
            state: TranscriptState::Idle,
        }
    }

    pub fn transition_to(&mut self, new_state: TranscriptState) {
        // Here we could add logic to ensure valid state transitions.
        // For example, Idle -> Listening -> Transcribing -> Formatting -> Injecting -> Complete -> Idle
        self.state = new_state;
    }
}
