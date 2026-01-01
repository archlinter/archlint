mod common;

use archlint::detectors::scattered_module::ScatteredModuleDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_config;

#[test]
fn test_scattered_utils_detected() {
    let mut config = archlint::config::Config::default();
    config.thresholds.module_cohesion.max_components = 2;
    config.thresholds.module_cohesion.min_exports = 5;

    let ctx = analyze_fixture_with_config("cohesion/scattered", config);
    let detector = ScatteredModuleDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect scattered module");
    assert!(smells
        .iter()
        .any(|s| s.files.iter().any(|f| f.ends_with("utils.ts"))));
}

#[test]
fn test_focused_module_ok() {
    let mut config = archlint::config::Config::default();
    config.thresholds.module_cohesion.max_components = 2;

    let ctx = analyze_fixture_with_config("cohesion/focused", config);
    let detector = ScatteredModuleDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected focused module to be ok");
}
