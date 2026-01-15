mod common;

use archlint::detectors::lcom::LcomDetector;
use archlint::detectors::Detector;
use common::{analyze_fixture, analyze_fixture_with_rule};

#[test]
fn test_high_lcom_detected() {
    let ctx = analyze_fixture_with_rule("lcom/high", "lcom", Some("max_lcom: 1"));
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect low cohesion");
    assert!(smells
        .iter()
        .any(|s| s.files.iter().any(|f| f.ends_with("class.ts"))));
}

#[test]
fn test_cohesive_class_ok() {
    let ctx = analyze_fixture("lcom/cohesive");
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected cohesive class to be ok");
}

#[test]
fn test_small_class_ignored() {
    let ctx = analyze_fixture("lcom/small");
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected small class to be ignored");
}

#[test]
fn test_typescript_parameter_properties_recognized() {
    // Test that TypeScript constructor parameter properties are recognized as fields
    let ctx = analyze_fixture_with_rule(
        "lcom/typescript_parameter_properties",
        "lcom",
        Some("max_lcom: 2"),
    );
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    // Methods using usersRepository should be connected, so LCOM should be low
    // generateNonce doesn't use fields, so it's separate
    // Expected: LCOM = 2 (main group + generateNonce)
    assert!(
        smells.is_empty(),
        "Expected TypeScript parameter properties to create cohesive class"
    );
}

#[test]
fn test_nestjs_di_pattern_supported() {
    // Test NestJS DI pattern where constructor parameters are fields
    let ctx = analyze_fixture_with_rule("lcom/nestjs_di", "lcom", Some("max_lcom: 2"));
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    // All methods use configService or orderRepository, so they should be connected
    assert!(
        smells.is_empty(),
        "Expected NestJS DI pattern to create cohesive class"
    );
}

#[test]
fn test_constructors_and_accessors_excluded() {
    // Test that constructors and accessors are excluded from LCOM calculation
    let ctx = analyze_fixture_with_rule("lcom/with_accessors", "lcom", Some("max_lcom: 2"));
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    // getData, setData, processData all use repository, so they should be connected
    // get/set accessors are excluded, so LCOM should be 1
    assert!(
        smells.is_empty(),
        "Expected accessors to be excluded from LCOM"
    );
}

#[test]
fn test_min_methods_uses_filtered_count() {
    // Test that min_methods check uses filtered method count (excluding constructor)
    // This class has 1 constructor + 2 methods = 3 total, but only 2 after filtering
    // With min_methods: 3, it should be ignored
    let ctx = analyze_fixture_with_rule(
        "lcom/min_methods_filtered",
        "lcom",
        Some("min_methods: 3\nmax_lcom: 4"),
    );
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected class with 2 filtered methods to be ignored when min_methods=3"
    );
}

#[test]
fn test_min_methods_passes_with_filtered_count() {
    // Test that min_methods check passes when filtered count meets threshold
    let ctx = analyze_fixture_with_rule(
        "lcom/min_methods_filtered",
        "lcom",
        Some("min_methods: 2\nmax_lcom: 1"),
    );
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    // Both methods use repo, so LCOM = 1, which is <= max_lcom: 1
    assert!(
        smells.is_empty(),
        "Expected class with 2 filtered methods to pass when min_methods=2"
    );
}

#[test]
fn test_mixed_explicit_and_constructor_fields() {
    // Test class with both explicit fields and constructor parameter properties
    let ctx = analyze_fixture_with_rule("lcom/mixed_fields", "lcom", Some("max_lcom: 2"));
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    // All methods use at least one field, and most use constructorField1 or constructorField2
    // They should be connected through shared fields
    assert!(
        smells.is_empty(),
        "Expected mixed fields to create cohesive class"
    );
}

#[test]
fn test_only_constructor_ignored() {
    // Test that class with only constructor is ignored (no methods after filtering)
    let ctx = analyze_fixture_with_rule("lcom/only_constructor", "lcom", Some("min_methods: 1"));
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected class with only constructor to be ignored"
    );
}

#[test]
fn test_only_accessors_ignored() {
    // Test that class with only accessors is ignored (no methods after filtering)
    let ctx = analyze_fixture_with_rule("lcom/only_accessors", "lcom", Some("min_methods: 1"));
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    assert!(
        smells.is_empty(),
        "Expected class with only accessors to be ignored"
    );
}

#[test]
fn test_method_calls_create_connections() {
    // Test that method calls create connections between methods
    let ctx = analyze_fixture_with_rule("lcom/method_calls", "lcom", Some("max_lcom: 1"));
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    // method1 -> method2 -> method3 creates a chain through method calls
    // method4 uses field, but doesn't share fields with others and doesn't call/gets called
    // Expected: LCOM = 2 (chain method1-3 + isolated method4)
    // With max_lcom: 1, this should be detected
    assert!(
        !smells.is_empty(),
        "Expected low cohesion due to method4 being isolated from method chain"
    );
}

#[test]
fn test_no_fields_high_lcom() {
    // Test that class with no fields has high LCOM (all methods isolated)
    let ctx = analyze_fixture_with_rule("lcom/no_fields", "lcom", Some("max_lcom: 3"));
    let detector = LcomDetector;
    let smells = detector.detect(&ctx);

    // All 4 methods have no fields and don't call each other
    // Expected: LCOM = 4 (each method is isolated)
    assert!(
        !smells.is_empty(),
        "Expected high LCOM for class with no fields"
    );
}
