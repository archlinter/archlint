mod common;

use archlint::config::Config;
use archlint::detectors::dead_code::DeadCodeDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_config;
use std::collections::HashSet;

#[test]
fn test_dead_code_detected() {
    let config = Config {
        entry_points: vec!["main.ts".to_string()],
        ..Default::default()
    };

    // Create a fixture:
    // test_data/dead_code/
    //   main.ts (entry point)
    //   used.ts (imported by main.ts)
    //   dead.ts (not imported by anyone)

    let ctx = analyze_fixture_with_config("dead_code", config);

    let detector = DeadCodeDetector::new(&ctx.config, HashSet::new(), Vec::new());
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect dead code");
    assert!(smells
        .iter()
        .any(|s| s.files.iter().any(|f| f.ends_with("dead.ts"))));
    assert!(!smells
        .iter()
        .any(|s| s.files.iter().any(|f| f.ends_with("used.ts"))));
    assert!(!smells
        .iter()
        .any(|s| s.files.iter().any(|f| f.ends_with("main.ts"))));
}
