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
        &[],
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
    use common::create_dead_code_config;

    // Test fixture has: main.ts (entry) -> used.ts, and dead.ts (unused)
    // We exclude dead.ts to verify that excluded files are not reported,
    // even when they would otherwise be detected as dead code.
    let config = create_dead_code_config(vec!["dead.ts".to_string()]);
    let ctx = analyze_fixture_with_config("dead_code", config);

    let rule = ctx.get_rule("dead_code").unwrap();
    // Explicitly construct the detector with values from config to avoid implicit behavior in detect()
    let detector = DeadCodeDetector::new(
        &ctx.config,
        HashSet::new(),
        Vec::new(),
        &rule.exclude,
        ctx.project_path.clone(),
    );
    let smells = detector.detect(&ctx);

    // dead.ts is normally dead code, but it's excluded, so should NOT be in results
    assert!(
        !smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with("dead.ts"))),
        "Excluded file (dead.ts) should not be reported as dead code"
    );

    // main.ts is entry point, should not be reported
    assert!(
        !smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with("main.ts"))),
        "Entry point should not be reported as dead code"
    );

    // used.ts is imported by main.ts, should not be reported
    assert!(
        !smells
            .iter()
            .any(|s| s.files.iter().any(|f| f.ends_with("used.ts"))),
        "Used file should not be reported as dead code"
    );

    // No dead code should be detected since dead.ts is excluded
    assert!(
        smells.is_empty(),
        "Expected no dead code when the only dead file is excluded"
    );
}
