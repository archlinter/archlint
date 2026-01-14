use archlint::{scan, ScanOptions, Severity};
use std::path::PathBuf;

fn get_test_data(subpath: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test_data");
    path.push(subpath);
    path
}

#[test]
fn test_scan_basic() {
    let test_data = get_test_data("cycles");
    let options = ScanOptions::new();
    let result = scan(&test_data, options).expect("Scan failed");

    assert!(result.summary.files_analyzed > 0);
    assert!(!result.smells.is_empty());
    assert!(!result.files.is_empty());

    // Check that we found cycles
    assert!(result.summary.cyclic_dependencies > 0);
    assert!(result.summary.cycle_clusters > 0);
}

#[test]
fn test_scan_with_detector_filter() {
    let test_data = get_test_data("cycles");

    // Only run cycles detector
    let mut options = ScanOptions::new();
    options.detectors = Some(vec!["cyclic_dependency".to_string()]);

    let result = scan(&test_data, options).expect("Scan failed");

    // Should only have cycle smells
    for smell in &result.smells {
        assert!(matches!(
            smell.smell.smell_type,
            archlint::SmellType::CyclicDependency | archlint::SmellType::CyclicDependencyCluster
        ));
    }
}

#[test]
fn test_scan_with_exclude_filter() {
    let test_data = get_test_data("cycles");

    // Exclude cycles detector
    let mut options = ScanOptions::new();
    options.exclude_detectors = vec!["cyclic_dependency".to_string()];

    let result = scan(&test_data, options).expect("Scan failed");

    // Should not have cycle smells
    for smell in &result.smells {
        assert!(!matches!(
            smell.smell.smell_type,
            archlint::SmellType::CyclicDependency | archlint::SmellType::CyclicDependencyCluster
        ));
    }
}

#[test]
fn test_scan_with_severity_filter() {
    let test_data = get_test_data("cycles");

    // Only Critical (unlikely in this test data, but we check filter works)
    let mut options = ScanOptions::new();
    options.min_severity = Some(Severity::Critical);

    let result = scan(&test_data, options).expect("Scan failed");

    for smell in &result.smells {
        assert!(smell.smell.severity >= Severity::Critical);
    }
}

#[test]
fn test_file_info_details() {
    let test_data = get_test_data("cycles/simple_cycle");
    let options = ScanOptions::new();
    let result = scan(&test_data, options).expect("Scan failed");

    // Find a specific file (e.g., a.ts)
    let file_a = result
        .files
        .iter()
        .find(|f| f.path.ends_with("a.ts"))
        .expect("a.ts not found");

    assert!(file_a.metrics.lines > 0);
    assert!(!file_a.imports.is_empty());

    // Check import
    let import = &file_a.imports[0];
    assert!(import.source.contains("b"));
}

#[test]
fn test_load_config() {
    // Default config
    let config = archlint::load_config::<PathBuf>(None).expect("Failed to load default config");
    assert!(!config.ignore.is_empty());

    // Load from test data
    let config_path = get_test_data("config/.archlint.yaml");
    if config_path.exists() {
        let config = archlint::load_config(Some(config_path)).expect("Failed to load config file");
        assert!(config.git.enabled);
    }
}

#[test]
fn test_get_detectors() {
    let detectors = archlint::get_detectors();
    assert!(!detectors.is_empty());

    let cycle_detector = detectors
        .iter()
        .find(|d| d.id == "cyclic_dependency")
        .expect("Cycles detector not found");
    assert_eq!(cycle_detector.name, "Cycle Detector");
}

#[test]
fn test_serialization_compatibility() {
    let test_data = get_test_data("cycles/simple_cycle");
    let result = scan(&test_data, ScanOptions::new()).expect("Scan failed");

    let json = serde_json::to_value(&result).expect("Failed to serialize");

    // Check fields for JS compatibility (camelCase)
    assert!(json.get("summary").is_some());
    assert!(json.get("summary").unwrap().get("filesAnalyzed").is_some());
    assert!(json.get("smells").is_some());
    assert!(json.get("files").is_some());
    assert!(json.get("grade").is_some());
}
