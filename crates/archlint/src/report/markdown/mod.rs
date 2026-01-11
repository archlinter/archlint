pub mod common;
#[macro_use]
pub mod macros;
mod summary;

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

    let grouped = common::group_smells_by_detector(&report.smells);
    let registry = crate::detectors::DetectorRegistry::new();

    let report_order = vec![
        "cycles",
        "god_module",
        "dead_code",
        "dead_symbols",
        "complexity",
        "large_file",
        "unstable_interface",
        "feature_envy",
        "shotgun_surgery",
        "hub_dependency",
        "barrel_file",
        "vendor_coupling",
        "side_effect_import",
        "hub_module",
        "lcom",
        "module_cohesion",
        "high_coupling",
        "package_cycles",
        "shared_mutable_state",
        "deep_nesting",
        "long_params",
        "primitive_obsession",
        "orphan_types",
        "circular_type_deps",
        "abstractness",
        "scattered_config",
        "code_clone",
        "test_leakage",
        "layer_violation",
        "sdp_violation",
    ];

    for detector_id in report_order {
        if let Some(smells) = grouped.get(detector_id) {
            if let Some(detector) = registry.create_detector(detector_id, &report.config) {
                output.push_str(&detector.render_markdown(smells, severity_config, graph));
            }
        }
    }

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
