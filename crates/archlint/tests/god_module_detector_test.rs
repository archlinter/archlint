mod common;

use common::analyze_fixture_with_config;
use archlint::detectors::god_module::GodModuleDetector;
use archlint::detectors::Detector;
use archlint::config::Config;


#[test]
fn test_god_module_detected() {
    let mut config = Config::default();
    config.thresholds.god_module.fan_in = 2;
    config.thresholds.god_module.fan_out = 2;
    config.thresholds.god_module.churn = 0; // Disable churn check for this test

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
