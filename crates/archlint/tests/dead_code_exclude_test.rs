mod common;

use archlint::config::{Config, RuleConfig, RuleFullConfig};
use archlint::detectors::dead_code::DeadCodeDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_config;
use std::collections::{HashMap, HashSet};

#[test]
fn test_dead_code_with_nested_exclude() {
    let mut rules = HashMap::new();
    rules.insert(
        "dead_code".to_string(),
        RuleConfig::Full(RuleFullConfig {
            enabled: Some(true),
            exclude: vec!["ignored/**/*.ts".to_string()],
            ..Default::default()
        }),
    );

    let config = Config {
        rules,
        entry_points: vec!["main.ts".to_string()],
        ..Default::default()
    };

    let ctx = analyze_fixture_with_config("dead_code_nested", config);
    let rule = ctx.get_rule("dead_code").unwrap();

    let detector = DeadCodeDetector::new(
        &ctx.config,
        HashSet::new(),
        Vec::new(),
        rule.exclude.clone(),
        ctx.project_path.clone(),
    );
    let smells = detector.detect(&ctx);

    // ignored/unused.ts is excluded, should not be in results
    assert!(
        !smells.iter().any(|s| s
            .files
            .iter()
            .any(|f| f.to_string_lossy().contains("ignored/unused.ts"))),
        "Excluded file in nested directory should not be reported"
    );
}

#[test]
fn test_dead_code_excluded_importer() {
    let mut rules = HashMap::new();
    rules.insert(
        "dead_code".to_string(),
        RuleConfig::Full(RuleFullConfig {
            enabled: Some(true),
            exclude: vec!["importer_ignored/ignored_importer.ts".to_string()],
            ..Default::default()
        }),
    );

    let config = Config {
        rules,
        entry_points: vec!["main.ts".to_string()],
        ..Default::default()
    };

    let ctx = analyze_fixture_with_config("dead_code_nested", config);
    let rule = ctx.get_rule("dead_code").unwrap();

    let detector = DeadCodeDetector::new(
        &ctx.config,
        HashSet::new(),
        Vec::new(),
        rule.exclude.clone(),
        ctx.project_path.clone(),
    );
    let smells = detector.detect(&ctx);

    // importer_ignored/ignored_importer.ts is excluded, so its import of should_be_dead.ts
    // should be ignored, making should_be_dead.ts dead code.
    assert!(
        smells.iter().any(|s| s.files.iter().any(|f| f
            .to_string_lossy()
            .contains("importer_ignored/should_be_dead.ts"))),
        "should_be_dead.ts should be reported as dead code because its only importer is excluded"
    );

    // importer_ignored/ignored_importer.ts itself should not be in results because it's excluded
    assert!(
        !smells.iter().any(|s| s.files.iter().any(|f| f
            .to_string_lossy()
            .contains("importer_ignored/ignored_importer.ts"))),
        "Excluded importer should not be reported as dead code"
    );
}
