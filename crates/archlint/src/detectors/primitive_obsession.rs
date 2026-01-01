use crate::config::Config;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;

pub fn init() {}

pub struct PrimitiveObsessionDetector;

pub struct PrimitiveObsessionDetectorFactory;

impl DetectorFactory for PrimitiveObsessionDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "primitive_obsession",
            name: "Primitive Obsession Detector",
            description: "Detects functions with too many primitive parameters",
            default_enabled: false,
            is_deep: false,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(PrimitiveObsessionDetector)
    }
}

inventory::submit! {
    &PrimitiveObsessionDetectorFactory as &dyn DetectorFactory
}

impl Detector for PrimitiveObsessionDetector {
    fn name(&self) -> &'static str {
        "PrimitiveObsession"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.primitive_obsession;

        for (path, functions) in &ctx.function_complexity {
            for func in functions {
                if func.primitive_params > thresholds.max_primitives {
                    smells.push(ArchSmell::new_primitive_obsession(
                        path.clone(),
                        func.name.to_string(),
                        func.primitive_params,
                    ));
                }
            }
        }

        smells
    }
}
