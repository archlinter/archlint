mod common;

use archlint::detectors::package_cycle::PackageCycleDetector;
use archlint::detectors::Detector;
use common::analyze_fixture;

#[test]
fn test_package_cycle_detected() {
    let ctx = analyze_fixture("pkg_cycle/basic");
    let detector = PackageCycleDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect package cycle");
    assert!(smells.iter().any(|s| {
        if let archlint::detectors::SmellType::PackageCycle { packages } = &s.smell_type {
            packages.contains(&"auth".to_string()) && packages.contains(&"users".to_string())
        } else {
            false
        }
    }));
}

#[test]
fn test_file_cycle_not_package_cycle() {
    let ctx = analyze_fixture("pkg_cycle/file_only");
    let detector = PackageCycleDetector;
    let smells = detector.detect(&ctx);

    // If depth is 1 (default), they are in the same package "pkg"
    // So it's a file cycle, but NOT a package cycle between different packages.
    assert!(
        smells.is_empty(),
        "Expected no package cycle between different packages"
    );
}
