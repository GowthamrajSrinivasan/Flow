#[derive(Debug, Clone, PartialEq)]
pub enum RewriteOperation {
    InlineCorrection { before: String, after: String },
    FalseStartRecovery { before: String, after: String },
    UndoCommand { before: String, after: String },
    ReplaceCommand { before: String, after: String },
    DeleteCommand { before: String, after: String },
    DeleteLastSentence { before: String, after: String },
    ReplaceLastWord { before: String, after: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PendingCorrection {
    pub trigger: String,
    pub target_span: (usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    Time,
    Date,
    Number,
    Phone,
    Email,
    Location,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CorrectionConfidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct RewriteContext {
    pub pending_correction: Option<PendingCorrection>,
    pub history: Vec<RewriteOperation>,
}

impl RewriteContext {
    pub fn new() -> Self {
        Self {
            pending_correction: None,
            history: Vec::new(),
        }
    }
}

impl Default for RewriteContext {
    fn default() -> Self {
        Self::new()
    }
}
