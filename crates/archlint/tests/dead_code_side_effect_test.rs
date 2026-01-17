mod common;

use archlint::config::Config;
use archlint::detectors::dead_code::DeadCodeDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_config;
use std::collections::HashSet;

#[test]
fn test_dead_code_side_effect_import() {
    let config = Config {
        entry_points: vec!["app.ts".to_string()],
        ..Default::default()
    };

    // Fixture side_effect/basic has:
    // app.ts: import './init'; export const run = () => {};
    // init.ts: console.log('init'); (no exports)

    let ctx = analyze_fixture_with_config("side_effect/basic", config);

    let detector = DeadCodeDetector::new(
        &ctx.config,
        HashSet::new(),
        Vec::new(),
        Vec::new(),
        ctx.project_path.clone(),
    );
    let smells = detector.detect(&ctx);

    // init.ts should NOT be reported as dead code because it's imported for side effects
    let is_init_dead = smells
        .iter()
        .any(|s| s.files.iter().any(|f| f.ends_with("init.ts")));

    assert!(
        !is_init_dead,
        "File with side-effect import (init.ts) should not be reported as dead code"
    );
}
