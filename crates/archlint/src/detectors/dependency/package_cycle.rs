use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;
use petgraph::graph::DiGraph;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(
    id = "package_cycles",
    name = "Package-level Cycle Detector",
    description = "Detects circular dependencies between logical folders (packages)",
    category = DetectorCategory::GraphBased,
    default_enabled = false
)]
pub struct PackageCycleDetector;

impl PackageCycleDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn build_package_graph(
        &self,
        ctx: &AnalysisContext,
        package_depth: usize,
    ) -> DiGraph<String, ()> {
        let mut pkg_graph = DiGraph::<String, ()>::new();
        let mut pkg_to_node = HashMap::new();
        let mut processed_edges = HashSet::new();

        for (from_idx, to_idx) in ctx.graph.edges() {
            if let Some((from_pkg, to_pkg)) =
                self.get_package_pair(ctx, from_idx, to_idx, package_depth)
            {
                if from_pkg != to_pkg {
                    self.add_package_edge(
                        &mut pkg_graph,
                        &mut pkg_to_node,
                        &mut processed_edges,
                        from_pkg,
                        to_pkg,
                    );
                }
            }
        }

        pkg_graph
    }

    fn get_package_pair(
        &self,
        ctx: &AnalysisContext,
        from_idx: petgraph::graph::NodeIndex,
        to_idx: petgraph::graph::NodeIndex,
        package_depth: usize,
    ) -> Option<(String, String)> {
        let from_path = ctx.graph.get_file_path(from_idx)?;
        let to_path = ctx.graph.get_file_path(to_idx)?;

        let from_pkg = self.get_package_name(from_path, &ctx.project_path, package_depth);
        let to_pkg = self.get_package_name(to_path, &ctx.project_path, package_depth);

        Some((from_pkg, to_pkg))
    }

    fn add_package_edge(
        &self,
        pkg_graph: &mut DiGraph<String, ()>,
        pkg_to_node: &mut HashMap<String, petgraph::graph::NodeIndex>,
        processed_edges: &mut HashSet<(petgraph::graph::NodeIndex, petgraph::graph::NodeIndex)>,
        from_pkg: String,
        to_pkg: String,
    ) {
        let from_node = *pkg_to_node
            .entry(from_pkg.clone())
            .or_insert_with(|| pkg_graph.add_node(from_pkg));
        let to_node = *pkg_to_node
            .entry(to_pkg.clone())
            .or_insert_with(|| pkg_graph.add_node(to_pkg));

        if processed_edges.insert((from_node, to_node)) {
            pkg_graph.add_edge(from_node, to_node, ());
        }
    }

    fn find_package_cycles(&self, pkg_graph: &DiGraph<String, ()>) -> Vec<ArchSmell> {
        let sccs = petgraph::algo::tarjan_scc(pkg_graph);
        let mut smells = Vec::new();

        for scc in sccs {
            if scc.len() > 1 {
                let packages: Vec<String> = scc.iter().map(|&idx| pkg_graph[idx].clone()).collect();
                smells.push(ArchSmell::new_package_cycle(packages));
            } else if scc.len() == 1 && pkg_graph.contains_edge(scc[0], scc[0]) {
                let packages = vec![pkg_graph[scc[0]].clone()];
                smells.push(ArchSmell::new_package_cycle(packages));
            }
        }

        smells
    }

    fn get_package_name(&self, path: &Path, project_root: &Path, depth: usize) -> String {
        let dir = if path.is_file() {
            path.parent().unwrap_or(path)
        } else {
            path
        };

        let rel_path = dir.strip_prefix(project_root).unwrap_or(dir);
        let components: Vec<_> = rel_path
            .components()
            .take(depth)
            .filter_map(|c| c.as_os_str().to_str())
            .collect();

        if components.is_empty() {
            "root".to_string()
        } else {
            components.join("/")
        }
    }
}

impl Detector for PackageCycleDetector {
    crate::impl_detector_report!(
        name: "PackageCycle",
        explain: _smell => {
            crate::detectors::Explanation {
                problem: "Package-level Cycle".into(),
                reason: "Circular dependency detected between different packages/folders. This violates the goal of creating a hierarchical, directed dependency graph between logical components.".into(),
                risks: crate::strings![
                    "Packages cannot be developed or deployed in isolation",
                    "Modular structure becomes a big ball of mud"
                ],
                recommendations: crate::strings![
                    "Move shared code to a lower-level package or use abstractions to break the cycle"
                ]
            }
        },
        table: {
            title: "Package Cycles",
            columns: ["Cycle Path", "pts"],
            row: PackageCycle { packages } (smell, location, pts) => [
                packages.join(" → "),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let rule = match ctx.get_rule("package_cycles") {
            Some(r) => r,
            None => return Vec::new(),
        };

        let package_depth: usize = rule.get_option("package_depth").unwrap_or(2);

        let pkg_graph = self.build_package_graph(ctx, package_depth);
        let smells = self.find_package_cycles(&pkg_graph);

        smells
            .into_iter()
            .map(|mut s| {
                s.severity = rule.severity;
                s
            })
            .collect()
    }
}
