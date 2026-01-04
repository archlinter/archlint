mod common;
mod complexity;
mod cycles;
mod dead_code;
mod dead_symbols;
mod envy;
mod god_modules;
mod hub_deps;
mod large_files;
mod surgery;
mod summary;
mod unstable;

pub use common::filter_smells;

use crate::config::SeverityConfig;
use crate::graph::DependencyGraph;
use crate::report::AnalysisReport;
use crate::Result;
use std::fs;
use std::path::Path;

pub fn generate_markdown(
    report: &AnalysisReport,
    graph: Option<&DependencyGraph>,
    include_diagram: bool,
    severity_config: &SeverityConfig,
) -> String {
    let mut output = summary::generate(report, severity_config);

    if report.smells.is_empty() {
        output.push_str("✅ No architectural smells detected!\n");
        return output;
    }

    if include_diagram {
        if let Some(graph) = graph {
            output.push_str("## Dependency Graph (Problem Files)\n\n");
            let diagram = super::mermaid::MermaidGenerator::generate(&report.smells, graph);
            if !diagram.is_empty() {
                output.push_str(&diagram);
                output.push('\n');
            }
        }
    }

    let filtered = filter_smells(&report.smells);

    output.push_str(&cycles::generate(&filtered.cycles, &filtered.cycle_clusters, severity_config));
    output.push_str(&god_modules::generate(&filtered.gods, graph, severity_config));
    output.push_str(&dead_code::generate(&filtered.dead));
    output.push_str(&dead_symbols::generate(&filtered.dead_symbols));
    output.push_str(&complexity::generate(&filtered.high_complexity, severity_config));
    output.push_str(&large_files::generate(&filtered.large_files, severity_config));
    output.push_str(&unstable::generate(&filtered.unstable_interfaces, severity_config));
    output.push_str(&envy::generate(&filtered.feature_envy, severity_config));
    output.push_str(&surgery::generate(&filtered.shotgun_surgery, severity_config));
    output.push_str(&hub_deps::generate(&filtered.hub_dependencies, severity_config));

    output
}

pub fn write_report<P: AsRef<Path>>(
    report: &AnalysisReport,
    path: P,
    graph: Option<&DependencyGraph>,
    include_diagram: bool,
    severity_config: &SeverityConfig,
) -> Result<()> {
    let output = generate_markdown(report, graph, include_diagram, severity_config);
    fs::write(path, output)?;
    Ok(())
}
