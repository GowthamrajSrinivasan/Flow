use crate::pipeline::changes::ChangeId;
use crate::formatting::regions::RegionType;

#[derive(Debug, Clone)]
pub enum PipelineEvent {
    TransformationStarted,
    TransformationFinished,
    WindowBuilt,
    WindowExpanded,
    Classification,
    RuleStarted { rule: String },
    RuleSkipped { rule: String },
    RuleFinished { rule: String, duration_micros: u64 },
    ValidationStarted,
    ValidationFinished,
    MergeStarted,
    MergeFinished,
    RegionProtected { start: usize, end: usize },
    RegionLocked { start: usize, end: usize },
    ChangeEmitted { change_id: ChangeId },
    ChangeApplied { change_id: ChangeId },
}

pub trait EventObserver: Send + Sync {
    fn on_event(&mut self, event: &PipelineEvent);
}

pub struct EventBus {
    observers: Vec<Box<dyn EventObserver>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self { observers: Vec::new() }
    }

    pub fn add_observer(&mut self, observer: Box<dyn EventObserver>) {
        self.observers.push(observer);
    }

    pub fn dispatch(&mut self, event: &PipelineEvent) {
        for observer in &mut self.observers {
            observer.on_event(event);
        }
    }
}

// Example Implementations
#[derive(Default, Debug)]
pub struct MetricsCollector {
    pub rules_executed: u64,
    pub rules_skipped: u64,
    pub changes_emitted: u64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self::default()
    }
}

impl EventObserver for MetricsCollector {
    fn on_event(&mut self, event: &PipelineEvent) {
        match event {
            PipelineEvent::RuleStarted { .. } => self.rules_executed += 1,
            PipelineEvent::RuleSkipped { .. } => self.rules_skipped += 1,
            PipelineEvent::ChangeEmitted { .. } => self.changes_emitted += 1,
            _ => {}
        }
    }
}

#[derive(Default, Debug)]
pub struct DiagnosticsCollector {
    pub events: Vec<PipelineEvent>,
}

impl DiagnosticsCollector {
    pub fn new() -> Self {
        Self::default()
    }
}

impl EventObserver for DiagnosticsCollector {
    fn on_event(&mut self, event: &PipelineEvent) {
        self.events.push(event.clone());
    }
}
