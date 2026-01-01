mod common;

use archlint::detectors::vendor_coupling::VendorCouplingDetector;
use archlint::detectors::Detector;
use common::analyze_fixture;

#[test]
fn test_axios_spread_detected() {
    let ctx = analyze_fixture("vendor/axios_spread");
    let detector = VendorCouplingDetector;
    let smells = detector.detect(&ctx);

    assert!(
        !smells.is_empty(),
        "Expected to detect vendor coupling for axios"
    );
    assert!(smells.iter().any(|s| {
        if let archlint::detectors::SmellType::VendorCoupling { package } = &s.smell_type {
            package == "axios"
        } else {
            false
        }
    }));
}

#[test]
fn test_react_ignored() {
    let ctx = analyze_fixture("vendor/react_ok");
    let detector = VendorCouplingDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected react to be ignored");
}

#[test]
fn test_wrapped_package_ok() {
    let ctx = analyze_fixture("vendor/wrapped");
    let detector = VendorCouplingDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected wrapped axios to be ok");
}
