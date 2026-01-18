use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use petgraph::graph::NodeIndex;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

#[detector(SmellType::SdpViolation, default_enabled = false)]
pub struct SdpViolationDetector;

impl SdpViolationDetector {
    #[must_use]
    pub const fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn check_node_violations(
        &self,
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

        let from_i = self.calculate_instability(ctx, node);
        let mut smells = Vec::new();

        for to_node in ctx.graph.dependencies(node) {
            let to_i = self.calculate_instability(ctx, to_node);

            if from_i < to_i && (to_i - from_i) > instability_diff {
                if let (Some(from_path), Some(to_path)) = (
                    ctx.graph.get_file_path(node),
                    ctx.graph.get_file_path(to_node),
                ) {
                    let edge_data = ctx.graph.get_edge_data(node, to_node);
                    let (import_line, import_range) =
                        edge_data.map_or((0, None), |e| (e.import_line, e.import_range));

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

    fn calculate_instability(&self, ctx: &AnalysisContext, node: NodeIndex) -> f64 {
        let fan_in = ctx.graph.fan_in(node);
        let fan_out = ctx.graph.fan_out(node);
        if fan_in + fan_out == 0 {
            return 0.0;
        }
        fan_out as f64 / (fan_in + fan_out) as f64
    }
}

impl Detector for SdpViolationDetector {
    crate::impl_detector_report!(
        explain: _smell => (
            problem: "Stable Dependency Principle (SDP) Violation",
            reason: "A stable module (rarely changing, many dependents) depends on an unstable module (frequently changing).",
            risks: [
                "Stable modules become unstable due to their dependencies",
                "Fragile architecture: changes in unstable parts break the core"
            ],
            recommendations: [
                "Identify stable interfaces and depend on them",
                "Refactor the unstable dependency to be more stable",
                "Invert the dependency using abstractions"
            ]
        ),
        table: {
            title: "SDP Violations",
            columns: ["Location", "Stability Gap", "pts"],
            row: SdpViolation { } (smell, location, pts) => [location, "High instability diff", pts]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        ctx.graph
            .nodes()
            .flat_map(|node| {
                if let Some(path) = ctx.graph.get_file_path(node) {
                    if let Some(rule) = ctx.get_rule_for_file("sdp_violation", path) {
                        let mut node_smells = self.check_node_violations(ctx, node, &rule);
                        for smell in &mut node_smells {
                            smell.severity = rule.severity;
                        }
                        return node_smells;
                    }
                }
                Vec::new()
            })
            .collect()
    }
}
