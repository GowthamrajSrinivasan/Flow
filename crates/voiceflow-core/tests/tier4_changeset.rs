use voiceflow_core::formatting::streaming::{MergeEngine, WindowBuilder, WindowPolicy};
use voiceflow_core::formatting::regions::{RegionClassifier, RegionType};
use voiceflow_core::pipeline::changes::{Change, ChangeKind, ChangeSet, TextRange, ChangeSource, Confidence};

#[test]
fn test_changeset_replace() {
    let engine = MergeEngine::new();
    
    let historic_text = "Let's meet on monday";
    let mut changeset = ChangeSet::new();
    
    // Simulating the formatter turning "monday" into "Monday"
    changeset.add(Change {
        id: 0,
        kind: ChangeKind::Replace {
            replacement: "Monday".to_string(),
        },
        range: TextRange {
            start: 14,
            end: 20,
        },
        source: ChangeSource::Rule("CapitalizationRule".to_string()),
        confidence: Confidence::Certain,
    });
    
    let result = engine.apply_changeset(historic_text, &changeset);
    assert_eq!(result, "Let's meet on Monday");
}

#[test]
fn test_changeset_multiple_edits() {
    let engine = MergeEngine::new();
    
    let historic_text = "Check out github dot com";
    let mut changeset = ChangeSet::new();
    
    // Turn "github dot com" into "github.com"
    changeset.add(Change {
        id: 0,
        kind: ChangeKind::Replace {
            replacement: "github.com".to_string(),
        },
        range: TextRange {
            start: 10,
            end: 24,
        },
        source: ChangeSource::Rule("UrlRule".to_string()),
        confidence: Confidence::Certain,
    });
    
    // Add " slash pricing" -> "/pricing"
    changeset.add(Change {
        id: 1,
        kind: ChangeKind::Insert {
            text: "/pricing".to_string(),
        },
        range: TextRange {
            start: 24,
            end: 24,
        },
        source: ChangeSource::Rule("UrlRule".to_string()),
        confidence: Confidence::Certain,
    });
    
    let result = engine.apply_changeset(historic_text, &changeset);
    assert_eq!(result, "Check out github.com/pricing");
}

#[test]
fn test_region_classifier() {
    let classifier = RegionClassifier::new();
    
    let text = "Navigate to github.com and then log in.";
    
    // Pretend github.com was previously formatted and is protected
    let protected = vec![(12, 22)];
    
    let regions = classifier.classify(text, &protected);
    
    assert_eq!(regions.len(), 3);
    assert_eq!(regions[0].region_type, RegionType::Stable);
    assert_eq!(regions[0].start, 0);
    assert_eq!(regions[0].end, 12);
    
    assert_eq!(regions[1].region_type, RegionType::Protected);
    assert_eq!(regions[1].start, 12);
    assert_eq!(regions[1].end, 22);
    
    assert_eq!(regions[2].region_type, RegionType::Stable);
    assert_eq!(regions[2].start, 22);
    assert_eq!(regions[2].end, text.len());
}
