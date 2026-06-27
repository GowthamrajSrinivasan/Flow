use std::collections::HashMap;
use crate::formatting::context::{AppContext, DocumentContext, FormattingProfile, UserContext};
use crate::pipeline::request::FormattingMode;
use crate::pipeline::changes::ChangeSet;

#[derive(Debug, Clone)]
pub struct TransformationRequest {
    pub input: String,
    pub previous_output: Option<String>,
    pub cursor_position: Option<usize>,
    pub selection: Option<(usize, usize)>,
    pub mode: FormattingMode,
    pub profile: FormattingProfile,
    pub app_context: AppContext,
    pub user_context: UserContext,
    pub document_context: DocumentContext,
    pub metadata: HashMap<String, String>,
}

impl TransformationRequest {
    pub fn new(input: String) -> Self {
        Self {
            input,
            previous_output: None,
            cursor_position: None,
            selection: None,
            mode: FormattingMode::Smart,
            profile: FormattingProfile::General,
            app_context: AppContext::default(),
            user_context: UserContext::default(),
            document_context: DocumentContext::default(),
            metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub rule: &'static str,
    pub severity: String,
    pub before: String,
    pub after: String,
    pub duration_ms: u128,
}

#[derive(Debug, Clone)]
pub struct TransformationState {
    pub current_text: String,
    
    // Streaming context
    pub previous_window: Option<String>,
    pub current_window: Option<String>,
    pub window_start: Option<usize>,
    pub window_end: Option<usize>,

    pub changes: ChangeSet,
    pub diagnostics: Vec<Diagnostic>,
}

impl TransformationState {
    pub fn new(initial_text: String) -> Self {
        Self {
            current_text: initial_text,
            previous_window: None,
            current_window: None,
            window_start: None,
            window_end: None,
            changes: ChangeSet::new(),
            diagnostics: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransformationMetrics {
    pub duration_ms: u128,
    pub rules_executed: usize,
    pub characters_in: usize,
    pub characters_out: usize,
    pub allocations: usize,
}

#[derive(Debug, Clone)]
pub struct TransformationResult {
    pub output: String,
    pub changes: ChangeSet,
    pub diagnostics: Vec<Diagnostic>,
    pub metrics: TransformationMetrics,
}

impl TransformationResult {
    pub fn new(output: String, changes: ChangeSet, diagnostics: Vec<Diagnostic>, metrics: TransformationMetrics) -> Self {
        Self {
            output,
            changes,
            diagnostics,
            metrics,
        }
    }
}
