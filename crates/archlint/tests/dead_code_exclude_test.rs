mod common;

use archlint::detectors::dead_code::DeadCodeDetector;
use archlint::detectors::Detector;
use common::{analyze_fixture_with_config, create_dead_code_config};
use std::collections::HashSet;

#[test]
fn test_dead_code_with_nested_exclude() {
    let config = create_dead_code_config(vec!["ignored/**/*.ts".to_string()]);
    let ctx = analyze_fixture_with_config("dead_code_nested", config);
    let rule = ctx.get_rule("dead_code").unwrap();

    let detector = DeadCodeDetector::new(
        &ctx.config,
        HashSet::new(),
        Vec::new(),
        &rule.exclude,
        ctx.project_path.clone(),
    );
    let smells = detector.detect(&ctx);

    // ignored/unused.ts is excluded, should not be in results
    let ignored_unused = std::path::Path::new("ignored").join("unused.ts");
    assert!(
        !smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with(&ignored_unused))),
        "Excluded file in nested directory should not be reported"
    );
}

#[test]
fn test_dead_code_excluded_importer() {
    let config = create_dead_code_config(vec!["importer_ignored/ignored_importer.ts".to_string()]);
    let ctx = analyze_fixture_with_config("dead_code_nested", config);
    let rule = ctx.get_rule("dead_code").unwrap();

    let detector = DeadCodeDetector::new(
        &ctx.config,
        HashSet::new(),
        Vec::new(),
        &rule.exclude,
        ctx.project_path.clone(),
    );
    let smells = detector.detect(&ctx);

    // importer_ignored/ignored_importer.ts is excluded, so its import of should_be_dead.ts
    // should be ignored, making should_be_dead.ts dead code.
    let should_be_dead = std::path::Path::new("importer_ignored").join("should_be_dead.ts");
    assert!(
        smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with(&should_be_dead))),
        "should_be_dead.ts should be reported as dead code because its only importer is excluded"
    );

    // importer_ignored/ignored_importer.ts itself should not be in results because it's excluded
    let ignored_importer = std::path::Path::new("importer_ignored").join("ignored_importer.ts");
    assert!(
        !smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with(&ignored_importer))),
        "Excluded importer should not be reported as dead code"
    );
}
