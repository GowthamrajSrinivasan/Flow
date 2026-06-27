#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change {
    /// Replaces the text between `start` and `end` byte indices with `replacement`.
    Replace {
        start: usize,
        end: usize,
        replacement: String,
    },
    /// Inserts `text` at the given `offset`.
    Insert {
        offset: usize,
        text: String,
    },
    /// Deletes the text between `start` and `end` byte indices.
    Delete {
        start: usize,
        end: usize,
    },
    /// Moves text from one region to another.
    Move {
        from_start: usize,
        from_end: usize,
        to_offset: usize,
    },
}

#[derive(Debug, Clone, Default)]
pub struct ChangeSet {
    pub changes: Vec<Change>,
}

impl ChangeSet {
    pub fn new() -> Self {
        Self { changes: Vec::new() }
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
