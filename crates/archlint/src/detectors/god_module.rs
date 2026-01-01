use crate::config::Config;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;

pub fn init() {}

pub struct GodModuleDetector;

pub struct GodModuleDetectorFactory;

impl DetectorFactory for GodModuleDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "god_module",
            name: "God Module Detector",
            description: "Detects modules that have too many dependencies or are too central",
            default_enabled: true,
            is_deep: false,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(GodModuleDetector)
    }
}

inventory::submit! {
    &GodModuleDetectorFactory as &dyn DetectorFactory
}

impl Detector for GodModuleDetector {
    fn name(&self) -> &'static str {
        "GodModule"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.god_module;

        for node in ctx.graph.nodes() {
            let fan_in = ctx.graph.fan_in(node);
            let fan_out = ctx.graph.fan_out(node);

            if let Some(path) = ctx.graph.get_file_path(node) {
                let file_churn = ctx.churn_map.get(path).copied().unwrap_or(0);

                if fan_in >= thresholds.fan_in
                    && fan_out >= thresholds.fan_out
                    && file_churn >= thresholds.churn
                {
                    smells.push(ArchSmell::new_god_module(
                        path.clone(),
                        fan_in,
                        fan_out,
                        file_churn,
                    ));
                }
            }
        }

        smells
    }
}
