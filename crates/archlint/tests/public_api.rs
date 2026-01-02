use archlint::{scan, ScanOptions};
use std::path::PathBuf;

#[test]
fn test_public_scan() {
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("test_data/cycles");

    let options = ScanOptions::new();
    let result = scan(&test_data, options).expect("Scan failed");

    assert!(result.summary.files_analyzed > 0);
    assert!(!result.smells.is_empty());
    assert!(!result.files.is_empty());
    assert_eq!(
        result.project_path.canonicalize().unwrap(),
        test_data.canonicalize().unwrap()
    );

    // Verify serialization
    let json = serde_json::to_string(&result).expect("Failed to serialize result");
    assert!(json.contains("summary"));
    assert!(json.contains("smells"));
    assert!(json.contains("files"));
}

#[test]
fn test_load_config() {
    let config = archlint::load_config::<PathBuf>(None).expect("Failed to load default config");
    assert!(!config.ignore.is_empty());
}

#[test]
fn test_get_detectors() {
    let detectors = archlint::get_detectors();
    assert!(!detectors.is_empty());
    assert!(detectors.iter().any(|d| d.id == "cycles"));
}
