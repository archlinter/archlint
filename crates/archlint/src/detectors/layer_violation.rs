use crate::config::{Config, LayerConfig};
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use petgraph::graph::NodeIndex;
use std::path::{Path, PathBuf};

pub fn init() {}

pub struct LayerViolationDetector;

pub struct LayerViolationDetectorFactory;

impl DetectorFactory for LayerViolationDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "layer_violation",
            name: "Layer Architecture Violation Detector",
            description: "Detects violations of layered architecture rules",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::ImportBased,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(LayerViolationDetector)
    }
}

inventory::submit! {
    &LayerViolationDetectorFactory as &dyn DetectorFactory
}

impl Detector for LayerViolationDetector {
    fn name(&self) -> &'static str {
        "LayerViolation"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let layers = &ctx.config.thresholds.layer_violation.layers;

        if layers.is_empty() {
            return Vec::new();
        }

        ctx.graph
            .nodes()
            .filter_map(|node| self.get_node_info(ctx, node, layers))
            .flat_map(|from_info| {
                Self::check_dependencies_for_violations(ctx, from_info, layers, self)
            })
            .collect()
    }
}

impl LayerViolationDetector {
    fn check_dependencies_for_violations(
        ctx: &AnalysisContext,
        from_info: (&PathBuf, &LayerConfig),
        layers: &[LayerConfig],
        detector: &LayerViolationDetector,
    ) -> Vec<ArchSmell> {
        let (from_path, _) = from_info;
        let mut smells = Vec::new();

        // Find node index for from_path
        if let Some(node) = ctx.graph.get_node(from_path) {
            for to_node in ctx.graph.dependencies(node) {
                if let Some(to_info) = detector.get_node_info(ctx, to_node, layers) {
                    let edge_data = ctx.graph.get_edge_data(node, to_node);
                    if let Some(smell) = detector.check_violation(from_info, to_info, edge_data) {
                        smells.push(smell);
                    }
                }
            }
        }

        smells
    }
}

impl LayerViolationDetector {
    fn find_layer<'a>(&self, path: &Path, layers: &'a [LayerConfig]) -> Option<&'a LayerConfig> {
        // Find the most specific layer (longest matching path)
        let mut matching_layers: Vec<&LayerConfig> = layers
            .iter()
            .filter(|l| self.matches_path(path, &l.path))
            .collect();

        matching_layers.sort_by_key(|l| std::cmp::Reverse(l.path.len()));

        matching_layers.first().copied()
    }

    fn matches_path(&self, path: &Path, pattern: &str) -> bool {
        let path_str = path.to_string_lossy();

        if pattern.contains("**") {
            Self::matches_glob_pattern(&path_str, pattern, true)
        } else if pattern.contains('*') {
            Self::matches_glob_pattern(&path_str, pattern, false)
        } else {
            path_str.contains(pattern)
        }
    }

    fn matches_glob_pattern(path_str: &str, pattern: &str, double_star: bool) -> bool {
        let cleaned = if double_star {
            pattern.replace("**", "")
        } else {
            pattern.to_string()
        };
        let parts: Vec<&str> = cleaned.split('*').filter(|p| !p.is_empty()).collect();
        parts.iter().all(|part| path_str.contains(part))
    }

    fn get_node_info<'a>(
        &self,
        ctx: &'a AnalysisContext,
        node: NodeIndex,
        layers: &'a [LayerConfig],
    ) -> Option<(&'a PathBuf, &'a LayerConfig)> {
        let path = ctx.graph.get_file_path(node)?;
        let layer = self.find_layer(path, layers)?;
        Some((path, layer))
    }

    fn check_violation(
        &self,
        from: (&PathBuf, &LayerConfig),
        to: (&PathBuf, &LayerConfig),
        edge_data: Option<&crate::graph::EdgeData>,
    ) -> Option<ArchSmell> {
        let (from_path, from_layer) = from;
        let (to_path, to_layer) = to;

        if from_layer.name != to_layer.name && !from_layer.allowed_imports.contains(&to_layer.name)
        {
            let (import_line, import_range) = edge_data
                .map(|e| (e.import_line, e.import_range))
                .unwrap_or((0, None));

            Some(ArchSmell::new_layer_violation(
                from_path.clone(),
                to_path.clone(),
                from_layer.name.clone(),
                to_layer.name.clone(),
                import_line,
                import_range,
            ))
        } else {
            None
        }
    }
}
