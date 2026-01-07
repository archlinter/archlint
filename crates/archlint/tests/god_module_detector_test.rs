mod common;

use archlint::config::Config;
use archlint::detectors::god_module::GodModuleDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_config;

#[test]
fn test_god_module_detected() {
    let mut config = Config::default();
    config.rules.insert(
        "god_module".to_string(),
        archlint::config::RuleConfig::Full(archlint::config::RuleFullConfig {
            enabled: Some(true),
            severity: None,
            exclude: Vec::new(),
            options: serde_yaml::from_str("fan_in: 2\nfan_out: 2\nchurn: 0").unwrap(),
        }),
    );

    // Create a fixture with 2 incoming and 2 outgoing dependencies
    // test_data/god_module/
    //   god.ts (imports dep1, dep2)
    //   caller1.ts (imports god)
    //   caller2.ts (imports god)
    //   dep1.ts
    //   dep2.ts

    let ctx = analyze_fixture_with_config("god_module", config);
    let detector = GodModuleDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect God Module");
    assert!(smells[0].files.iter().any(|f| f.ends_with("god.ts")));
}
