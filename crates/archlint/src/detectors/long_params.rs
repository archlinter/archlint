use crate::config::Config;
use crate::detectors::DetectorCategory;
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
            category: DetectorCategory::FileLocal,
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

        for (path, functions) in ctx.function_complexity.as_ref() {
            let rule = ctx.resolve_rule("long_params", Some(path));
            if !rule.enabled || ctx.is_excluded(path, &rule.exclude) {
                continue;
            }

            let ignore_constructors: bool = rule.get_option("ignore_constructors").unwrap_or(true);
            let max_params: usize = rule.get_option("max_params").unwrap_or(5);

            for func in functions {
                if ignore_constructors && func.is_constructor {
                    continue;
                }

                if func.param_count > max_params {
                    let mut smell = ArchSmell::new_long_params(
                        path.clone(),
                        func.name.to_string(),
                        func.param_count,
                        func.line,
                        func.range,
                    );
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
