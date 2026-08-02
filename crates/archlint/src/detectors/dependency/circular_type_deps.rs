use crate::detectors::{detector, ArchSmell, Detector, Severity, SmellType};
use crate::engine::AnalysisContext;
use crate::parser::ImportedSymbol;
use petgraph::graph::DiGraph;
use std::collections::HashMap;
use std::path::PathBuf;

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
                    if let Some(target_path) = Self::resolve_type_import(import, ctx) {
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

    /// Resolves a type-only import to the project file it points at.
    ///
    /// Deliberately does not go through `ctx.graph`: the graph only holds files with
    /// runtime code (see `AnalysisEngine::get_runtime_files`), and a cycle between
    /// pure type declarations — the case this detector exists for — has no such file
    /// on either end. `import.source` is already an absolute path by this point
    /// (`EngineBuilder::resolve_symbols`); imports that failed to resolve keep their
    /// raw specifier and simply are not known project files.
    fn resolve_type_import(import: &ImportedSymbol, ctx: &AnalysisContext) -> Option<PathBuf> {
        let target = PathBuf::from(import.source.as_str());
        ctx.file_symbols.contains_key(&target).then_some(target)
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
