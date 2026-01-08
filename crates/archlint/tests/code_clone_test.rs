mod common;

use archlint::detectors::code_clone::CodeCloneDetector;
use archlint::detectors::Detector;
use common::analyze_fixture_with_rule;

#[test]
fn test_code_clones_detected_exact() {
    let ctx = analyze_fixture_with_rule(
        "clones/exact",
        "code_clone",
        Some(
            r#"
            min_tokens: 20
            min_lines: 3
            "#,
        ),
    );

    let detector = CodeCloneDetector;
    let smells = detector.detect(&ctx);

    assert!(
        !smells.is_empty(),
        "Expected to detect code clones, got {:?}",
        smells
    );
}

#[test]
fn test_code_clones_in_exported_class() {
    let ctx = analyze_fixture_with_rule(
        "clones/exported",
        "code_clone",
        Some(
            r#"
            min_tokens: 20
            min_lines: 3
            "#,
        ),
    );

    let detector = CodeCloneDetector;
    let smells = detector.detect(&ctx);

    assert!(
        !smells.is_empty(),
        "Expected to detect code clones in exported class, got {:?}",
        smells
    );
}
