use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use petgraph::graph::DiGraph;
use std::collections::HashMap;

pub fn init() {}

pub struct CircularTypeDepsDetector;

pub struct CircularTypeDepsDetectorFactory;

impl DetectorFactory for CircularTypeDepsDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "circular_type_deps",
            name: "Circular Type Dependencies Detector",
            description: "Detects circular dependencies between modules that only involve types",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::GraphBased,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(CircularTypeDepsDetector)
    }
}

inventory::submit! {
    &CircularTypeDepsDetectorFactory as &dyn DetectorFactory
}

impl Detector for CircularTypeDepsDetector {
    fn name(&self) -> &'static str {
        "CircularTypeDependencies"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        let mut type_graph = DiGraph::<std::path::PathBuf, ()>::new();
        let mut path_to_node = HashMap::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let rule = ctx.resolve_rule("circular_type_deps", Some(path));
            if !rule.enabled || ctx.is_excluded(path, &rule.exclude) {
                continue;
            }

            let from_node = *path_to_node
                .entry(path.clone())
                .or_insert_with(|| type_graph.add_node(path.clone()));

            for import in &symbols.imports {
                if import.is_type_only {
                    if let Some(target_path) = self.resolve_import(import, path, ctx) {
                        let to_node = *path_to_node
                            .entry(target_path.clone())
                            .or_insert_with(|| type_graph.add_node(target_path));
                        type_graph.add_edge(from_node, to_node, ());
                    }
                }
            }
        }

        let sccs = petgraph::algo::tarjan_scc(&type_graph);
        for scc in sccs {
            if scc.len() > 1 {
                let files: Vec<_> = scc.iter().map(|&idx| type_graph[idx].clone()).collect();

                // Get severity from the first file in cycle
                let severity = if let Some(path) = files.first() {
                    ctx.resolve_rule("circular_type_deps", Some(path)).severity
                } else {
                    crate::detectors::Severity::Low
                };

                smells.push(ArchSmell {
                    smell_type: crate::detectors::SmellType::CircularTypeDependency,
                    severity,
                    files,
                    metrics: Vec::new(),
                    locations: Vec::new(),
                    cluster: None,
                });
            }
        }

        smells
    }
}

impl CircularTypeDepsDetector {
    fn resolve_import(
        &self,
        import: &crate::parser::ImportedSymbol,
        from: &std::path::Path,
        ctx: &AnalysisContext,
    ) -> Option<std::path::PathBuf> {
        let node_idx = ctx.graph.get_node(from)?;
        for target_node in ctx.graph.dependencies(node_idx) {
            if let Some(target_path) = ctx.graph.get_file_path(target_node) {
                // If it's a relative import, check if target_path matches resolved version
                // For now, let's use a simpler check: does the target_path match the import.source
                // when resolved relative to 'from'?

                // We can't easily resolve here without PathResolver,
                // but we know that an edge already exists in ctx.graph.
                // We just need to find WHICH edge corresponds to this specific import.

                // Since we don't store resolved path in ImportedSymbol yet,
                // we'll use a slightly better heuristic.
                let target_str = target_path.to_string_lossy();
                let source_parts: Vec<&str> = import
                    .source
                    .split('/')
                    .filter(|s| !s.is_empty() && *s != "." && *s != "..")
                    .collect();

                if source_parts.iter().all(|part| target_str.contains(part)) {
                    return Some(target_path.clone());
                }
            }
        }
        None
    }
}
