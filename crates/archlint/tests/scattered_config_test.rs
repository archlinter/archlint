mod common;

use common::analyze_fixture_with_config;
use archlint::detectors::scattered_config::ScatteredConfigDetector;
use archlint::detectors::Detector;

#[test]
fn test_scattered_config_detected() {
    let mut config = archlint::config::Config::default();
    config.thresholds.scattered_config.max_files = 3;

    let ctx = analyze_fixture_with_config("config/scattered", config);
    let detector = ScatteredConfigDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect scattered config");
    assert!(smells.iter().any(|s| {
        if let archlint::detectors::SmellType::ScatteredConfiguration { env_var, files_count } = &s.smell_type {
            env_var == "API_KEY" && *files_count == 4
        } else {
            false
        }
    }));
}

#[test]
fn test_centralized_config_ok() {
    let mut config = archlint::config::Config::default();
    config.thresholds.scattered_config.max_files = 3;

    let ctx = analyze_fixture_with_config("config/centralized", config);
    let detector = ScatteredConfigDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected no scattered config smells for centralized config");
}
