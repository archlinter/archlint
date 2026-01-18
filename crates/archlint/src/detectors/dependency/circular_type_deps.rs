use crate::detectors::{detector, ArchSmell, Detector, Severity, SmellType};
use crate::engine::AnalysisContext;
use crate::parser::ImportedSymbol;
use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

#[detector(SmellType::CircularTypeDependency, default_enabled = false)]
pub struct CircularTypeDepsDetector;

impl CircularTypeDepsDetector {
    #[must_use]
    pub const fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

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
                if self.path_matches_source(target_path, &import.source) {
                    return Some(target_path.clone());
                }
            }
        }
        None
    }

    fn path_matches_source(&self, target_path: &Path, source: &str) -> bool {
        let source_normalized = source.replace('\\', "/");
        let source_parts: Vec<&str> = source_normalized
            .split('/')
            .filter(|s| !s.is_empty() && *s != "." && *s != "..")
            .collect();

        if source_parts.is_empty() {
            return false;
        }

        let mut current_target = target_path;
        for part in source_parts.iter().rev() {
            let matches_part = match current_target.file_name().and_then(|n| n.to_str()) {
                Some(file_name) => file_name == *part || file_name.starts_with(&format!("{part}.")),
                None => false,
            };

            if !matches_part {
                return false;
            }

            if let Some(parent) = current_target.parent() {
                current_target = parent;
            } else {
                return source_parts.len() == 1 || *part == source_parts[0];
            }
        }
        true
    }
}

impl Detector for CircularTypeDepsDetector {
    crate::impl_detector_report!(
        explain: smell => (
            problem: "Circular Type Dependency",
            reason: "Two or more modules have a circular dependency that only involves types (type-only imports). While allowed by some compilers, it often indicates a flaw in module design.",
            risks: [
                "Difficult to reason about data structures",
                "Tight coupling between types"
            ],
            recommendations: [
                "Refactor shared types into a dedicated common module"
            ]
        ),
        table: {
            title: "Circular Type Dependencies",
            columns: ["Cycle Path", "pts"],
            row: CircularTypeDependency { } (smell, location, pts) => [
                smell.files.iter().map(|p| crate::explain::ExplainEngine::format_file_path(p)).collect::<Vec<_>>().join(" → "),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let type_graph = self.build_type_graph(ctx);
        self.process_sccs(&type_graph, ctx)
    }
}
