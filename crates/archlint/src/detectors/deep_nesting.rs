use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;

pub fn init() {}

pub struct DeepNestingDetector;

pub struct DeepNestingDetectorFactory;

impl DetectorFactory for DeepNestingDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "deep_nesting",
            name: "Deep Nesting Detector",
            description: "Detects functions with excessive nesting depth",
            default_enabled: true,
            is_deep: false,
            category: DetectorCategory::FileLocal,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(DeepNestingDetector)
    }
}

inventory::submit! {
    &DeepNestingDetectorFactory as &dyn DetectorFactory
}

impl Detector for DeepNestingDetector {
    fn name(&self) -> &'static str {
        "DeepNesting"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.deep_nesting;

        for (path, functions) in ctx.function_complexity.as_ref() {
            for func in functions {
                if func.max_depth > thresholds.max_depth {
                    smells.push(ArchSmell::new_deep_nesting(
                        path.clone(),
                        func.name.to_string(),
                        func.max_depth,
                    ));
                }
            }
        }

        smells
    }
}
