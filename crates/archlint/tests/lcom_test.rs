mod common;

use archlint::detectors::lcom::LcomDetector;
use archlint::detectors::Detector;
use common::analyze_fixture;

#[test]
fn test_high_lcom_detected() {
    let mut config = archlint::config::Config::default();
    config.thresholds.lcom.max_lcom = 1; // 2+ components is a smell
    let ctx = analyze_fixture_with_config("lcom/high", config);
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect low cohesion");
    assert!(smells
        .iter()
        .any(|s| s.files.iter().any(|f| f.ends_with("class.ts"))));
}

#[test]
fn test_cohesive_class_ok() {
    let ctx = analyze_fixture("lcom/cohesive");
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected cohesive class to be ok");
}

#[test]
fn test_small_class_ignored() {
    let ctx = analyze_fixture("lcom/small");
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected small class to be ignored");
}

use common::analyze_fixture_with_config;
