use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;

pub struct GodModuleDetector;

pub struct GodModuleDetectorFactory;

impl DetectorFactory for GodModuleDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "god_module",
            name: "God Module Detector",
            description: "Detects large modules with many incoming and outgoing dependencies",
            default_enabled: true,
            is_deep: false,
            category: DetectorCategory::Global,
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

        // Check if git churn information is available
        let git_available = ctx.config.git.enabled && !ctx.churn_map.is_empty();

        for node in ctx.graph.nodes() {
            let fan_in = ctx.graph.fan_in(node);
            let fan_out = ctx.graph.fan_out(node);

            if let Some(path) = ctx.graph.get_file_path(node) {
                let rule = match ctx.get_rule_for_file("god_module", path) {
                    Some(r) => r,
                    None => continue,
                };

                let fan_in_threshold: usize = rule.get_option("fan_in").unwrap_or(10);
                let fan_out_threshold: usize = rule.get_option("fan_out").unwrap_or(10);
                let churn_threshold: usize = rule.get_option("churn").unwrap_or(20);

                let file_churn = ctx.churn_map.get(path).copied().unwrap_or(0);

                // If git is not available, we skip the churn threshold check
                let churn_ok = !git_available || file_churn >= churn_threshold;

                if fan_in >= fan_in_threshold && fan_out >= fan_out_threshold && churn_ok {
                    let mut smell =
                        ArchSmell::new_god_module(path.clone(), fan_in, fan_out, file_churn);
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}

pub fn init() {}
