mod common;

use archlint::config::{Config, RuleConfig, RuleFullConfig};
use archlint::detectors::Detector;
use common::analyze_fixture_with_config;
use std::collections::HashMap;

fn create_dead_symbols_config(count_test_imports: bool) -> Config {
    let mut options = serde_yaml::Mapping::new();
    options.insert(
        serde_yaml::Value::String("count_test_imports".to_string()),
        serde_yaml::Value::Bool(count_test_imports),
    );

    let mut rules = HashMap::new();
    rules.insert(
        "dead_symbols".to_string(),
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

fn is_dead_symbol(smells: &[archlint::detectors::ArchSmell], name: &str) -> bool {
    smells.iter().any(|s| {
        if let archlint::detectors::SmellType::DeadSymbol { name: sname, .. } = &s.smell_type {
            sname == name
        } else {
            false
        }
    })
}

#[test]
fn test_barrel_reexport_without_consumer_is_dead() {
    // testHelper is re-exported through index.ts barrel, but no production
    // code imports it from the barrel. Only the test file imports it directly.
    // With count_test_imports=false, testHelper should be dead.
    let config = create_dead_symbols_config(false);
    let ctx = analyze_fixture_with_config("dead_symbols_barrel", config);

    let detector = archlint::detectors::dead_symbols::DeadSymbolsDetector;
    let smells = detector.detect(&ctx);

    assert!(
        is_dead_symbol(&smells, "testHelper"),
        "testHelper should be flagged: re-exported through barrel but never consumed. Got smells: {:?}",
        smells.iter().filter_map(|s| if let archlint::detectors::SmellType::DeadSymbol { name, .. } = &s.smell_type { Some(name.as_str()) } else { None }).collect::<Vec<_>>()
    );
}

#[test]
fn test_barrel_reexport_with_consumer_is_alive() {
    // prodHelper is re-exported through index.ts barrel AND imported
    // by main.ts from the barrel. It should NOT be dead.
    let config = create_dead_symbols_config(false);
    let ctx = analyze_fixture_with_config("dead_symbols_barrel", config);

    let detector = archlint::detectors::dead_symbols::DeadSymbolsDetector;
    let smells = detector.detect(&ctx);

    assert!(
        !is_dead_symbol(&smells, "prodHelper"),
        "prodHelper should NOT be flagged: consumed through barrel by main.ts"
    );
}

#[test]
fn test_barrel_with_test_imports_counted() {
    // With count_test_imports=true (default), testHelper should NOT be dead
    // because the test file's import counts as usage.
    let config = create_dead_symbols_config(true);
    let ctx = analyze_fixture_with_config("dead_symbols_barrel", config);

    let detector = archlint::detectors::dead_symbols::DeadSymbolsDetector;
    let smells = detector.detect(&ctx);

    assert!(
        !is_dead_symbol(&smells, "testHelper"),
        "testHelper should NOT be flagged when count_test_imports=true"
    );
}

#[test]
fn test_barrel_default_counts_test_imports() {
    // Default behavior (no option) should count test imports (backward compat)
    let mut rules = HashMap::new();
    rules.insert(
        "dead_symbols".to_string(),
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
    let ctx = analyze_fixture_with_config("dead_symbols_barrel", config);

    let detector = archlint::detectors::dead_symbols::DeadSymbolsDetector;
    let smells = detector.detect(&ctx);

    assert!(
        !is_dead_symbol(&smells, "testHelper"),
        "Default should count test imports (backward compat)"
    );
}
