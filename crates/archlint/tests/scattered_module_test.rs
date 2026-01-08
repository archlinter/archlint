mod common;

use archlint::detectors::scattered_module::ScatteredModuleDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_rule;

#[test]
fn test_scattered_utils_detected() {
    let ctx = analyze_fixture_with_rule(
        "cohesion/scattered",
        "module_cohesion",
        Some("max_components: 2\nmin_exports: 5"),
    );
    let detector = ScatteredModuleDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect scattered module");
    assert!(smells
        .iter()
        .any(|s| s.files.iter().any(|f| f.ends_with("utils.ts"))));
}

#[test]
fn test_focused_module_ok() {
    let ctx = analyze_fixture_with_rule(
        "cohesion/focused",
        "module_cohesion",
        Some("max_components: 2"),
    );
    let detector = ScatteredModuleDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected focused module to be ok");
}
