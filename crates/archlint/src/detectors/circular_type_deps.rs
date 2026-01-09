use crate::config::Config;
use crate::detectors::{
    ArchSmell, Detector, DetectorCategory, DetectorFactory, DetectorInfo, Severity, SmellType,
};
use crate::engine::AnalysisContext;
use crate::parser::ImportedSymbol;
use inventory;
use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
        let type_graph = self.build_type_graph(ctx);
        self.process_sccs(&type_graph, ctx)
    }
}

impl CircularTypeDepsDetector {
    fn build_type_graph(&self, ctx: &AnalysisContext) -> DiGraph<PathBuf, ()> {
        let mut type_graph = DiGraph::new();
        let mut path_to_node = HashMap::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let _rule = match ctx.get_rule_for_file("circular_type_deps", path) {
                Some(r) => r,
                None => continue,
            };

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
        type_graph
    }

    fn process_sccs(
        &self,
        type_graph: &DiGraph<PathBuf, ()>,
        ctx: &AnalysisContext,
    ) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let sccs = petgraph::algo::tarjan_scc(type_graph);

        for scc in sccs {
            if scc.len() > 1 {
                let files: Vec<_> = scc.iter().map(|&idx| type_graph[idx].clone()).collect();
                let severity = self.get_severity(&files, ctx);

                smells.push(ArchSmell {
                    smell_type: SmellType::CircularTypeDependency,
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

    fn get_severity(&self, files: &[PathBuf], ctx: &AnalysisContext) -> Severity {
        files
            .iter()
            .map(|path| ctx.resolve_rule("circular_type_deps", Some(path)).severity)
            .max()
            .unwrap_or(Severity::Low)
    }

    fn resolve_import(
        &self,
        import: &ImportedSymbol,
        from: &Path,
        ctx: &AnalysisContext,
    ) -> Option<PathBuf> {
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
                let source_normalized = import.source.replace('\\', "/");
                let source_parts: Vec<&str> = source_normalized
                    .split('/')
                    .filter(|s| !s.is_empty() && *s != "." && *s != "..")
                    .collect();

                if !source_parts.is_empty()
                    && source_parts.iter().all(|part| target_str.contains(part))
                {
                    // Check if it ends with the components to be more precise
                    let mut matches = true;
                    let mut current_target = target_path.as_path();
                    for part in source_parts.iter().rev() {
                        if let Some(file_name) = current_target.file_name().and_then(|n| n.to_str())
                        {
                            if !file_name.starts_with(part) {
                                matches = false;
                                break;
                            }
                        }
                        if let Some(parent) = current_target.parent() {
                            current_target = parent;
                        } else {
                            break;
                        }
                    }

                    if matches {
                        return Some(target_path.clone());
                    }
                }
            }
        }
        None
    }
}
