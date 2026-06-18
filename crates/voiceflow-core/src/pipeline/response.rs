use super::intent::UserIntent;

#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub raw_text: String,
    pub formatted_text: String,
    pub final_text: String,
    pub intent: UserIntent,
    pub used_llm: bool,
    pub processing_time_ms: u64,
}
