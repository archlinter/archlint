use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(
    smell_type = SmellType::HighCoupling,
    name = "High Coupling Detector (CBO)",
    description = "Detects modules with too many incoming and outgoing dependencies",
    category = DetectorCategory::GraphBased,
    default_enabled = false
)]
pub struct HighCouplingDetector;

impl HighCouplingDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for HighCouplingDetector {
    crate::impl_detector_report!(
        name: "HighCoupling",
        explain: smell => (
            problem: {
                let cbo = if let crate::detectors::SmellType::HighCoupling { cbo } = &smell.smell_type {
                    *cbo
                } else {
                    0
                };
                format!("High Coupling (CBO): {}", cbo)
            },
            reason: "Module has too many incoming and outgoing dependencies (Coupling Between Objects). High coupling makes code difficult to change and test in isolation.",
            risks: [
                "Fragile system: changes ripple through many modules",
                "Difficult to mock dependencies for testing"
            ],
            recommendations: [
                "Refactor to reduce dependencies or move functionality to a more appropriate place"
            ]
        ),
        table: {
            title: "High Coupling",
            columns: ["File", "CBO Score", "pts"],
            row: HighCoupling { cbo } (smell, location, pts) => [location, cbo, pts]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                let rule = match ctx.get_rule_for_file("high_coupling", path) {
                    Some(r) => r,
                    None => continue,
                };

                let max_cbo: usize = rule.get_option("max_cbo").unwrap_or(20);

                let fan_in = ctx.graph.fan_in(node);
                let fan_out = ctx.graph.fan_out(node);
                let cbo = fan_in + fan_out;

                if cbo > max_cbo {
                    let mut smell = ArchSmell::new_high_coupling(path.clone(), cbo);
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
