mod common;

use archlint::config::{Config, RuleConfig, RuleFullConfig, RuleSeverity};
use archlint::detectors::design::abstractness::AbstractnessViolationDetector;
use archlint::detectors::Detector;
use common::{analyze_fixture_with_config, analyze_fixture_with_rule};

#[test]
fn test_zone_of_pain_detected() {
    let ctx = analyze_fixture_with_rule(
        "abstractness/pain",
        "abstractness",
        Some("distance_threshold: 0.5\nfan_in_threshold: 5"),
    );
    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // database.service.ts: 12 concrete clients -> A=0.0, I=0.0 -> D=1.0
    assert!(!smells.is_empty(), "Expected to detect Zone of Pain");
    assert!(smells.iter().any(|s| s
        .files
        .iter()
        .any(|f| f.to_string_lossy().contains("database.service.ts"))));
}

#[test]
fn test_smart_abstractness_ok() {
    let ctx = analyze_fixture_with_rule(
        "abstractness/smart_ok",
        "abstractness",
        Some("distance_threshold: 0.5\nfan_in_threshold: 5"),
    );
    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // service.ts: 10 clients importing IService -> A=1.0, I=0.0 -> D=0.0
    assert!(
        smells.is_empty(),
        "Expected smart abstractness to be ok when using interfaces"
    );
}

#[test]
fn test_smart_abstractness_pain() {
    let ctx = analyze_fixture_with_rule(
        "abstractness/smart_pain",
        "abstractness",
        Some("distance_threshold: 0.5\nfan_in_threshold: 5"),
    );
    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // service.ts: 10 clients importing Service class -> A=0.0, I=0.0 -> D=1.0
    assert!(
        !smells.is_empty(),
        "Expected to detect Zone of Pain when using concrete classes"
    );
}

#[test]
fn test_passive_roles_ignored() {
    let ctx = analyze_fixture_with_rule(
        "abstractness/smart_passive",
        "abstractness",
        Some("distance_threshold: 0.5\nfan_in_threshold: 5"),
    );
    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // error.ts and dto.ts should be ignored
    assert!(
        smells.is_empty(),
        "Expected passive roles (Errors, DTOs) to be ignored"
    );
}

#[test]
fn test_zone_of_uselessness_detected_with_low_fan_in() {
    let mut config = Config::default();
    config.rules.insert(
        "abstractness".to_string(),
        RuleConfig::Full(RuleFullConfig {
            enabled: Some(true),
            severity: Some(RuleSeverity::Medium),
            exclude: Vec::new(),
            options: serde_yaml::from_str("fan_in_threshold: 10\ndistance_threshold: 0.4").unwrap(),
        }),
    );

    let ctx = analyze_fixture_with_config("abstractness/useless", config);
    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // We expect one smell for useless.service.ts
    assert!(
        !smells.is_empty(),
        "Should detect uselessness even with low fan-in"
    );
    let useless_smell = smells
        .iter()
        .find(|s| {
            s.files
                .iter()
                .any(|f| f.to_string_lossy().contains("useless.service.ts"))
        })
        .expect("Should find useless service smell");

    assert!(useless_smell.abstractness().unwrap() >= 0.8);
    assert!(useless_smell.instability().unwrap() >= 0.8);
    assert!(useless_smell.fan_in().unwrap() < 10);
}

#[test]
fn test_external_interface_concrete_dependency() {
    let ctx = analyze_fixture_with_rule(
        "abstractness/external_interface",
        "abstractness",
        Some("distance_threshold: 0.5\nfan_in_threshold: 5"),
    );
    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // service.ts: 10 clients import UserService class (even if they also import IService from elsewhere)
    // This should still be a Zone of Pain for service.ts
    if smells.is_empty() {
        println!("No smells found! Context: {:?}", ctx.graph.node_count());
        for node in ctx.graph.nodes() {
            println!("Node: {:?}", ctx.graph.get_file_path(node));
        }
    }
    assert!(!smells.is_empty(), "Expected to detect Zone of Pain when class is imported directly, even with external interfaces");
    assert!(smells.iter().any(|s| s
        .files
        .iter()
        .any(|f| f.to_string_lossy().contains("user.service.ts"))));
}

#[test]
fn test_external_interface_abstract_dependency_ok() {
    let ctx = analyze_fixture_with_rule(
        "abstractness/external_interface_good",
        "abstractness",
        Some("distance_threshold: 0.5\nfan_in_threshold: 5"),
    );
    let detector = AbstractnessViolationDetector;
    let smells = detector.detect(&ctx);

    // user.service.ts: 10 clients import ONLY UserServiceOptions (interface)
    // Abstractness should be 1.0, so it should be OK
    assert!(
        smells.is_empty(),
        "Expected to be OK when only interfaces/types are imported from the service file"
    );
}
