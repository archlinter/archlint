use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use petgraph::graph::NodeIndex;

pub fn init() {}

#[detector(
    id = "sdp_violation",
    name = "Stable Dependency Principle Violation Detector",
    description = "Detects when stable modules depend on unstable ones",
    category = DetectorCategory::GraphBased,
    default_enabled = false
)]
pub struct SdpViolationDetector;

impl SdpViolationDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

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

impl Detector for SdpViolationDetector {
    fn name(&self) -> &'static str {
        "SdpViolation"
    }

    fn explain(&self, _smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Stable Dependency Principle (SDP) Violation".to_string(),
            reason: "A stable module (rarely changing, many dependants) depends on an unstable module (frequently changing).".to_string(),
            risks: vec![
                "Stable modules become unstable due to their dependencies".to_string(),
                "Fragile architecture: changes in unstable parts break the core".to_string(),
            ],
            recommendations: vec![
                "Identify stable interfaces and depend on them".to_string(),
                "Refactor the unstable dependency to be more stable".to_string(),
                "Invert the dependency using abstractions".to_string(),
            ],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::report::format_location;
        crate::define_report_section!("SDP Violations", smells, {
            crate::render_table!(
                vec!["Location", "Stability Gap", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let file_path = smell.files.first().unwrap();
                    let location = format_location(file_path, 0, None); // Should have line info
                    let pts = smell.score(severity_config);
                    vec![
                        format!("`{}`", location),
                        "High instability diff".to_string(),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        ctx.graph
            .nodes()
            .flat_map(|node| {
                if let Some(path) = ctx.graph.get_file_path(node) {
                    let rule = match ctx.get_rule_for_file("sdp_violation", path) {
                        Some(r) => r,
                        None => return Vec::new(),
                    };

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
