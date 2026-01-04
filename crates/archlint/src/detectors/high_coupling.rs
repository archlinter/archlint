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
        let thresholds = &ctx.config.thresholds.high_coupling;

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                if ctx.is_excluded(path, &thresholds.exclude_patterns)
                    || ctx.should_skip_detector(path, "high_coupling")
                {
                    continue;
                }
                let fan_in = ctx.graph.fan_in(node);
                let fan_out = ctx.graph.fan_out(node);
                let cbo = fan_in + fan_out;

                if cbo >= thresholds.max_cbo {
                    smells.push(ArchSmell::new_high_coupling(path.clone(), cbo));
                }
            }
        }

        smells
    }
}
