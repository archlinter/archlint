mod common;

use common::analyze_fixture;
use archlint::detectors::barrel_abuse::BarrelFileAbuseDetector;
use archlint::detectors::Detector;

#[test]
fn test_barrel_too_many_exports() {
    let ctx = analyze_fixture("barrel/many_exports");
    let detector = BarrelFileAbuseDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect barrel file abuse");
    assert_eq!(smells.len(), 1);

    let smell = &smells[0];
    assert!(smell.files.iter().any(|f| f.ends_with("index.ts")));
    assert!(smell.metrics.iter().any(|m| matches!(m, archlint::detectors::SmellMetric::DependantCount(15))));
}

#[test]
fn test_small_barrel_ok() {
    let ctx = analyze_fixture("barrel/small");
    let detector = BarrelFileAbuseDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected no abuse for small barrel file");
}
