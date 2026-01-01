use crate::config::SeverityConfig;
use crate::detectors::{Severity, SmellType};
use crate::explain::ExplainEngine;
use crate::graph::DependencyGraph;
use crate::report::{format_location, format_location_detail, AnalysisReport};
use crate::Result;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn generate_markdown(
    report: &AnalysisReport,
    graph: Option<&DependencyGraph>,
    include_diagram: bool,
    severity_config: &SeverityConfig,
) -> String {
    let mut output = String::new();

    output.push_str("# Architecture Smell Report\n\n");

    output.push_str("## Summary\n\n");
    let grade = report.grade(severity_config);
    let total_score = report.total_score(severity_config);

    output.push_str("| Metric | Value |\n");
    output.push_str("| :--- | :--- |\n");
    output.push_str(&format!(
        "| **Architecture Quality** | **{:.1}/10 ({})** |\n",
        grade.score, grade.level
    ));
    output.push_str(&format!(
        "| **Total Score** | **{} points** (density: {:.2}) |\n",
        total_score, grade.density
    ));
    output.push_str(&format!("| Files analyzed | {} |\n", report.files_analyzed));
    output.push_str(&format!(
        "| Cyclic dependencies | {} |\n",
        report.cyclic_dependencies
    ));
    output.push_str(&format!("| God modules | {} |\n", report.god_modules));
    output.push_str(&format!("| Dead code files | {} |\n", report.dead_code));
    output.push_str(&format!("| Dead symbols | {} |\n", report.dead_symbols));
    output.push_str(&format!(
        "| High complexity functions | {} |\n",
        report.high_complexity_functions
    ));
    output.push_str(&format!("| Large files | {} |\n", report.large_files));
    output.push_str(&format!(
        "| Unstable interfaces | {} |\n",
        report.unstable_interfaces
    ));
    output.push_str(&format!("| Feature envy | {} |\n", report.feature_envy));
    output.push_str(&format!(
        "| Shotgun surgery | {} |\n",
        report.shotgun_surgery
    ));
    output.push_str(&format!(
        "| Hub dependencies | {} |\n",
        report.hub_dependencies
    ));
    output.push('\n');

    if report.smells.is_empty() {
        output.push_str("✅ No architectural smells detected!\n");
    } else {
        // Add Mermaid diagram if requested
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

        let cycles: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| {
                matches!(
                    s.smell_type,
                    SmellType::CyclicDependency | SmellType::CyclicDependencyCluster
                )
            })
            .collect();

        let cycle_clusters: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::CyclicDependencyCluster))
            .collect();

        let gods: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::GodModule))
            .collect();

        let dead: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::DeadCode))
            .collect();

        let dead_symbols: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::DeadSymbol { .. }))
            .collect();

        let high_complexity: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::HighComplexity { .. }))
            .collect();

        let large_files: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::LargeFile))
            .collect();

        let unstable_interfaces: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::UnstableInterface))
            .collect();

        let feature_envy: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::FeatureEnvy { .. }))
            .collect();

        let shotgun_surgery: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::ShotgunSurgery))
            .collect();

        let hub_dependencies: Vec<_> = report
            .smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::HubDependency { .. }))
            .collect();

        if !cycles.is_empty() {
            // Count total files in cycles
            let total_files: usize = cycle_clusters.iter().map(|(s, _)| s.files.len()).sum();

            if !cycle_clusters.is_empty() {
                output.push_str(&format!(
                    "## Cyclic Dependencies ({} clusters, {} files)\n\n",
                    cycle_clusters.len(),
                    total_files
                ));

                // Show recommendations once at the top
                if let Some((_, explanation)) = cycle_clusters.first() {
                    output.push_str(&format!("**Problem:** {}\n\n", explanation.problem));
                    output.push_str(&format!("**Reason:** {}\n\n", explanation.reason));

                    output.push_str("**Risks:**\n");
                    for risk in &explanation.risks {
                        output.push_str(&format!("- {}\n", risk));
                    }
                    output.push('\n');

                    output.push_str("**Recommendations:**\n");
                    for rec in &explanation.recommendations {
                        output.push_str(&format!("- {}\n", rec));
                    }
                    output.push_str("\n---\n\n");
                }

                // Show each cluster
                for (i, (smell, _)) in cycle_clusters.iter().enumerate() {
                    if let Some(cluster) = &smell.cluster {
                        let score = smell.score(severity_config);
                        output.push_str(&format!(
                            "### Cluster {}: {} files ({} pts)\n\n",
                            i + 1,
                            cluster.files.len(),
                            score
                        ));

                        // Show hotspots (top 10)
                        if !cluster.hotspots.is_empty() {
                            output.push_str("**Hotspots (most connections):**\n\n");
                            output.push_str("| File | Imports | Imported by |\n");
                            output.push_str("|------|---------|-------------|\n");

                            for hotspot in cluster.hotspots.iter().take(10) {
                                let formatted_path = ExplainEngine::format_file_path(&hotspot.file);
                                output.push_str(&format!(
                                    "| `{}` | {} | {} |\n",
                                    formatted_path, hotspot.out_degree, hotspot.in_degree
                                ));
                            }
                            output.push('\n');
                        }

                        // Show critical edges
                        if !cluster.critical_edges.is_empty() {
                            output.push_str("**Critical edges to break:**\n\n");
                            output.push_str("| From (location) | To | Impact |\n");
                            output.push_str("|-----------------|-----|--------|\n");

                            for edge in &cluster.critical_edges {
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

                        // Collapsible section with all files
                        output.push_str("<details>\n");
                        output.push_str(&format!(
                            "<summary>All {} files in cluster</summary>\n\n",
                            cluster.files.len()
                        ));
                        for file in &cluster.files {
                            let formatted_path = ExplainEngine::format_file_path(file);
                            output.push_str(&format!("- `{}`\n", formatted_path));
                        }
                        output.push_str("\n</details>\n\n");

                        // Collapsible section with all edges
                        if !cluster.internal_edges.is_empty() {
                            output.push_str("<details>\n");
                            output.push_str(&format!(
                                "<summary>All edges ({} imports)</summary>\n\n",
                                cluster.internal_edges.len()
                            ));
                            output.push_str("| Location | Import |\n");
                            output.push_str("|----------|--------|\n");
                            for loc in &cluster.internal_edges {
                                let location = format_location_detail(loc);
                                output.push_str(&format!(
                                    "| `{}` | {} |\n",
                                    location, loc.description
                                ));
                            }
                            output.push_str("\n</details>\n\n");
                        }
                    }
                }
            } else {
                // Fallback to old format if no clusters (backward compatibility)
                output.push_str("## Cyclic Dependencies\n\n");
                for (i, (smell, explanation)) in cycles.iter().enumerate() {
                    output.push_str(&format!("### Cycle {}\n\n", i + 1));

                    // Show detailed locations table if available
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
                        // Fallback to old format if no locations
                        let cycle_path = smell
                            .files
                            .iter()
                            .map(|p| format!("`{}`", ExplainEngine::format_file_path(p)))
                            .collect::<Vec<_>>()
                            .join(" → ");
                        output.push_str(&format!("**Path:** {}\n\n", cycle_path));
                    }

                    output.push_str(&format!("**Problem:** {}\n\n", explanation.problem));
                    output.push_str(&format!("**Reason:** {}\n\n", explanation.reason));

                    output.push_str("**Risks:**\n");
                    for risk in &explanation.risks {
                        output.push_str(&format!("- {}\n", risk));
                    }
                    output.push('\n');

                    output.push_str("**Recommendations:**\n");
                    for rec in &explanation.recommendations {
                        output.push_str(&format!("- {}\n", rec));
                    }
                    output.push('\n');
                }
            }
        }

        if !gods.is_empty() {
            output.push_str("## God Modules\n\n");
            for (smell, explanation) in gods {
                let file_path = smell
                    .files
                    .first()
                    .cloned()
                    .unwrap_or_else(|| std::path::PathBuf::from("Unknown"));

                let formatted_path = ExplainEngine::format_file_path(&file_path);
                let score = smell.score(severity_config);

                output.push_str(&format!("### {} ({} pts)\n\n", formatted_path, score));

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

                // Add Dependency Details
                if let Some(graph) = graph {
                    if let Some(node_idx) = graph.get_node(&file_path) {
                        // Incoming dependencies (Used by)
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
                                output
                                    .push_str(&format!("- ... and {} more\n", incoming.len() - 15));
                            }
                            output.push('\n');
                        }

                        // Outgoing dependencies (Depends on)
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
                                output
                                    .push_str(&format!("- ... and {} more\n", outgoing.len() - 15));
                            }
                            output.push('\n');
                        }
                    }
                }

                output.push_str(&format!("**Problem:** {}\n\n", explanation.problem));
                output.push_str(&format!("**Reason:** {}\n\n", explanation.reason));

                output.push_str("**Risks:**\n");
                for risk in &explanation.risks {
                    output.push_str(&format!("- {}\n", risk));
                }
                output.push('\n');

                output.push_str("**Recommendations:**\n");
                for rec in &explanation.recommendations {
                    output.push_str(&format!("- {}\n", rec));
                }
                output.push('\n');
            }
        }

        if !dead.is_empty() {
            output.push_str(&format!("## Dead Code ({} files)\n\n", dead.len()));

            // Show explanation once
            if let Some((_, explanation)) = dead.first() {
                output.push_str(&format!("**Problem:** {}\n\n", explanation.problem));
                output.push_str(&format!("**Reason:** {}\n\n", explanation.reason));

                output.push_str("**Risks:**\n");
                for risk in &explanation.risks {
                    output.push_str(&format!("- {}\n", risk));
                }
                output.push('\n');

                output.push_str("**Recommendations:**\n");
                for rec in &explanation.recommendations {
                    output.push_str(&format!("- {}\n", rec));
                }
                output.push('\n');
            }

            // Group files by directory
            output.push_str("### Files by Directory\n\n");
            let grouped = group_files_by_directory(&dead);

            for (dir, files) in grouped {
                output.push_str(&format!("**{}** ({} files):\n", dir, files.len()));
                for file in files {
                    output.push_str(&format!("- `{}`\n", file));
                }
                output.push('\n');
            }
        }

        if !dead_symbols.is_empty() {
            output.push_str(&format!(
                "## Dead Symbols ({} symbols)\n\n",
                dead_symbols.len()
            ));

            // Flat table with clickable locations
            output.push_str("| Location | Symbol | Kind |\n");
            output.push_str("|----------|--------|------|\n");

            for (smell, _) in &dead_symbols {
                if let SmellType::DeadSymbol { name, kind } = &smell.smell_type {
                    let location = smell
                        .locations
                        .first()
                        .map(format_location_detail)
                        .unwrap_or_else(|| {
                            smell
                                .files
                                .first()
                                .map(|f| ExplainEngine::format_file_path(f))
                                .unwrap_or_else(|| "unknown".to_string())
                        });
                    output.push_str(&format!("| `{}` | `{}` | {} |\n", location, name, kind));
                }
            }
            output.push('\n');
        }

        if !high_complexity.is_empty() {
            output.push_str(&format!(
                "## High Complexity Functions ({} functions)\n\n",
                high_complexity.len()
            ));

            output.push_str("| Location | Function | Complexity | Score |\n");
            output.push_str("|----------|----------|------------|-------|\n");

            for (smell, _) in high_complexity {
                if let SmellType::HighComplexity { name, line } = &smell.smell_type {
                    if let Some(file_path) = smell.files.first() {
                        // Use location from locations if available (has column info)
                        let location = smell
                            .locations
                            .first()
                            .map(format_location_detail)
                            .unwrap_or_else(|| format_location(file_path, *line, None));
                        let complexity = smell.complexity().unwrap_or(0);
                        let score = smell.score(severity_config);
                        output.push_str(&format!(
                            "| `{}` | `{}` | {} | {} pts |\n",
                            location, name, complexity, score
                        ));
                    }
                }
            }
            output.push('\n');
        }

        // Large Files section
        if !large_files.is_empty() {
            output.push_str(&format!("## Large Files ({} files)\n\n", large_files.len()));

            if let Some((_, explanation)) = large_files.first() {
                output.push_str(&format!("**Problem:** {}\n\n", explanation.problem));
                output.push_str(&format!("**Reason:** {}\n\n", explanation.reason));

                output.push_str("**Risks:**\n");
                for risk in &explanation.risks {
                    output.push_str(&format!("- {}\n", risk));
                }
                output.push_str("\n**Recommendations:**\n");
                for rec in &explanation.recommendations {
                    output.push_str(&format!("- {}\n", rec));
                }
                output.push('\n');
            }

            output.push_str("| File | Lines | Severity |\n");
            output.push_str("|------|-------|----------|\n");

            for (smell, _) in &large_files {
                if let Some(file_path) = smell.files.first() {
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let lines = smell.lines().unwrap_or(0);
                    let effective_severity = smell.effective_severity(severity_config);
                    let score = smell.score(severity_config);

                    let severity_str = match effective_severity {
                        Severity::Low => "🔵 Low",
                        Severity::Medium => "🟡 Medium",
                        Severity::High => "🟠 High",
                        Severity::Critical => "🔴 Critical",
                    };
                    output.push_str(&format!(
                        "| `{}` | {} | {} ({} pts) |\n",
                        formatted_path, lines, severity_str, score
                    ));
                }
            }
            output.push('\n');
        }

        // Unstable Interfaces section
        if !unstable_interfaces.is_empty() {
            output.push_str(&format!(
                "## Unstable Interfaces ({} files)\n\n",
                unstable_interfaces.len()
            ));

            if let Some((_, explanation)) = unstable_interfaces.first() {
                output.push_str(&format!("**Problem:** {}\n\n", explanation.problem));
                output.push_str(&format!("**Reason:** {}\n\n", explanation.reason));

                output.push_str("**Risks:**\n");
                for risk in &explanation.risks {
                    output.push_str(&format!("- {}\n", risk));
                }
                output.push_str("\n**Recommendations:**\n");
                for rec in &explanation.recommendations {
                    output.push_str(&format!("- {}\n", rec));
                }
                output.push('\n');
            }

            output.push_str("| File | Churn | Dependants | Score | pts |\n");
            output.push_str("|------|-------|------------|-------|-----|\n");

            for (smell, _) in &unstable_interfaces {
                if let Some(file_path) = smell.files.first() {
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let churn = smell.churn().unwrap_or(0);
                    let dependants = smell.fan_in().unwrap_or(0);
                    let score = smell.instability_score().unwrap_or(0);
                    let pts = smell.score(severity_config);

                    output.push_str(&format!(
                        "| `{}` | {} | {} | {} | {} pts |\n",
                        formatted_path, churn, dependants, score, pts
                    ));
                }
            }
            output.push('\n');
        }

        // Feature Envy section
        if !feature_envy.is_empty() {
            output.push_str(&format!(
                "## Feature Envy ({} files)\n\n",
                feature_envy.len()
            ));

            if let Some((_, explanation)) = feature_envy.first() {
                output.push_str(&format!("**Problem:** {}\n\n", explanation.problem));
                output.push_str(&format!("**Reason:** {}\n\n", explanation.reason));

                output.push_str("**Risks:**\n");
                for risk in &explanation.risks {
                    output.push_str(&format!("- {}\n", risk));
                }
                output.push_str("\n**Recommendations:**\n");
                for rec in &explanation.recommendations {
                    output.push_str(&format!("- {}\n", rec));
                }
                output.push('\n');
            }

            output.push_str("| File | Envied Module | Ratio | pts |\n");
            output.push_str("|------|---------------|-------|-----|\n");

            for (smell, _) in &feature_envy {
                if let Some(file_path) = smell.files.first() {
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let ratio = smell.envy_ratio().unwrap_or(0.0);
                    let pts = smell.score(severity_config);

                    let envied_module = match &smell.smell_type {
                        SmellType::FeatureEnvy { most_envied_module } => {
                            ExplainEngine::format_file_path(most_envied_module)
                        }
                        _ => "unknown".to_string(),
                    };

                    output.push_str(&format!(
                        "| `{}` | `{}` | {:.1}x | {} pts |\n",
                        formatted_path, envied_module, ratio, pts
                    ));
                }
            }
            output.push('\n');
        }

        // Shotgun Surgery section
        if !shotgun_surgery.is_empty() {
            output.push_str(&format!(
                "## Shotgun Surgery ({} files)\n\n",
                shotgun_surgery.len()
            ));

            if let Some((_, explanation)) = shotgun_surgery.first() {
                output.push_str(&format!("**Problem:** {}\n\n", explanation.problem));
                output.push_str(&format!("**Reason:** {}\n\n", explanation.reason));

                output.push_str("**Risks:**\n");
                for risk in &explanation.risks {
                    output.push_str(&format!("- {}\n", risk));
                }
                output.push_str("\n**Recommendations:**\n");
                for rec in &explanation.recommendations {
                    output.push_str(&format!("- {}\n", rec));
                }
                output.push('\n');
            }

            output.push_str("| File | Avg Co-changes | Related Files (Top 5) | pts |\n");
            output.push_str("|------|----------------|-----------------------|-----|\n");

            for (smell, _) in &shotgun_surgery {
                if let Some(file_path) = smell.files.first() {
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let avg = smell.avg_co_changes().unwrap_or(0.0);
                    let pts = smell.score(severity_config);

                    let related = smell
                        .locations
                        .iter()
                        .filter(|l| l.file != *file_path) // Skip the primary file in the related list
                        .map(|l| {
                            let path = ExplainEngine::format_file_path(&l.file);
                            if l.description.is_empty() {
                                format!("`{}`", path)
                            } else {
                                format!("`{}` ({})", path, l.description)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    output.push_str(&format!(
                        "| `{}` | {:.1} | {} | {} pts |\n",
                        formatted_path, avg, related, pts
                    ));
                }
            }
            output.push('\n');
        }

        // Hub Dependencies section
        if !hub_dependencies.is_empty() {
            output.push_str(&format!(
                "## Hub Dependencies ({} packages)\n\n",
                hub_dependencies.len()
            ));

            if let Some((_, explanation)) = hub_dependencies.first() {
                output.push_str(&format!("**Problem:** {}\n\n", explanation.problem));
                output.push_str(&format!("**Reason:** {}\n\n", explanation.reason));

                output.push_str("**Risks:**\n");
                for risk in &explanation.risks {
                    output.push_str(&format!("- {}\n", risk));
                }
                output.push_str("\n**Recommendations:**\n");
                for rec in &explanation.recommendations {
                    output.push_str(&format!("- {}\n", rec));
                }
                output.push('\n');
            }

            output.push_str("| Package | Dependants | pts |\n");
            output.push_str("|---------|------------|-----|\n");

            for (smell, _) in &hub_dependencies {
                if let SmellType::HubDependency { package } = &smell.smell_type {
                    let count = smell.dependant_count().unwrap_or(0);
                    let pts = smell.score(severity_config);

                    output.push_str(&format!(
                        "| `{}` | {} files | {} pts |\n",
                        package, count, pts
                    ));
                }
            }
            output.push('\n');
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

fn group_files_by_directory(
    dead_smells: &[&(crate::detectors::ArchSmell, crate::explain::Explanation)],
) -> BTreeMap<String, Vec<String>> {
    let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (smell, _) in dead_smells {
        if let Some(file_path) = smell.files.first() {
            let path = std::path::Path::new(file_path);

            // Get parent directory
            let dir = if let Some(parent) = path.parent() {
                if let Some(parent_str) = parent.to_str() {
                    // Shorten path - show only last 2-3 components
                    let components: Vec<_> = parent_str.split('/').collect();
                    let start = if components.len() > 3 {
                        components.len() - 3
                    } else {
                        0
                    };
                    components[start..].join("/")
                } else {
                    "unknown".to_string()
                }
            } else {
                ".".to_string()
            };

            // Get filename
            let filename = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            grouped.entry(dir).or_default().push(filename);
        }
    }

    // Sort files within each directory
    for files in grouped.values_mut() {
        files.sort();
    }

    grouped
}
