use super::common::{append_explanation, SmellWithExplanation};
use crate::config::SeverityConfig;
use crate::explain::ExplainEngine;
use crate::report::{format_location, format_location_detail};

pub fn generate(
    cycles: &[&SmellWithExplanation],
    cycle_clusters: &[&SmellWithExplanation],
    severity_config: &SeverityConfig,
) -> String {
    if cycles.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    let total_files: usize = cycle_clusters.iter().map(|(s, _)| s.files.len()).sum();

    if !cycle_clusters.is_empty() {
        output.push_str(&format!(
            "## Cyclic Dependencies ({} clusters, {} files)\n\n",
            cycle_clusters.len(),
            total_files
        ));

        if let Some((_, explanation)) = cycle_clusters.first() {
            append_explanation(&mut output, explanation);
            output.push_str("---\n\n");
        }

        for (i, (smell, _)) in cycle_clusters.iter().enumerate() {
            if let Some(cluster) = &smell.cluster {
                generate_cluster(&mut output, i + 1, cluster, smell, severity_config);
            }
        }
    } else {
        generate_legacy_cycles(&mut output, cycles);
    }

    output
}

fn generate_cluster(
    output: &mut String,
    cluster_num: usize,
    cluster: &crate::detectors::CycleCluster,
    smell: &crate::detectors::ArchSmell,
    severity_config: &SeverityConfig,
) {
    let score = smell.score(severity_config);
    output.push_str(&format!(
        "### Cluster {}: {} files ({} pts)\n\n",
        cluster_num,
        cluster.files.len(),
        score
    ));

    generate_hotspots(output, &cluster.hotspots);
    generate_critical_edges(output, &cluster.critical_edges);
    generate_cluster_files(output, &cluster.files);
    generate_cluster_edges(output, &cluster.internal_edges);
}

fn generate_hotspots(output: &mut String, hotspots: &[crate::detectors::HotspotInfo]) {
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

fn generate_critical_edges(output: &mut String, critical_edges: &[crate::detectors::CriticalEdge]) {
    if critical_edges.is_empty() {
        return;
    }

    output.push_str("**Critical edges to break:**\n\n");
    output.push_str("| From (location) | To | Impact |\n");
    output.push_str("|-----------------|-----|--------|\n");

    for edge in critical_edges {
        let col = edge.range.map(|r| r.start_column);
        let from_loc = format_location(&edge.from, edge.line, col);
        let to_formatted = ExplainEngine::format_file_path(&edge.to);
        output.push_str(&format!(
            "| `{}` | `{}` | {} |\n",
            from_loc, to_formatted, edge.impact
        ));
    }
    output.push('\n');
}

fn generate_cluster_files(output: &mut String, files: &[std::path::PathBuf]) {
    output.push_str("<details>\n");
    output.push_str(&format!(
        "<summary>All {} files in cluster</summary>\n\n",
        files.len()
    ));
    for file in files {
        let formatted_path = ExplainEngine::format_file_path(file);
        output.push_str(&format!("- `{}`\n", formatted_path));
    }
    output.push_str("\n</details>\n\n");
}

fn generate_cluster_edges(
    output: &mut String,
    internal_edges: &[crate::detectors::LocationDetail],
) {
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
        let location = format_location_detail(loc);
        output.push_str(&format!("| `{}` | {} |\n", location, loc.description));
    }
    output.push_str("\n</details>\n\n");
}

fn generate_legacy_cycles(output: &mut String, cycles: &[&SmellWithExplanation]) {
    output.push_str("## Cyclic Dependencies\n\n");

    for (i, (smell, explanation)) in cycles.iter().enumerate() {
        output.push_str(&format!("### Cycle {}\n\n", i + 1));

        if !smell.locations.is_empty() {
            output.push_str("**Cycle Details:**\n\n");
            output.push_str("| Location | Import |\n");
            output.push_str("|----------|--------|\n");
            for loc in &smell.locations {
                let location = format_location_detail(loc);
                output.push_str(&format!("| `{}` | {} |\n", location, loc.description));
            }
            output.push('\n');
        } else {
            let cycle_path = smell
                .files
                .iter()
                .map(|p| format!("`{}`", ExplainEngine::format_file_path(p)))
                .collect::<Vec<_>>()
                .join(" → ");
            output.push_str(&format!("**Path:** {}\n\n", cycle_path));
        }

        append_explanation(output, explanation);
    }
}
