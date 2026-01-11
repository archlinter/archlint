use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;

pub fn init() {}

pub struct HighCouplingDetector;

pub struct HighCouplingDetectorFactory;

impl DetectorFactory for HighCouplingDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "high_coupling",
            name: "High Coupling Detector (CBO)",
            description: "Detects modules with too many incoming and outgoing dependencies",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::GraphBased,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(HighCouplingDetector)
    }
}

inventory::submit! {
    &HighCouplingDetectorFactory as &dyn DetectorFactory
}

impl Detector for HighCouplingDetector {
    fn name(&self) -> &'static str {
        "HighCoupling"
    }

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

                if cbo >= max_cbo {
                    let mut smell = ArchSmell::new_high_coupling(path.clone(), cbo);
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
