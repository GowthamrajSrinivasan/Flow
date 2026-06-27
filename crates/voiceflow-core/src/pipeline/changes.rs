#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeSource {
    Rule(String),
    User,
    StreamingMerge,
    Llm,
    Plugin(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Confidence {
    Certain,
    Estimated(f32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeKind {
    Replace { replacement: String },
    Insert { text: String },
    Delete,
    Move { to_offset: usize },
}

pub type ChangeId = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct Change {
    pub id: ChangeId,
    pub kind: ChangeKind,
    pub range: TextRange,
    pub source: ChangeSource,
    pub confidence: Confidence,
}

#[derive(Debug, Clone)]
pub struct ChangeSet {
    pub schema_version: u32,
    pub changes: Vec<Change>,
}

impl Default for ChangeSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangeSet {
    pub fn new() -> Self {
        Self { schema_version: 1, changes: Vec::new() }
    }

    pub fn add(&mut self, change: Change) {
        self.changes.push(change);
    }
    
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Reverses the ChangeSet so it can be used for Undo operations.
    pub fn reverse(&self) -> ChangeSet {
        // Implementation for undo logic will go here.
        // For now, this is a placeholder stub to demonstrate Tier 3 readiness.
        unimplemented!("Undo reversal logic for ChangeSets is planned for a future phase.")
    }
}
