use crate::config::Config;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use petgraph::graph::NodeIndex;

pub fn init() {}

pub struct SdpViolationDetector;

pub struct SdpViolationDetectorFactory;

impl DetectorFactory for SdpViolationDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "sdp_violation",
            name: "Stable Dependency Principle Violation Detector",
            description: "Detects when stable modules depend on unstable ones",
            default_enabled: false,
            is_deep: false,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(SdpViolationDetector)
    }
}

inventory::submit! {
    &SdpViolationDetectorFactory as &dyn DetectorFactory
}

impl Detector for SdpViolationDetector {
    fn name(&self) -> &'static str {
        "SdpViolation"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.sdp_violation;

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                if ctx.should_skip_detector(path, "sdp_violation") {
                    continue;
                }
            }

            let from_i = self.calculate_instability(ctx, node);

            // Only check modules with enough fan-total to be meaningful
            let fan_in = ctx.graph.fan_in(node);
            let fan_out = ctx.graph.fan_out(node);
            if fan_in + fan_out < thresholds.min_fan_total {
                continue;
            }

            for to_node in ctx.graph.dependencies(node) {
                let to_i = self.calculate_instability(ctx, to_node);

                // Stable module depends on unstable
                if from_i < to_i && (to_i - from_i) > thresholds.instability_diff {
                    if let (Some(from_path), Some(to_path)) = (
                        ctx.graph.get_file_path(node),
                        ctx.graph.get_file_path(to_node),
                    ) {
                        smells.push(ArchSmell::new_sdp_violation(
                            from_path.clone(),
                            to_path.clone(),
                            from_i,
                            to_i,
                        ));
                    }
                }
            }
        }

        smells
    }
}

impl SdpViolationDetector {
    fn calculate_instability(&self, ctx: &AnalysisContext, node: NodeIndex) -> f64 {
        let fan_in = ctx.graph.fan_in(node);
        let fan_out = ctx.graph.fan_out(node);
        if fan_in + fan_out == 0 {
            return 0.0;
        }
        fan_out as f64 / (fan_in + fan_out) as f64
    }
}
