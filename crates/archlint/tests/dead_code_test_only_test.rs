mod common;

use archlint::config::{Config, RuleConfig, RuleFullConfig};
use archlint::detectors::dead_code::DeadCodeDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_config;
use std::collections::{HashMap, HashSet};

fn create_config_with_count_test_imports(count: bool) -> Config {
    let mut options = serde_yaml::Mapping::new();
    options.insert(
        serde_yaml::Value::String("count_test_imports".to_string()),
        serde_yaml::Value::Bool(count),
    );

    let mut rules = HashMap::new();
    rules.insert(
        "dead_code".to_string(),
        RuleConfig::Full(RuleFullConfig {
            enabled: Some(true),
            exclude: Vec::new(),
            severity: None,
            options: serde_yaml::Value::Mapping(options),
        }),
    );

    Config {
        rules,
        entry_points: vec!["main.ts".to_string()],
        ignore: Vec::new(),
        ..Default::default()
    }
}

#[test]
fn test_dead_code_flags_test_only_usage_when_disabled() {
    let config = create_config_with_count_test_imports(false);
    let ctx = analyze_fixture_with_config("dead_code_test_only", config);

    let detector = DeadCodeDetector::new(
        &ctx.config,
        HashSet::new(),
        Vec::new(),
        &[],
        ctx.project_path.clone(),
    );
    let smells = detector.detect(&ctx);

    // test_only_util.ts should be flagged: its only importer is a test file
    assert!(
        smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with("test_only_util.ts"))),
        "test_only_util.ts should be flagged as dead code when count_test_imports=false"
    );

    // prod_util.ts should NOT be flagged: imported by main.ts (production)
    assert!(
        !smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with("prod_util.ts"))),
        "prod_util.ts should NOT be flagged (used in production)"
    );

    // main.ts should NOT be flagged: it's an entry point
    assert!(
        !smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with("main.ts"))),
        "main.ts should NOT be flagged (entry point)"
    );

    // test file itself should NOT be flagged (test files are entry points)
    assert!(
        !smells.iter().any(|s| s
            .files
            .iter()
            .any(|f| f.ends_with("test_only_util.test.ts"))),
        "test file should NOT be flagged (entry point)"
    );
}

#[test]
fn test_dead_code_keeps_test_only_usage_when_enabled() {
    let config = create_config_with_count_test_imports(true);
    let ctx = analyze_fixture_with_config("dead_code_test_only", config);

    let detector = DeadCodeDetector::new(
        &ctx.config,
        HashSet::new(),
        Vec::new(),
        &[],
        ctx.project_path.clone(),
    );
    let smells = detector.detect(&ctx);

    // test_only_util.ts should NOT be flagged: test imports count
    assert!(
        !smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with("test_only_util.ts"))),
        "test_only_util.ts should NOT be flagged when count_test_imports=true"
    );
}

#[test]
fn test_dead_code_default_flags_test_only_usage() {
    // Default behavior (no option set) should flag test-only code (count_test_imports=false)
    let mut rules = HashMap::new();
    rules.insert(
        "dead_code".to_string(),
        RuleConfig::Full(RuleFullConfig {
            enabled: Some(true),
            ..Default::default()
        }),
    );
    let config = Config {
        rules,
        entry_points: vec!["main.ts".to_string()],
        ignore: Vec::new(),
        ..Default::default()
    };
    let ctx = analyze_fixture_with_config("dead_code_test_only", config);

    let detector = DeadCodeDetector::new(
        &ctx.config,
        HashSet::new(),
        Vec::new(),
        &[],
        ctx.project_path.clone(),
    );
    let smells = detector.detect(&ctx);

    // Default: test_only_util.ts SHOULD be flagged (count_test_imports defaults to false)
    assert!(
        smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with("test_only_util.ts"))),
        "Default behavior should flag test-only code"
    );
}
