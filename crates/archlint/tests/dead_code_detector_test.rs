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

    let detector = DeadCodeDetector::new(
        &ctx.config,
        HashSet::new(),
        Vec::new(),
        Vec::new(),
        ctx.project_path.clone(),
    );
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

#[test]
fn test_dead_code_with_exclude() {
    use archlint::config::{RuleConfig, RuleFullConfig};
    use std::collections::HashMap;

    // A.ts imports B.ts
    // If A.ts is excluded, B.ts should be dead (if A.ts was the only one importing it)
    // In our fixture: main.ts imports used.ts.
    // If we exclude main.ts, used.ts should become dead.

    let mut rules = HashMap::new();
    rules.insert(
        "dead_code".to_string(),
        RuleConfig::Full(RuleFullConfig {
            enabled: Some(true),
            exclude: vec!["main.ts".to_string()],
            ..Default::default()
        }),
    );

    let config = Config {
        rules,
        entry_points: vec!["main.ts".to_string()],
        ..Default::default()
    };

    let ctx = analyze_fixture_with_config("dead_code", config);

    let detector = DeadCodeDetector::new_default(&ctx.config);
    let smells = detector.detect(&ctx);

    // used.ts is imported by main.ts. Since main.ts is excluded, used.ts should be detected as dead code.
    assert!(
        smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with("used.ts"))),
        "Expected used.ts to be dead code when its only importer (main.ts) is excluded"
    );

    // main.ts itself should not be in results because it's excluded
    assert!(
        !smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with("main.ts"))),
        "Excluded file should not be reported as dead code"
    );
}
