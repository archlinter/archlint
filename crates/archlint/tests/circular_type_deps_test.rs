mod common;

use common::analyze_fixture;
use archlint::detectors::circular_type_deps::CircularTypeDepsDetector;
use archlint::detectors::Detector;

#[test]
fn test_type_cycle_detected() {
    let ctx = analyze_fixture("type_cycles/basic");
    let detector = CircularTypeDepsDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect circular type dependency");
    assert!(smells.iter().any(|s| {
        s.files.iter().any(|f| f.ends_with("a.ts")) &&
        s.files.iter().any(|f| f.ends_with("b.ts"))
    }));
}

#[test]
fn test_mixed_cycle_not_type_only() {
    let ctx = analyze_fixture("type_cycles/mixed");
    let detector = CircularTypeDepsDetector;
    let smells = detector.detect(&ctx);

    // In mixed, a.ts uses normal import, b.ts uses import type.
    // CircularTypeDepsDetector only looks for cycles where ALL edges are type-only.
    assert!(smells.is_empty(), "Expected no type-only cycle for mixed imports");
}
