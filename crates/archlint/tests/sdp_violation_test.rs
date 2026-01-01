mod common;

use common::analyze_fixture;
use archlint::detectors::sdp_violation::SdpViolationDetector;
use archlint::detectors::Detector;

#[test]
fn test_stable_depends_unstable_violation() {
    let ctx = analyze_fixture("sdp/violation");
    let detector = SdpViolationDetector;
    let smells = detector.detect(&ctx);

    for smell in &smells {
        for loc in &smell.locations {
            eprintln!("SDP Detected: {}", loc.description);
        }
    }

    assert!(!smells.is_empty(), "Expected to detect SDP violation");
    assert!(smells.iter().any(|s| {
        s.files.iter().any(|f| f.ends_with("core.ts")) &&
        s.locations.iter().any(|l| l.description.contains("Stable module") && l.description.contains("depends on unstable module"))
    }));
}

#[test]
fn test_no_violation_valid() {
    let ctx = analyze_fixture("sdp/valid");
    let detector = SdpViolationDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected no SDP violations in valid fixture");
}

#[test]
fn test_isolated_files_ignored() {
    let ctx = analyze_fixture("sdp/isolated");
    let detector = SdpViolationDetector;
    let smells = detector.detect(&ctx);

    assert!(smells.is_empty(), "Expected no violations for isolated files (total fan < 5)");
}
