mod common;

use archlint::detectors::high_coupling::HighCouplingDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_rule;

#[test]
fn test_high_cbo_detected() {
    let ctx = analyze_fixture_with_rule("coupling/high", "high_coupling", Some("max_cbo: 20"));
    let detector = HighCouplingDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect high coupling");
    assert!(smells
        .iter()
        .any(|s| s.files.iter().any(|f| f.ends_with("target.ts"))));
}

#[test]
fn test_normal_coupling_ok() {
    let ctx = common::analyze_fixture("coupling/normal"); // empty directory = no dependencies
    let detector = HighCouplingDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected no coupling smells for isolated modules"
    );
}
