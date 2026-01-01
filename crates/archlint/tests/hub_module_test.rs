mod common;

use common::analyze_fixture;
use archlint::detectors::hub_module::HubModuleDetector;
use archlint::detectors::Detector;

#[test]
fn test_hub_detected() {
    let ctx = analyze_fixture("hub/basic");
    let detector = HubModuleDetector;
    let smells = detector.detect(&ctx);

    assert!(!smells.is_empty(), "Expected to detect hub module");
    assert!(smells.iter().any(|s| s.files.iter().any(|f| f.ends_with("hub.ts"))));
}

#[test]
fn test_god_module_not_hub() {
    let ctx = analyze_fixture("hub/god_module");
    let detector = HubModuleDetector;
    let smells = detector.detect(&ctx);

    // god.ts has high complexity, so it shouldn't be detected by HubModuleDetector
    // (though it would be detected by GodModuleDetector)
    assert!(smells.is_empty(), "God module should not be detected as a hub due to high complexity");
}
