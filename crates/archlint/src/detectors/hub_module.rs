use crate::config::Config;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;

pub fn init() {}

pub struct HubModuleDetector;

pub struct HubModuleDetectorFactory;

impl DetectorFactory for HubModuleDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "hub_module",
            name: "Hub Module Detector",
            description: "Detects modules that act as highly connected hubs with low internal logic",
            default_enabled: false,
            is_deep: false,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(HubModuleDetector)
    }
}

inventory::submit! {
    &HubModuleDetectorFactory as &dyn DetectorFactory
}

impl Detector for HubModuleDetector {
    fn name(&self) -> &'static str {
        "HubModule"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.hub_module;

        for node in ctx.graph.nodes() {
            let fan_in = ctx.graph.fan_in(node);
            let fan_out = ctx.graph.fan_out(node);

            if fan_in >= thresholds.min_fan_in && fan_out >= thresholds.min_fan_out {
                if let Some(path) = ctx.graph.get_file_path(node) {
                    // Check complexity
                    let mut max_complexity = 0;
                    if let Some(functions) = ctx.function_complexity.get(path) {
                        for func in functions {
                            if func.complexity > max_complexity {
                                max_complexity = func.complexity;
                            }
                        }
                    }

                    if max_complexity <= thresholds.max_complexity {
                        smells.push(ArchSmell::new_hub_module(path.clone(), fan_in, fan_out, max_complexity));
                    }
                }
            }
        }

        smells
    }
}
