mod common;

use archlint::detectors::primitive_obsession::PrimitiveObsessionDetector;
use archlint::detectors::Detector;
use common::analyze_fixture;

#[test]
fn test_high_primitives_detected() {
    let ctx = analyze_fixture("primitive/high");
    let detector = PrimitiveObsessionDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect primitive obsession");
    assert!(smells.iter().any(|s| {
        if let archlint::detectors::SmellType::PrimitiveObsession { primitives, .. } = &s.smell_type
        {
            *primitives == 4
        } else {
            false
        }
    }));
}

#[test]
fn test_ok_primitives_ok() {
    let ctx = analyze_fixture("primitive/ok");
    let detector = PrimitiveObsessionDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected object parameter to be ok");
}
