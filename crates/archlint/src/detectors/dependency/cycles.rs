use crate::detectors::{
    detector, ArchSmell, CriticalEdge, CycleCluster, Detector, HotspotInfo, LocationDetail,
    SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use crate::explain::ExplainEngine;
use crate::graph::DependencyGraph;
use petgraph::algo::tarjan_scc;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

#[detector(SmellType::CyclicDependency)]
pub struct CycleDetector;

impl CycleDetector {
    #[must_use]
    pub const fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn generate_cluster_report(
        output: &mut String,
        cluster_num: usize,
        cluster: &CycleCluster,
        smell: &ArchSmell,
        severity_config: &crate::config::SeverityConfig,
    ) {
        let score = smell.score(severity_config);
        output.push_str(&format!(
            "### Cluster {}: {} files ({} pts)\n\n",
            cluster_num,
            cluster.files.len(),
            score
        ));

        Self::generate_hotspots_report(output, &cluster.hotspots);
        Self::generate_critical_edges_report(output, &cluster.critical_edges);
        Self::generate_cluster_files_report(output, &cluster.files);
        Self::generate_cluster_edges_report(output, &cluster.internal_edges);
    }

    fn generate_hotspots_report(output: &mut String, hotspots: &[HotspotInfo]) {
        if hotspots.is_empty() {
            return;
        }

        output.push_str("**Hotspots (most connections):**\n\n");
        output.push_str("| File | Imports | Imported by |\n");
        output.push_str("|------|---------|-------------|\n");

        for hotspot in hotspots.iter().take(10) {
            let formatted_path = ExplainEngine::format_file_path(&hotspot.file);
            output.push_str(&format!(
                "| `{}` | {} | {} |\n",
                formatted_path, hotspot.out_degree, hotspot.in_degree
            ));
        }
        output.push('\n');
    }

    fn generate_critical_edges_report(output: &mut String, critical_edges: &[CriticalEdge]) {
        if critical_edges.is_empty() {
            return;
        }

        output.push_str("**Critical edges to break:**\n\n");
        output.push_str("| From (location) | To | Impact |\n");
        output.push_str("|-----------------|-----|--------|\n");

        for edge in critical_edges {
            let col = edge.range.map(|r| r.start_column);
            let from_loc = crate::report::format_location(&edge.from, edge.line, col);
            let to_formatted = ExplainEngine::format_file_path(&edge.to);
            output.push_str(&format!(
                "| `{}` | `{}` | {} |\n",
                from_loc, to_formatted, edge.impact
            ));
        }
        output.push('\n');
    }

    fn generate_cluster_files_report(output: &mut String, files: &[PathBuf]) {
        output.push_str("<details>\n");
        output.push_str(&format!(
            "<summary>All {} files in cluster</summary>\n\n",
            files.len()
        ));
        for file in files {
            let formatted_path = ExplainEngine::format_file_path(file);
            output.push_str(&format!("- `{formatted_path}`\n"));
        }
        output.push_str("\n</details>\n\n");
    }

    fn generate_cluster_edges_report(output: &mut String, internal_edges: &[LocationDetail]) {
        if internal_edges.is_empty() {
            return;
        }

        output.push_str("<details>\n");
        output.push_str(&format!(
            "<summary>All edges ({} imports)</summary>\n\n",
            internal_edges.len()
        ));
        output.push_str("| Location | Import |\n");
        output.push_str("|----------|--------|\n");
        for loc in internal_edges {
            let location = crate::report::format_location_detail(loc);
            output.push_str(&format!("| `{}` | {} |\n", location, loc.description));
        }
        output.push_str("\n</details>\n\n");
    }

    fn generate_legacy_cycles_report(output: &mut String, cycles: &[&&SmellWithExplanation]) {
        output.push_str("## Cyclic Dependencies\n\n");

        for (i, (smell, explanation)) in cycles.iter().enumerate() {
            output.push_str(&format!("### Cycle {}\n\n", i + 1));

            if smell.locations.is_empty() {
                let cycle_path = smell
                    .files
                    .iter()
                    .map(|p| format!("`{}`", ExplainEngine::format_file_path(p)))
                    .collect::<Vec<_>>()
                    .join(" → ");
                output.push_str(&format!("**Path:** {cycle_path}\n\n"));
            } else {
                output.push_str("**Cycle Details:**\n\n");
                output.push_str("| Location | Import |\n");
                output.push_str("|----------|--------|\n");
                for loc in &smell.locations {
                    let location = crate::report::format_location_detail(loc);
                    output.push_str(&format!("| `{}` | {} |\n", location, loc.description));
                }
                output.push('\n');
            }

            crate::report::markdown::common::append_explanation(output, explanation);
        }
    }

    #[must_use]
    pub fn detect_graph(graph: &DependencyGraph) -> Vec<ArchSmell> {
        let sccs = tarjan_scc(graph.graph());

        let cycle_sccs: Vec<_> = sccs
            .into_iter()
            .filter(|scc| scc.len() > 1)
            .filter(|scc| !Self::is_false_positive_scc(graph, scc))
            .collect();

        cycle_sccs
            .into_iter()
            .map(|scc| {
                let cluster = Self::build_cluster(graph, &scc);
                ArchSmell::new_cycle_cluster(cluster)
            })
            .collect()
    }

    fn build_cluster(graph: &DependencyGraph, scc: &[NodeIndex]) -> CycleCluster {
        let scc_set: HashSet<_> = scc.iter().copied().collect();

        let files = Self::collect_cluster_files(graph, scc);
        let mut hotspots = Self::calculate_hotspots(graph, scc, &scc_set);
        hotspots.sort_by(|a, b| {
            let a_total = a.in_degree + a.out_degree;
            let b_total = b.in_degree + b.out_degree;
            b_total.cmp(&a_total)
        });

        let internal_edges = Self::collect_internal_edges(graph, scc, &scc_set);
        let critical_edges = Self::find_critical_edges(graph, scc, &scc_set);

        CycleCluster {
            files,
            hotspots,
            critical_edges,
            internal_edges,
        }
    }

    fn collect_cluster_files(graph: &DependencyGraph, scc: &[NodeIndex]) -> Vec<PathBuf> {
        scc.iter()
            .filter_map(|&node| graph.get_file_path(node).cloned())
            .collect()
    }

    fn calculate_hotspots(
        graph: &DependencyGraph,
        scc: &[NodeIndex],
        scc_set: &HashSet<NodeIndex>,
    ) -> Vec<HotspotInfo> {
        scc.iter()
            .filter_map(|&node| {
                graph.get_file_path(node).map(|file_path| {
                    let in_degree = graph
                        .graph()
                        .neighbors_directed(node, petgraph::Direction::Incoming)
                        .filter(|n| scc_set.contains(n))
                        .count();

                    let out_degree = graph
                        .dependencies(node)
                        .filter(|n| scc_set.contains(n))
                        .count();

                    HotspotInfo {
                        file: file_path.clone(),
                        in_degree,
                        out_degree,
                    }
                })
            })
            .collect()
    }

    fn collect_internal_edges(
        graph: &DependencyGraph,
        scc: &[NodeIndex],
        scc_set: &HashSet<NodeIndex>,
    ) -> Vec<LocationDetail> {
        let mut internal_edges = Vec::new();
        for &from_node in scc {
            for to_node in graph.dependencies(from_node) {
                if scc_set.contains(&to_node) {
                    if let Some(detail) = Self::create_edge_detail(graph, from_node, to_node) {
                        internal_edges.push(detail);
                    }
                }
            }
        }
        internal_edges
    }

    fn create_edge_detail(
        graph: &DependencyGraph,
        from_node: NodeIndex,
        to_node: NodeIndex,
    ) -> Option<LocationDetail> {
        let (from_path, to_path) = (
            graph.get_file_path(from_node)?,
            graph.get_file_path(to_node)?,
        );
        let edge_data = graph.get_edge_data(from_node, to_node)?;

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
        Some(detail)
    }

    fn find_critical_edges(
        graph: &DependencyGraph,
        scc: &[NodeIndex],
        scc_set: &HashSet<NodeIndex>,
    ) -> Vec<CriticalEdge> {
        let mut edge_scores: HashMap<(NodeIndex, NodeIndex), usize> = HashMap::new();

        for &from_node in scc {
            for to_node in graph.dependencies(from_node) {
                if scc_set.contains(&to_node) {
                    let from_degree = graph.fan_in(from_node) + graph.fan_out(from_node);
                    let to_degree = graph.fan_in(to_node) + graph.fan_out(to_node);
                    let score = from_degree + to_degree;
                    edge_scores.insert((from_node, to_node), score);
                }
            }
        }

        let mut scored_edges: Vec<_> = edge_scores.iter().collect();
        scored_edges.sort_by(|a, b| b.1.cmp(a.1));

        scored_edges
            .into_iter()
            .take(5)
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

impl Detector for CycleDetector {
    crate::impl_detector_report!(
        explain: smell => (
            problem: format!("Circular dependency detected between {} files", smell.cycle_length().unwrap_or(0)),
            reason: "Files form a dependency cycle where A depends on B, B depends on C, and C depends back on A (or similar pattern). This creates tight coupling between modules.",
            risks: [
                "Difficult to reason about initialization order",
                "Changes in one module can cascade unpredictably to others",
                "Testing becomes difficult due to interdependencies",
                "Refactoring is risky and error-prone",
                "May cause compilation or runtime initialization issues"
            ],
            recommendations: [
                "Extract shared logic into a separate, independent module",
                "Use dependency injection to break direct dependencies",
                "Introduce interfaces/abstractions to invert dependencies",
                "Apply the Dependency Inversion Principle (DIP)",
                "Consider using event-driven architecture for loose coupling"
            ]
        )
    );

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        let cycle_clusters: Vec<_> = smells
            .iter()
            .filter(|(s, _)| {
                matches!(
                    s.smell_type,
                    crate::detectors::SmellType::CyclicDependencyCluster
                )
            })
            .collect();
        let cycles: Vec<_> = smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, crate::detectors::SmellType::CyclicDependency))
            .collect();

        crate::define_report_section!("Cyclic Dependencies", smells, {
            let mut body = String::new();
            let total_files: usize = cycle_clusters.iter().map(|(s, _)| s.files.len()).sum();

            if cycle_clusters.is_empty() {
                Self::generate_legacy_cycles_report(&mut body, &cycles);
            } else {
                body.push_str(&format!(
                    "(Found {} clusters, {} files)\n\n",
                    cycle_clusters.len(),
                    total_files
                ));

                for (i, (smell, _)) in cycle_clusters.iter().enumerate() {
                    if let Some(cluster) = &smell.cluster {
                        Self::generate_cluster_report(
                            &mut body,
                            i + 1,
                            cluster,
                            smell,
                            severity_config,
                        );
                    }
                }
            }
            body
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let sccs = tarjan_scc(ctx.graph.graph());

        let cycle_sccs: Vec<_> = sccs
            .into_iter()
            .filter(|scc| scc.len() > 1)
            .filter(|scc| !Self::is_false_positive_scc(ctx.graph.as_ref(), scc))
            .filter(|scc| {
                !scc.iter().any(|&node| {
                    if let Some(path) = ctx.graph.get_file_path(node) {
                        ctx.get_rule_for_file("cyclic_dependency", path).is_none()
                    } else {
                        false
                    }
                })
            })
            .collect();

        cycle_sccs
            .iter()
            .map(|scc| {
                let cluster = Self::build_cluster(ctx.graph.as_ref(), scc);
                let mut smell = ArchSmell::new_cycle_cluster(cluster);

                if let Some(node) = scc.first() {
                    if let Some(path) = ctx.graph.get_file_path(*node) {
                        if let Some(rule) = ctx.get_rule_for_file("cyclic_dependency", path) {
                            smell.severity = rule.severity;
                        }
                    }
                }
                smell
            })
            .collect()
    }
}
