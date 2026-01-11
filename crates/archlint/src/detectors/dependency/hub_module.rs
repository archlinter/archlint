use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use std::path::PathBuf;

pub fn init() {}

#[detector(
    id = "hub_module",
    name = "Hub Module Detector",
    description = "Detects modules that act as highly connected hubs with low internal logic",
    category = DetectorCategory::GraphBased,
    default_enabled = false
)]
pub struct HubModuleDetector;

impl HubModuleDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn check_hub_node(
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
                    .map(|func| func.complexity)
                    .max()
                    .unwrap_or(0)
            })
            .unwrap_or(0)
    }
}

impl Detector for HubModuleDetector {
    fn name(&self) -> &'static str {
        "HubModule"
    }

    fn explain(&self, _smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Hub Module".to_string(),
            reason: "Module acting as a pass-through hub with many incoming and outgoing connections but little internal logic.".to_string(),
            risks: vec!["Fragile bridge".to_string(), "Unnecessary abstraction layer".to_string()],
            recommendations: vec!["Consolidate the hub or direct dependants to the target modules".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::explain::ExplainEngine;
        crate::define_report_section!("Hub Modules", smells, {
            crate::render_table!(
                vec!["File", "Fan-In", "Fan-Out", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let file_path = smell.files.first().unwrap();
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let fan_in = smell.fan_in().unwrap_or(0);
                    let fan_out = smell.fan_out().unwrap_or(0);
                    let pts = smell.score(severity_config);
                    vec![
                        format!("`{}`", formatted_path),
                        fan_in.to_string(),
                        fan_out.to_string(),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        ctx.graph
            .nodes()
            .filter_map(|node| {
                let path = ctx.graph.get_file_path(node)?;
                let rule = ctx.get_rule_for_file("hub_module", path)?;

                let mut smell = Self::check_hub_node(ctx, node, &rule)?;
                smell.severity = rule.severity;
                Some(smell)
            })
            .collect()
    }
}
