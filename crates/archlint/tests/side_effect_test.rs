mod common;

use archlint::detectors::side_effect_import::SideEffectImportDetector;
use archlint::detectors::Detector;
use common::analyze_fixture;

#[test]
fn test_side_effect_import_detected() {
    let ctx = analyze_fixture("side_effect/basic");
    let detector = SideEffectImportDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect side-effect import");
    assert_eq!(smells.len(), 1);

    let smell = &smells[0];
    assert!(smell
        .locations
        .iter()
        .any(|l| l.description.contains("Side-effect import of './init'")));
}

#[test]
fn test_css_import_ok() {
    let ctx = analyze_fixture("side_effect/css");
    let detector = SideEffectImportDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected CSS import to be ignored");
}

#[test]
fn test_normal_import_ok() {
    let ctx = analyze_fixture("side_effect/normal");
    let detector = SideEffectImportDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected normal named import to be ok");
}

/// Test for issue #23: dynamic import() should NOT be flagged as side-effect import
/// Vue Router / React lazy loading pattern
#[test]
fn test_dynamic_import_ok() {
    let ctx = analyze_fixture("side_effect/dynamic");
    let detector = SideEffectImportDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Dynamic import() should not be flagged as side-effect. Found: {:?}",
        smells.iter().map(|s| &s.locations).collect::<Vec<_>>()
    );
}
