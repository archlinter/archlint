use crate::config::Config;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;

pub fn init() {}

pub struct LongParameterListDetector;

pub struct LongParameterListDetectorFactory;

impl DetectorFactory for LongParameterListDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "long_params",
            name: "Long Parameter List Detector",
            description: "Detects functions with too many parameters",
            default_enabled: true,
            is_deep: false,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(LongParameterListDetector)
    }
}

inventory::submit! {
    &LongParameterListDetectorFactory as &dyn DetectorFactory
}

impl Detector for LongParameterListDetector {
    fn name(&self) -> &'static str {
        "LongParameterList"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.long_params;

        for (path, functions) in &ctx.function_complexity {
            for func in functions {
                if thresholds.ignore_constructors && func.is_constructor {
                    continue;
                }

                if func.param_count > thresholds.max_params {
                    smells.push(ArchSmell::new_long_params(
                        path.clone(),
                        func.name.to_string(),
                        func.param_count,
                    ));
                }
            }
        }

        smells
    }
}
