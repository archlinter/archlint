use super::common::{append_explanation, SmellWithExplanation};
use crate::config::SeverityConfig;
use crate::explain::ExplainEngine;
use crate::graph::DependencyGraph;
use std::path::{Path, PathBuf};

pub fn generate(
    gods: &[&SmellWithExplanation],
    graph: Option<&DependencyGraph>,
    severity_config: &SeverityConfig,
) -> String {
    if gods.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str("## God Modules\n\n");

    for (smell, explanation) in gods {
        let file_path = smell
            .files
            .first()
            .cloned()
            .unwrap_or_else(|| PathBuf::from("Unknown"));

        let formatted_path = ExplainEngine::format_file_path(&file_path);
        let score = smell.score(severity_config);

        output.push_str(&format!("### {} ({} pts)\n\n", formatted_path, score));

        generate_metrics(&mut output, smell);
        generate_dependencies(&mut output, &file_path, graph);
        append_explanation(&mut output, explanation);
    }

    output
}

fn generate_metrics(output: &mut String, smell: &crate::detectors::ArchSmell) {
    output.push_str("**Metrics:**\n");
    if let Some(fan_in) = smell.fan_in() {
        output.push_str(&format!("- Fan-in: {}\n", fan_in));
    }
    if let Some(fan_out) = smell.fan_out() {
        output.push_str(&format!("- Fan-out: {}\n", fan_out));
    }
    if let Some(churn) = smell.churn() {
        output.push_str(&format!("- Churn: {} commits\n", churn));
    }
    output.push('\n');
}

fn generate_dependencies(output: &mut String, file_path: &Path, graph: Option<&DependencyGraph>) {
    let Some(graph) = graph else { return };
    let Some(node_idx) = graph.get_node(file_path) else {
        return;
    };

    let mut incoming: Vec<_> = graph
        .graph()
        .neighbors_directed(node_idx, petgraph::Direction::Incoming)
        .filter_map(|idx| graph.get_file_path(idx))
        .map(|p| ExplainEngine::format_file_path(p))
        .collect();

    if !incoming.is_empty() {
        incoming.sort();
        output.push_str("**Used by:**\n");
        for p in incoming.iter().take(15) {
            output.push_str(&format!("- `{}`\n", p));
        }
        if incoming.len() > 15 {
            output.push_str(&format!("- ... and {} more\n", incoming.len() - 15));
        }
        output.push('\n');
    }

    let mut outgoing: Vec<_> = graph
        .dependencies(node_idx)
        .filter_map(|idx| graph.get_file_path(idx))
        .map(|p| ExplainEngine::format_file_path(p))
        .collect();

    if !outgoing.is_empty() {
        outgoing.sort();
        output.push_str("**Depends on:**\n");
        for p in outgoing.iter().take(15) {
            output.push_str(&format!("- `{}`\n", p));
        }
        if outgoing.len() > 15 {
            output.push_str(&format!("- ... and {} more\n", outgoing.len() - 15));
        }
        output.push('\n');
    }
}
