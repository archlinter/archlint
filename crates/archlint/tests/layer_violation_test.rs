mod common;

use archlint::config::{Config, LayerConfig};
use archlint::detectors::layer_violation::LayerViolationDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_config;

fn get_layer_config() -> Config {
    let mut config = Config::default();
    config.thresholds.layer_violation.layers = vec![
        LayerConfig {
            name: "domain".to_string(),
            path: "**/domain/**".to_string(),
            allowed_imports: vec![],
        },
        LayerConfig {
            name: "infra".to_string(),
            path: "**/infra/**".to_string(),
            allowed_imports: vec!["domain".to_string()],
        },
    ];
    config
}

#[test]
fn test_domain_imports_infra_violation() {
    let config = get_layer_config();
    let ctx = analyze_fixture_with_config("layers/violation", config);
    let detector = LayerViolationDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect layer violation");
    assert!(smells.iter().any(|s| {
        s.files.iter().any(|f| f.ends_with("user.ts"))
            && s.locations
                .iter()
                .any(|l| l.description.contains("layer 'infra'"))
    }));
}

#[test]
fn test_valid_layers_ok() {
    let config = get_layer_config();
    let ctx = analyze_fixture_with_config("layers/valid", config);
    let detector = LayerViolationDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected no layer violations in valid fixture"
    );
}

#[test]
fn test_no_layers_config() {
    let ctx = common::analyze_fixture("layers/violation");
    let detector = LayerViolationDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected no violations when no layers are configured"
    );
}
