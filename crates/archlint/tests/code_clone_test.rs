mod common;

use archlint::config::{Config, RuleConfig, RuleFullConfig};
use archlint::detectors::code_clone::CodeCloneDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_config;
use std::collections::HashMap;

#[test]
fn test_code_clones_detected_type2() {
    let mut rules = HashMap::new();
    let mut code_clone_options = HashMap::new();
    code_clone_options.insert("min_tokens".to_string(), serde_yaml::Value::from(20));
    code_clone_options.insert("min_lines".to_string(), serde_yaml::Value::from(3));

    let options = serde_yaml::to_value(code_clone_options).unwrap();

    rules.insert(
        "code_clone".to_string(),
        RuleConfig::Full(RuleFullConfig {
            enabled: Some(true),
            severity: None,
            exclude: Vec::new(),
            options,
        }),
    );

    let config = Config {
        rules,
        ..Config::default()
    };

    let ctx = analyze_fixture_with_config("clones/simple", config);

    // We manually create the detector with the same params as in config
    let detector = CodeCloneDetector::create_for_test(20, 3);
    let smells = detector.detect(&ctx);

    assert!(
        !smells.is_empty(),
        "Expected to detect code clones, got {:?}",
        smells
    );
    assert!(smells.iter().any(|s| {
        matches!(
            s.smell_type,
            archlint::detectors::SmellType::CodeClone { .. }
        )
    }));
}
