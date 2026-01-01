use crate::config::Config;
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
        let thresholds = &ctx.config.thresholds.unstable_interface;

        ctx.graph
            .nodes()
            .filter_map(|node| {
                let path = ctx.graph.get_file_path(node)?;
                let churn = ctx.churn_map.get(path).copied().unwrap_or(0);
                let dependants = ctx.graph.fan_in(node);

                let score = churn * dependants;

                if score >= thresholds.score_threshold
                    && churn >= thresholds.min_churn
                    && dependants >= thresholds.min_dependants
                {
                    Some(ArchSmell::new_unstable_interface(
                        path.clone(),
                        churn,
                        dependants,
                        score,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn init() {}
