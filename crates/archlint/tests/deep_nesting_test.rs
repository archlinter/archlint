mod common;

use common::analyze_fixture;
use archlint::detectors::deep_nesting::DeepNestingDetector;
use archlint::detectors::Detector;

#[test]
fn test_deep_nesting_detected() {
    let ctx = analyze_fixture("nesting/deep");
    let detector = DeepNestingDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect deep nesting");
    assert!(smells.iter().any(|s| {
        if let archlint::detectors::SmellType::DeepNesting { depth } = &s.smell_type {
            *depth >= 4
        } else {
            false
        }
    }));
}

#[test]
fn test_normal_nesting_ok() {
    let ctx = analyze_fixture("nesting/normal");
    let detector = DeepNestingDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected normal nesting to be ok");
}
