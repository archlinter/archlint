mod common;

use common::analyze_fixture;
use archlint::detectors::side_effect_import::SideEffectImportDetector;
use archlint::detectors::Detector;

#[test]
fn test_side_effect_import_detected() {
    let ctx = analyze_fixture("side_effect/basic");
    let detector = SideEffectImportDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect side-effect import");
    assert_eq!(smells.len(), 1);

    let smell = &smells[0];
    assert!(smell.locations.iter().any(|l| l.description.contains("Side-effect import of './init'")));
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
