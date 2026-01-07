mod common;

use archlint::detectors::scattered_config::ScatteredConfigDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_config;

#[test]
fn test_scattered_config_detected() {
    let mut config = archlint::config::Config::default();
    config.rules.insert(
        "scattered_config".to_string(),
        archlint::config::RuleConfig::Full(archlint::config::RuleFullConfig {
            enabled: Some(true),
            severity: None,
            exclude: Vec::new(),
            options: serde_yaml::from_str("max_files: 3").unwrap(),
        }),
    );

    let ctx = analyze_fixture_with_config("config/scattered", config);
    let detector = ScatteredConfigDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect scattered config");
    assert!(smells.iter().any(|s| {
        if let archlint::detectors::SmellType::ScatteredConfiguration {
            env_var,
            files_count,
        } = &s.smell_type
        {
            env_var == "API_KEY" && *files_count == 4
        } else {
            false
        }
    }));
}

#[test]
fn test_centralized_config_ok() {
    let mut config = archlint::config::Config::default();
    config.rules.insert(
        "scattered_config".to_string(),
        archlint::config::RuleConfig::Full(archlint::config::RuleFullConfig {
            enabled: Some(true),
            severity: None,
            exclude: Vec::new(),
            options: serde_yaml::from_str("max_files: 3").unwrap(),
        }),
    );

    let ctx = analyze_fixture_with_config("config/centralized", config);
    let detector = ScatteredConfigDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected no scattered config smells for centralized config"
    );
}
