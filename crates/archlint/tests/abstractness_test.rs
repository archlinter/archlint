mod common;

use common::analyze_fixture_with_config;
use archlint::detectors::abstractness::AbstractnessViolationDetector;
use archlint::detectors::Detector;

#[test]
fn test_zone_of_pain_detected() {
    let mut config = archlint::config::Config::default();
    config.thresholds.abstractness.distance_threshold = 0.5;

    let ctx = analyze_fixture_with_config("abstractness/pain", config);
    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // rigid.ts: A=0 (concrete), I=0 (stable) -> D = |0+0-1| = 1.0
    assert!(!smells.is_empty(), "Expected to detect Zone of Pain");
    assert!(smells.iter().any(|s| s.files.iter().any(|f| f.ends_with("rigid.ts"))));
}

#[test]
fn test_zone_of_uselessness_detected() {
    let mut config = archlint::config::Config::default();
    config.thresholds.abstractness.distance_threshold = 0.5;

    let ctx = analyze_fixture_with_config("abstractness/useless", config);
    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // unused.ts: A=1.0 (only type), I=1.0 (depends on something, no one depends on it) -> D = |1+1-1| = 1.0
    assert!(!smells.is_empty(), "Expected to detect Zone of Uselessness");
    assert!(smells.iter().any(|s| s.files.iter().any(|f| f.ends_with("unused.ts"))));
}

#[test]
fn test_balanced_module_ok() {
    let ctx = common::analyze_fixture("abstractness/ok");
    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected balanced module to be ok");
}
