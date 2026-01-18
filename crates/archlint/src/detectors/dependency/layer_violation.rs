use crate::config::LayerConfig;
use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use petgraph::graph::NodeIndex;
use std::path::{Path, PathBuf};

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

#[detector(SmellType::LayerViolation, default_enabled = false)]
pub struct LayerViolationDetector;

impl LayerViolationDetector {
    #[must_use]
    pub const fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn check_dependencies_for_violations(
        &self,
        ctx: &AnalysisContext,
        from_info: (&PathBuf, &LayerConfig),
        layers: &[LayerConfig],
        _global_rule: &crate::rule_resolver::ResolvedRuleConfig,
    ) -> Vec<ArchSmell> {
        let (from_path, _) = from_info;
        let mut smells = Vec::new();

        if let Some(node) = ctx.graph.get_node(from_path) {
            let rule = match ctx.get_rule_for_file("layer_violation", from_path) {
                Some(r) => r,
                None => return Vec::new(),
            };

            for to_node in ctx.graph.dependencies(node) {
                if let Some(to_info) = self.get_node_info(ctx, to_node, layers) {
                    let edge_data = ctx.graph.get_edge_data(node, to_node);
                    if let Some(mut smell) = self.check_violation(from_info, to_info, edge_data) {
                        smell.severity = rule.severity;
                        smells.push(smell);
                    }
                }
            }
        }

        smells
    }

    fn find_layer<'a>(&self, path: &Path, layers: &'a [LayerConfig]) -> Option<&'a LayerConfig> {
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
            let (import_line, import_range) =
                edge_data.map_or((0, None), |e| (e.import_line, e.import_range));

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

impl Detector for LayerViolationDetector {
    crate::impl_detector_report!(
        explain: smell => (
            problem: {
                let (from, to) = if let crate::detectors::SmellType::LayerViolation { from_layer, to_layer } = &smell.smell_type {
                    (from_layer.as_str(), to_layer.as_str())
                } else {
                    ("unknown", "unknown")
                };
                format!("Layer Architecture Violation: {from} → {to}")
            },
            reason: "A module in one layer imports a module from a layer it shouldn't know about (e.g., domain depending on infrastructure).",
            risks: [
                "Circular dependencies between layers",
                "Difficult to test domain logic in isolation",
                "Leaking implementation details into business logic"
            ],
            recommendations: [
                "Use Dependency Inversion Principle (DIP)",
                "Introduce interfaces in the stable layer",
                "Move the code to the appropriate layer"
            ]
        ),
        table: {
            title: "Layer Violations",
            columns: ["Location", "Violation", "pts"],
            row: LayerViolation { from_layer, to_layer } (smell, location, pts) => [
                location,
                format!("`{}` → `{}`", from_layer, to_layer),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let rule = match ctx.get_rule("layer_violation") {
            Some(r) => r,
            None => return Vec::new(),
        };

        let layers: Vec<LayerConfig> = rule.get_option("layers").unwrap_or_default();

        if layers.is_empty() {
            return Vec::new();
        }

        ctx.graph
            .nodes()
            .filter_map(|node| self.get_node_info(ctx, node, &layers))
            .flat_map(|from_info| {
                self.check_dependencies_for_violations(ctx, from_info, &layers, &rule)
            })
            .collect()
    }
}
