mod common;

use common::analyze_fixture;
use archlint::detectors::orphan_types::OrphanTypesDetector;
use archlint::detectors::Detector;

#[test]
fn test_unused_type_detected() {
    let ctx = analyze_fixture("orphan/basic");
    let detector = OrphanTypesDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect unused type");
    assert!(smells.iter().any(|s| {
        if let archlint::detectors::SmellType::OrphanType { name } = &s.smell_type {
            name == "UnusedType"
        } else {
            false
        }
    }));
}

#[test]
fn test_used_interface_ok() {
    let ctx = analyze_fixture("orphan/used");
    let detector = OrphanTypesDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected no orphan types for used interface");
}
