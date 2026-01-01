use crate::config::Config;
use crate::detectors::{
    ArchSmell, CriticalEdge, CycleCluster, Detector, DetectorFactory, DetectorInfo, HotspotInfo,
    LocationDetail,
};
use crate::engine::AnalysisContext;
use crate::explain::ExplainEngine;
use crate::graph::DependencyGraph;
use inventory;
use petgraph::algo::tarjan_scc;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub fn init() {}

pub struct CycleDetector;

pub struct CycleDetectorFactory;

impl DetectorFactory for CycleDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "cycles",
            name: "Cycle Detector",
            description: "Detects circular dependencies between modules",
            default_enabled: true,
            is_deep: false,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(CycleDetector)
    }
}

inventory::submit! {
    &CycleDetectorFactory as &dyn DetectorFactory
}

impl Detector for CycleDetector {
    fn name(&self) -> &'static str {
        "Cycles"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let thresholds = &ctx.config.thresholds.cycles;

        // Use Tarjan's SCC algorithm to find strongly connected components
        let sccs = tarjan_scc(ctx.graph.graph());

        // Filter SCCs with size > 1 (these are cycles)
        let cycle_sccs: Vec<_> = sccs
            .into_iter()
            .filter(|scc| scc.len() > 1)
            .filter(|scc| !Self::is_false_positive_scc(&ctx.graph, scc))
            .filter(|scc| {
                // Check if any file in SCC is excluded by cycles.exclude_patterns
                // or if it should be skipped due to framework context
                !scc.iter().any(|&node| {
                    if let Some(path) = ctx.graph.get_file_path(node) {
                        ctx.is_excluded(path, &thresholds.exclude_patterns)
                            || ctx.should_skip_detector(path, "cycles")
                    } else {
                        false
                    }
                })
            })
            .collect();

        cycle_sccs
            .iter()
            .map(|scc| {
                let cluster = Self::build_cluster(&ctx.graph, scc);
                ArchSmell::new_cycle_cluster(cluster)
            })
            .collect()
    }
}

impl CycleDetector {
    pub fn detect_graph(graph: &DependencyGraph) -> Vec<ArchSmell> {
        // Use Tarjan's SCC algorithm to find strongly connected components
        let sccs = tarjan_scc(graph.graph());

        // Filter SCCs with size > 1 (these are cycles)
        let cycle_sccs: Vec<_> = sccs
            .into_iter()
            .filter(|scc| scc.len() > 1)
            .filter(|scc| !Self::is_false_positive_scc(graph, scc))
            .collect();

        cycle_sccs
            .iter()
            .map(|scc| {
                let cluster = Self::build_cluster(graph, scc);
                ArchSmell::new_cycle_cluster(cluster)
            })
            .collect()
    }

