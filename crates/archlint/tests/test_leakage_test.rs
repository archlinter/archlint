mod common;

use archlint::detectors::test_leakage::TestLeakageDetector;
use archlint::detectors::Detector;
use common::analyze_fixture;

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
    let ctx = analyze_fixture("test_leakage/test_to_test");
    let detector = TestLeakageDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected no leakage for test-to-test imports"
    );
}
