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
        let thresholds = &ctx.config.thresholds.hub_module;

        ctx.graph
            .nodes()
            .filter_map(|node| Self::check_hub_node(ctx, node, thresholds))
            .collect()
    }
}

impl HubModuleDetector {
    fn check_hub_node(
        ctx: &AnalysisContext,
        node: petgraph::graph::NodeIndex,
        thresholds: &crate::config::HubModuleThresholds,
    ) -> Option<ArchSmell> {
        let fan_in = ctx.graph.fan_in(node);
        let fan_out = ctx.graph.fan_out(node);

        if !Self::meets_fan_thresholds(fan_in, fan_out, thresholds) {
            return None;
        }

        let path = ctx.graph.get_file_path(node)?;
        let max_complexity = Self::get_max_complexity(ctx, path);

        if max_complexity <= thresholds.max_complexity {
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

    fn meets_fan_thresholds(
        fan_in: usize,
        fan_out: usize,
        thresholds: &crate::config::HubModuleThresholds,
    ) -> bool {
        fan_in >= thresholds.min_fan_in && fan_out >= thresholds.min_fan_out
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