    fn build_cluster(graph: &DependencyGraph, scc: &[NodeIndex]) -> CycleCluster {
        let scc_set: HashSet<_> = scc.iter().copied().collect();

        // 1. Collect all files
        let files: Vec<PathBuf> = scc
            .iter()
            .filter_map(|&node| graph.get_file_path(node).cloned())
            .collect();

        // 2. Calculate hotspots (in/out degree within SCC)
        let mut hotspots = Vec::new();
        for &node in scc {
            if let Some(file_path) = graph.get_file_path(node) {
                let in_degree = graph
                    .graph()
                    .neighbors_directed(node, petgraph::Direction::Incoming)
                    .filter(|n| scc_set.contains(n))
                    .count();

                let out_degree = graph
                    .dependencies(node)
                    .filter(|n| scc_set.contains(n))
                    .count();

                hotspots.push(HotspotInfo {
                    file: file_path.clone(),
                    in_degree,
                    out_degree,
                });
            }
        }

        // Sort hotspots by total degree (descending)
        hotspots.sort_by(|a, b| {
            let a_total = a.in_degree + a.out_degree;
            let b_total = b.in_degree + b.out_degree;
            b_total.cmp(&a_total)
        });

        // 3. Collect all internal edges
        let mut internal_edges = Vec::new();
        for &from_node in scc {
            for to_node in graph.dependencies(from_node) {
                if scc_set.contains(&to_node) {
                    if let (Some(from_path), Some(to_path)) =
                        (graph.get_file_path(from_node), graph.get_file_path(to_node))
                    {
                        if let Some(edge_data) = graph.get_edge_data(from_node, to_node) {
                            let symbols_str = if edge_data.imported_symbols.is_empty() {
                                String::new()
                            } else {
                                format!(" ({})", edge_data.imported_symbols.join(", "))
                            };

                            let mut detail = LocationDetail::new(
                                from_path.clone(),
                                edge_data.import_line,
                                format!(
                                    "imports from '{}'{}",
                                    ExplainEngine::format_file_path(to_path),
                                    symbols_str
                                ),
                            );
                            if let Some(range) = edge_data.import_range {
                                detail = detail.with_range(range);
                            }
                            internal_edges.push(detail);
                        }
                    }
                }
            }
        }

        // 4. Find critical edges (edges with high centrality/impact)
        let critical_edges = Self::find_critical_edges(graph, scc, &scc_set);

        CycleCluster {
            files,
            hotspots,
            critical_edges,
            internal_edges,
        }
    }

    fn find_critical_edges(
        graph: &DependencyGraph,
        scc: &[NodeIndex],
        scc_set: &HashSet<NodeIndex>,
    ) -> Vec<CriticalEdge> {
        // Calculate edge importance based on node degrees
        let mut edge_scores: HashMap<(NodeIndex, NodeIndex), usize> = HashMap::new();

        for &from_node in scc {
            for to_node in graph.dependencies(from_node) {
                if scc_set.contains(&to_node) {
                    // Score is sum of degrees (higher degree nodes = more critical)
                    let from_degree = graph.fan_in(from_node) + graph.fan_out(from_node);
                    let to_degree = graph.fan_in(to_node) + graph.fan_out(to_node);
                    let score = from_degree + to_degree;
                    edge_scores.insert((from_node, to_node), score);
                }
            }
        }

        // Sort edges by score and take top ones
        let mut scored_edges: Vec<_> = edge_scores.iter().collect();
        scored_edges.sort_by(|a, b| b.1.cmp(a.1));

        scored_edges
            .iter()
            .take(5) // Top 5 critical edges
            .filter_map(|((from_node, to_node), &score)| {
                let from_path = graph.get_file_path(*from_node)?;
                let to_path = graph.get_file_path(*to_node)?;
                let edge_data = graph.get_edge_data(*from_node, *to_node)?;

                let impact = if score > 50 {
                    "High centrality".to_string()
                } else if score > 20 {
                    "Medium centrality".to_string()
                } else {
                    "Low centrality".to_string()
                };

                Some(CriticalEdge {
                    from: from_path.clone(),
                    to: to_path.clone(),
                    line: edge_data.import_line,
                    range: edge_data.import_range,
                    impact,
                })
            })
            .collect()
    }

    fn is_false_positive_scc(graph: &DependencyGraph, scc: &[NodeIndex]) -> bool {
        // Check if any file in the SCC is a test file
        scc.iter().any(|&node| {
            if let Some(path) = graph.get_file_path(node) {
                Self::is_test_file(path)
            } else {
                false
            }
        })
    }

    fn is_test_file(path: &Path) -> bool {
        let s = path.to_string_lossy().to_lowercase();
        s.contains("/test/")
            || s.contains("/tests/")
            || s.contains("/__tests__/")
            || s.contains("/__fixtures__/")
            || s.contains("/__mocks__/")
            || s.contains(".test.")
            || s.contains(".spec.")
            || s.contains(".fixture.")
            || s.contains(".mock.")
    }
}
