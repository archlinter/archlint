mod common;

use archlint::detectors::long_params::LongParameterListDetector;
use archlint::detectors::Detector;
use common::analyze_fixture;

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
    let mut config = archlint::config::Config::default();
    config.rules.insert(
        "long_params".to_string(),
        archlint::config::RuleConfig::Full(archlint::config::RuleFullConfig {
            enabled: Some(true),
            severity: None,
            exclude: Vec::new(),
            options: serde_yaml::from_str("ignore_constructors: false\nmax_params: 5").unwrap(),
        }),
    );

    let ctx = analyze_fixture_with_config("params/constructor", config);
    let detector = LongParameterListDetector;
    let smells = detector.detect(&ctx);

    assert!(
        !smells.is_empty(),
        "Expected to detect long constructor when not ignored"
    );
}

use common::analyze_fixture_with_config;
