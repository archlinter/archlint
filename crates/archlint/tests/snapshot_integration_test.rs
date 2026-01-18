mod common;

use archlint::config::{Config, RuleConfig, RuleFullConfig, RuleSeverity};
use archlint::detectors::{ArchSmell, Detector, Severity, SmellMetric, SmellType};
use archlint::snapshot::{SnapshotGenerator, SnapshotSmell};
use common::analyze_fixture_with_config;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;

#[test]
fn test_metrics_roundtrip_integration() {
    let project_root = PathBuf::from("/test");
    let generator = SnapshotGenerator::new(project_root.clone());

    // Create a smell with diverse metrics
    let metrics = vec![
        SmellMetric::FanIn(10),
        SmellMetric::CyclomaticComplexity(25),
        SmellMetric::InstabilityDiff(0.75),
        SmellMetric::FilesCount(5),
    ];

    let original_smell = ArchSmell {
        smell_type: SmellType::GodModule,
        severity: Severity::High,
        files: vec![project_root.join("src/god.ts")],
        metrics,
        locations: vec![],
        cluster: None,
    };

    // 1. Convert ArchSmell -> SnapshotSmell (via SnapshotGenerator)
    // We'll use the internal convert_smell logic by proxy or just test the parts
    // Since convert_smell is private, we'll use generate() which uses it.

    // Create a mock ScanResult
    let smells = vec![archlint::api::SmellWithExplanation {
        smell: original_smell,
        explanation: archlint::Explanation {
            problem: "test".into(),
            reason: "test".into(),
            risks: vec![],
            recommendations: vec![],
        },
    }];

    let scan_result = archlint::api::ScanResult {
        smells,
        summary: Default::default(),
        files: vec![],
        grade: Default::default(),
        project_path: project_root,
    };

    let snapshot = generator.generate(&scan_result);
    let snapshot_smell = &snapshot.smells[0];

    // 2. Convert SnapshotSmell -> ArchSmell (via TryFrom trait)
    let restored_smell = ArchSmell::try_from(snapshot_smell).unwrap();

    // 3. Verify all metrics are preserved
    assert_eq!(restored_smell.fan_in(), Some(10));
    assert_eq!(restored_smell.cyclomatic_complexity(), Some(25));
    // InstabilityDiff doesn't have an accessor yet, but we can check the metrics vector
    let has_instability = restored_smell
        .metrics
        .iter()
        .any(|m| matches!(m, SmellMetric::InstabilityDiff(v) if (*v - 0.75).abs() < 0.001));
    assert!(has_instability, "InstabilityDiff should be preserved");

    let has_files_count = restored_smell
        .metrics
        .iter()
        .any(|m| matches!(m, SmellMetric::FilesCount(5)));
    assert!(has_files_count, "FilesCount should be preserved");
}

#[test]
fn test_vendor_coupling_per_file_rule_integration() {
    // Setup a config with ONLY per-file rules for vendor_coupling
    let mut config = Config::default();
    let mut options = serde_yaml::Mapping::new();
    options.insert(
        serde_yaml::Value::String("max_files_per_package".to_string()),
        serde_yaml::Value::Number(1.into()), // Trigger with > 1 file
    );

    config.rules.insert(
        "src/**".to_string(),
        RuleConfig::Full(RuleFullConfig {
            enabled: Some(true),
            severity: Some(RuleSeverity::High),
            exclude: vec![],
            options: serde_yaml::Value::Mapping(options),
        }),
    );

    // Use analyze_fixture with this config
    // axios_spread fixture has many files importing axios
    let ctx = analyze_fixture_with_config("vendor/axios_spread", config);

    let detector = archlint::detectors::dependency::vendor_coupling::VendorCouplingDetector;
    let smells = detector.detect(&ctx);

    // Should detect axios because per-file rule enabled it
    assert!(
        !smells.is_empty(),
        "Should detect vendor coupling via per-file rules"
    );
    assert!(smells.iter().any(|s| {
        if let SmellType::VendorCoupling { package } = &s.smell_type {
            package == "axios"
        } else {
            false
        }
    }));
}

#[test]
fn test_unknown_smell_type_integration() {
    // Create a snapshot smell with a type that doesn't exist in current code
    let snapshot = SnapshotSmell {
        id: "test:unknown".into(),
        smell_type: "FutureSmellFrom2030".into(),
        severity: "Medium".into(),
        files: vec!["src/file.ts".into()],
        metrics: HashMap::new(),
        details: None,
        locations: vec![],
    };

    let restored_smell = ArchSmell::try_from(&snapshot).unwrap();

    match restored_smell.smell_type {
        SmellType::Unknown { raw_type } => {
            assert_eq!(raw_type, "FutureSmellFrom2030");
        }
        _ => panic!(
            "Expected SmellType::Unknown, got {:?}",
            restored_smell.smell_type
        ),
    }
}
