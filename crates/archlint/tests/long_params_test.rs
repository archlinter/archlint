mod common;

use archlint::detectors::long_params::LongParameterListDetector;
use archlint::detectors::Detector;
use common::{analyze_fixture, analyze_fixture_with_rule};

#[test]
fn test_many_params_detected() {
    let ctx = analyze_fixture("params/many");
    let detector = LongParameterListDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect long parameter list");
    assert!(smells.iter().any(|s| {
        if let archlint::detectors::SmellType::LongParameterList { count, .. } = &s.smell_type {
            *count == 7
        } else {
            false
        }
    }));
}

#[test]
fn test_few_params_ok() {
    let ctx = analyze_fixture("params/few");
    let detector = LongParameterListDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected few parameters to be ok");
}

#[test]
fn test_constructor_ignored_by_default() {
    let ctx = analyze_fixture("params/constructor");
    let detector = LongParameterListDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected constructor to be ignored by default"
    );
}

#[test]
fn test_constructor_detected_when_not_ignored() {
    let ctx = analyze_fixture_with_rule(
        "params/constructor",
        "long_params",
        Some("ignore_constructors: false\nmax_params: 5"),
    );
    let detector = LongParameterListDetector;
    let smells = detector.detect(&ctx);

    assert!(
        !smells.is_empty(),
        "Expected to detect long constructor when not ignored"
    );
}
