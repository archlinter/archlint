mod common;

use common::analyze_fixture;
use archlint::detectors::cycles::CycleDetector;
use archlint::detectors::Detector;

#[test]
fn test_simple_cycle_detected() {
    let ctx = analyze_fixture("cycles/simple_cycle");
    let detector = CycleDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect at least one cycle");

    let has_cycle = smells.iter().any(|s| {
        s.files.iter().any(|f| f.ends_with("a.ts")) && s.files.iter().any(|f| f.ends_with("b.ts"))
    });

    assert!(has_cycle, "Expected to find cycle between a.ts and b.ts");
}

#[test]
fn test_no_cycle_detected() {
    let ctx = analyze_fixture("cycles/no_cycle");
    let detector = CycleDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected no cycles to be detected");
}
