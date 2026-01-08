mod common;

use archlint::detectors::scattered_config::ScatteredConfigDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_rule;

#[test]
fn test_scattered_config_detected() {
    let ctx =
        analyze_fixture_with_rule("config/scattered", "scattered_config", Some("max_files: 3"));
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
    let ctx = analyze_fixture_with_rule(
        "config/centralized",
        "scattered_config",
        Some("max_files: 3"),
    );
    let detector = ScatteredConfigDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected no scattered config smells for centralized config"
    );
}
