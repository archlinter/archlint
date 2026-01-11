use crate::config::Config;
use crate::detectors::DetectorCategory;
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
            category: DetectorCategory::FileLocal,
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

        for (path, functions) in ctx.function_complexity.as_ref() {
            let rule = match ctx.get_rule_for_file("primitive_obsession", path) {
                Some(r) => r,
                None => continue,
            };

            let max_primitives: usize = rule.get_option("max_primitives").unwrap_or(3);

            for func in functions {
                if func.primitive_params > max_primitives {
                    let mut smell = ArchSmell::new_primitive_obsession(
                        path.clone(),
                        func.name.to_string(),
                        func.primitive_params,
                    );
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
