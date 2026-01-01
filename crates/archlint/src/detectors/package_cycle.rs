use crate::config::Config;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use petgraph::graph::DiGraph;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn init() {}

pub struct PackageCycleDetector;

pub struct PackageCycleDetectorFactory;

impl DetectorFactory for PackageCycleDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "package_cycle",
            name: "Package-level Cycle Detector",
            description: "Detects circular dependencies between logical folders (packages)",
            default_enabled: false,
            is_deep: false,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(PackageCycleDetector)
    }
}

inventory::submit! {
    &PackageCycleDetectorFactory as &dyn DetectorFactory
}

impl Detector for PackageCycleDetector {
    fn name(&self) -> &'static str {
        "PackageCycle"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let thresholds = &ctx.config.thresholds.package_cycles;
        let mut smells = Vec::new();

        // 1. Build package graph
        let mut pkg_graph = DiGraph::<String, ()>::new();
        let mut pkg_to_node = HashMap::new();
        let mut processed_edges = HashSet::new();

        for (from_idx, to_idx) in ctx.graph.edges() {
            let from_path = match ctx.graph.get_file_path(from_idx) {
                Some(p) => p,
                None => continue,
            };
            let to_path = match ctx.graph.get_file_path(to_idx) {
                Some(p) => p,
                None => continue,
            };

            let from_pkg =
                self.get_package_name(from_path, &ctx.project_path, thresholds.package_depth);
            let to_pkg =
                self.get_package_name(to_path, &ctx.project_path, thresholds.package_depth);

            if from_pkg != to_pkg {
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
        }

        // 2. Find cycles in package graph
        let sccs = petgraph::algo::tarjan_scc(&pkg_graph);
        for scc in sccs {
            if scc.len() > 1 {
                let packages: Vec<String> = scc.iter().map(|&idx| pkg_graph[idx].clone()).collect();
                smells.push(ArchSmell::new_package_cycle(packages));
            } else if scc.len() == 1 {
                // Check for self-cycle
                if pkg_graph.contains_edge(scc[0], scc[0]) {
                    let packages = vec![pkg_graph[scc[0]].clone()];
                    smells.push(ArchSmell::new_package_cycle(packages));
                }
            }
        }

        smells
    }
}

impl PackageCycleDetector {
    fn get_package_name(&self, path: &Path, project_root: &Path, depth: usize) -> String {
        // Use parent directory if path is a file
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
