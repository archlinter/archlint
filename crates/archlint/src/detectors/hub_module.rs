use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use std::path::PathBuf;

pub fn init() {}

pub struct HubModuleDetector;

pub struct HubModuleDetectorFactory;

impl DetectorFactory for HubModuleDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "hub_module",
            name: "Hub Module Detector",
            description:
                "Detects modules that act as highly connected hubs with low internal logic",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::GraphBased,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(HubModuleDetector)
    }
}

inventory::submit! {
    &HubModuleDetectorFactory as &dyn DetectorFactory
}

impl Detector for HubModuleDetector {
    fn name(&self) -> &'static str {
        "HubModule"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        ctx.graph
            .nodes()
            .filter_map(|node| {
                let path = ctx.graph.get_file_path(node)?;
                let rule = ctx.resolve_rule("hub_module", Some(path));
                if !rule.enabled || ctx.is_excluded(path, &rule.exclude) {
                    return None;
                }

                let mut smell = Self::check_hub_node(ctx, node, &rule)?;
                smell.severity = rule.severity;
                Some(smell)
            })
            .collect()
    }
}

impl HubModuleDetector {
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
