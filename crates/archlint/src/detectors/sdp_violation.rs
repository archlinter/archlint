use crate::config::Config;
use crate::detectors::DetectorCategory;
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
            category: DetectorCategory::GraphBased,
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
        ctx.graph
            .nodes()
            .flat_map(|node| {
                if let Some(path) = ctx.graph.get_file_path(node) {
                    let rule = ctx.resolve_rule("sdp_violation", Some(path));
                    if !rule.enabled
                        || ctx.is_excluded(path, &rule.exclude)
                        || ctx.should_skip_detector(path, "sdp_violation")
                    {
                        return Vec::new();
                    }

                    let mut node_smells = Self::check_node_violations(ctx, node, &rule);
                    for smell in &mut node_smells {
                        smell.severity = rule.severity;
                    }
                    node_smells
                } else {
                    Vec::new()
                }
            })
            .collect()
    }
}

impl SdpViolationDetector {
    fn check_node_violations(
        ctx: &AnalysisContext,
        node: NodeIndex,
        rule: &crate::rule_resolver::ResolvedRuleConfig,
    ) -> Vec<ArchSmell> {
        let min_fan_total: usize = rule.get_option("min_fan_total").unwrap_or(5);
        let instability_diff: f64 = rule.get_option("instability_diff").unwrap_or(0.3);

        let fan_in = ctx.graph.fan_in(node);
        let fan_out = ctx.graph.fan_out(node);

        if fan_in + fan_out < min_fan_total {
            return Vec::new();
        }

        let from_i = Self::calculate_instability_static(ctx, node);
        let mut smells = Vec::new();

        for to_node in ctx.graph.dependencies(node) {
            let to_i = Self::calculate_instability_static(ctx, to_node);

            if from_i < to_i && (to_i - from_i) > instability_diff {
                if let (Some(from_path), Some(to_path)) = (
                    ctx.graph.get_file_path(node),
                    ctx.graph.get_file_path(to_node),
                ) {
                    let edge_data = ctx.graph.get_edge_data(node, to_node);
                    let (import_line, import_range) = edge_data
                        .map(|e| (e.import_line, e.import_range))
                        .unwrap_or((0, None));

                    smells.push(ArchSmell::new_sdp_violation(
                        from_path.clone(),
                        to_path.clone(),
                        from_i,
                        to_i,
                        import_line,
                        import_range,
                    ));
                }
            }
        }

        smells
    }

    fn calculate_instability_static(ctx: &AnalysisContext, node: NodeIndex) -> f64 {
        let fan_in = ctx.graph.fan_in(node);
        let fan_out = ctx.graph.fan_out(node);
        if fan_in + fan_out == 0 {
            return 0.0;
        }
        fan_out as f64 / (fan_in + fan_out) as f64
    }
}
