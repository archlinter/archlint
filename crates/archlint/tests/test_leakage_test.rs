mod common;

use archlint::config::Config;
use archlint::detectors::test_leakage::TestLeakageDetector;
use archlint::detectors::Detector;
use common::{analyze_fixture, analyze_fixture_with_config};

#[test]
fn test_leakage_detected() {
    let ctx = analyze_fixture("test_leakage/basic");
    let detector = TestLeakageDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect leakage");
    assert_eq!(smells.len(), 1);

    let smell = &smells[0];
    assert!(smell.files.iter().any(|f| f.ends_with("prod.ts")));
    assert!(smell
        .locations
        .iter()
        .any(|l| l.description.contains("test.test.ts")));
}

#[test]
fn test_leakage_mock_import() {
    let ctx = analyze_fixture("test_leakage/mock");
    let detector = TestLeakageDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect leakage from mock");
    assert_eq!(smells.len(), 1);

    let smell = &smells[0];
    assert!(smell.files.iter().any(|f| f.ends_with("prod.ts")));
    assert!(smell
        .locations
        .iter()
        .any(|l| l.description.contains("api.ts")));
}

#[test]
fn test_no_leakage() {
    let ctx = analyze_fixture("test_leakage/clean");
    let detector = TestLeakageDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected no leakage to be detected");
}

#[test]
fn test_test_to_test_ok() {
    // Empty the global ignore list: it covers test files by default, which would make
    // this pass without ever reaching the detector's own test-to-test guard.
    let config = Config {
        ignore: Vec::new(),
        ..Default::default()
    };
    let ctx = analyze_fixture_with_config("test_leakage/test_to_test", config);
    let detector = TestLeakageDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected no leakage for test-to-test imports"
    );
}
