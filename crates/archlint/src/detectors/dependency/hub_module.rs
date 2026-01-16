use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use std::path::PathBuf;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(SmellType::HubModule, default_enabled = false)]
pub struct HubModuleDetector;

impl HubModuleDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn check_hub_node(
        &self,
        ctx: &AnalysisContext,
        node: petgraph::graph::NodeIndex,
        rule: &crate::rule_resolver::ResolvedRuleConfig,
    ) -> Option<ArchSmell> {
        let fan_in = ctx.graph.fan_in(node);
        let fan_out = ctx.graph.fan_out(node);

        let min_fan_in: usize = rule.get_option("min_fan_in").unwrap_or(5);
        let min_fan_out: usize = rule.get_option("min_fan_out").unwrap_or(5);
        let max_complexity_threshold: usize = rule.get_option("max_complexity").unwrap_or(5);

        if fan_in < min_fan_in || fan_out < min_fan_out {
            return None;
        }

        let path = ctx.graph.get_file_path(node)?;
        let max_complexity = Self::get_max_complexity(ctx, path);

        if max_complexity <= max_complexity_threshold {
            Some(ArchSmell::new_hub_module(
                path.clone(),
                fan_in,
                fan_out,
                max_complexity,
            ))
        } else {
            None
        }
    }

    fn get_max_complexity(ctx: &AnalysisContext, path: &PathBuf) -> usize {
        ctx.function_complexity
            .get(path)
            .map(|functions| {
                functions
                    .iter()
                    .map(|func| func.cyclomatic_complexity)
                    .max()
                    .unwrap_or(0)
            })
            .unwrap_or(0)
    }
}

impl Detector for HubModuleDetector {
    crate::impl_detector_report!(
        explain: _smell => (
            problem: "Hub Module",
            reason: "Module acting as a pass-through hub with many incoming and outgoing connections but little internal logic.",
            risks: [
                "Fragile bridge",
                "Unnecessary abstraction layer"
            ],
            recommendations: [
                "Consolidate the hub or direct dependents to the target modules"
            ]
        ),
        table: {
            title: "Hub Modules",
            columns: ["File", "Fan-In", "Fan-Out", "pts"],
            row: HubModule { } (smell, location, pts) => [
                location,
                smell.fan_in().unwrap_or(0),
                smell.fan_out().unwrap_or(0),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        ctx.graph
            .nodes()
            .filter_map(|node| {
                let path = ctx.graph.get_file_path(node)?;
                let rule = ctx.get_rule_for_file("hub_module", path)?;

                let mut smell = self.check_hub_node(ctx, node, &rule)?;
                smell.severity = rule.severity;
                Some(smell)
            })
            .collect()
    }
}
