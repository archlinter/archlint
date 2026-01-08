use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;

pub struct UnstableInterfaceDetector;

pub struct UnstableInterfaceDetectorFactory;

impl DetectorFactory for UnstableInterfaceDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "unstable_interface",
            name: "Unstable Interface Detector",
            description: "Detects modules with high churn and many dependants",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::Global,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(UnstableInterfaceDetector)
    }
}

inventory::submit! {
    &UnstableInterfaceDetectorFactory as &dyn DetectorFactory
}

impl Detector for UnstableInterfaceDetector {
    fn name(&self) -> &'static str {
        "UnstableInterface"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        // Check if git churn information is available
        let git_available = ctx.config.enable_git && !ctx.churn_map.is_empty();

        ctx.graph
            .nodes()
            .filter_map(|node| {
                let path = ctx.graph.get_file_path(node)?;
                let rule = ctx.get_rule_for_file("unstable_interface", path)?;

                let min_churn: usize = rule.get_option("min_churn").unwrap_or(10);
                let min_dependants: usize = rule.get_option("min_dependants").unwrap_or(5);
                let score_threshold: usize = rule.get_option("score_threshold").unwrap_or(100);

                let churn = ctx.churn_map.get(path).copied().unwrap_or(0);
                let dependants = ctx.graph.fan_in(node);

                let score = churn * dependants;

                // If git is not available, we skip the churn and score threshold checks
                let churn_ok = !git_available || churn >= min_churn;
                let score_ok = !git_available || score >= score_threshold;

                if churn_ok && score_ok && dependants >= min_dependants {
                    let mut smell =
                        ArchSmell::new_unstable_interface(path.clone(), churn, dependants, score);
                    smell.severity = rule.severity;
                    Some(smell)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn init() {}
