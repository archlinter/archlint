mod common;

use archlint::detectors::shared_mutable_state::SharedMutableStateDetector;
use archlint::detectors::Detector;
use common::analyze_fixture;

#[test]
fn test_export_let_detected() {
    let ctx = analyze_fixture("mutable/let");
    let detector = SharedMutableStateDetector;
    let smells = detector.detect(&ctx);

    assert!(
        !smells.is_empty(),
        "Expected to detect shared mutable state"
    );
    assert_eq!(smells.len(), 2);
    assert!(smells.iter().any(|s| {
        if let archlint::detectors::SmellType::SharedMutableState { symbol } = &s.smell_type {
            symbol == "count"
        } else {
            false
        }
    }));
}

#[test]
fn test_const_primitive_ok() {
    let ctx = analyze_fixture("mutable/const");
    let detector = SharedMutableStateDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected const exports to be ok");
}
