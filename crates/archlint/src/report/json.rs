use crate::detectors::SmellType;
use crate::report::AnalysisReport;
use crate::Result;
use serde_json::json;
use std::fs;
use std::path::Path;

pub fn generate_json(
    report: &AnalysisReport,
    config: &crate::config::SeverityConfig,
) -> serde_json::Value {
    let smells_json: Vec<_> = report.smells.iter()
        .map(|(smell, explanation)| {
            let mut smell_json = json!({
                "type": match &smell.smell_type {
                    SmellType::CyclicDependency => "cyclic_dependency".to_string(),
                    SmellType::CyclicDependencyCluster => "cyclic_dependency_cluster".to_string(),
                    SmellType::GodModule => "god_module".to_string(),
                    SmellType::DeadCode => "dead_code".to_string(),
                    SmellType::DeadSymbol { name, kind } => format!("dead_symbol: {} ({})", name, kind),
                    SmellType::HighComplexity { name, line, complexity } => format!("high_complexity: {} at line {} (complexity: {})", name, line, complexity),
                    SmellType::LargeFile => "large_file".to_string(),
                    SmellType::UnstableInterface => "unstable_interface".to_string(),
                    SmellType::FeatureEnvy { most_envied_module } => format!("feature_envy: envies {}", most_envied_module.display()),
                    SmellType::ShotgunSurgery => "shotgun_surgery".to_string(),
                    SmellType::HubDependency { package } => format!("hub_dependency: {}", package),
                    SmellType::TestLeakage { test_file } => format!("test_leakage: imports {}", test_file.display()),
                    SmellType::LayerViolation { from_layer, to_layer } => format!("layer_violation: {} -> {}", from_layer, to_layer),
                    SmellType::SdpViolation => "sdp_violation".to_string(),
                    _ => format!("{:?}", smell.smell_type),
                },
                "severity": format!("{:?}", smell.severity),
                "files": smell.files.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>(),
                "locations": smell.locations.iter().map(|loc| {
                    let mut loc_json = json!({
                        "file": loc.file.to_string_lossy(),
                        "line": loc.line,
                        "description": loc.description,
                    });
                    if let Some(col) = loc.column {
                        loc_json["column"] = json!(col);
                    }
                    if let Some(range) = &loc.range {
                        loc_json["range"] = json!({
                            "start_line": range.start_line,
                            "start_column": range.start_column,
                            "end_line": range.end_line,
                            "end_column": range.end_column,
                        });
                    }
                    loc_json
                }).collect::<Vec<_>>(),
                "explanation": {
                    "problem": explanation.problem,
                    "reason": explanation.reason,
                    "risks": explanation.risks,
                    "recommendations": explanation.recommendations,
                },
                "metrics": smell.metrics,
            });

            // Add cluster information if present
            if let Some(cluster) = &smell.cluster {
                smell_json["cluster"] = json!({
                    "files": cluster.files.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>(),
                    "hotspots": cluster.hotspots.iter().map(|h| {
                        json!({
                            "file": h.file.to_string_lossy(),
                            "in_degree": h.in_degree,
                            "out_degree": h.out_degree,
                        })
                    }).collect::<Vec<_>>(),
                    "critical_edges": cluster.critical_edges.iter().map(|e| {
                        let mut edge_json = json!({
                            "from": e.from.to_string_lossy(),
                            "to": e.to.to_string_lossy(),
                            "line": e.line,
                            "impact": e.impact,
                        });
                        if let Some(range) = &e.range {
                            edge_json["column"] = json!(range.start_column);
                            edge_json["range"] = json!({
                                "start_line": range.start_line,
                                "start_column": range.start_column,
                                "end_line": range.end_line,
                                "end_column": range.end_column,
                            });
                        }
                        edge_json
                    }).collect::<Vec<_>>(),
                    "internal_edges_count": cluster.internal_edges.len(),
                });
            }

            smell_json
        })
        .collect();

    // Count cycle clusters and files in cycles
    let cycle_clusters_count = report
        .smells
        .iter()
        .filter(|(s, _)| matches!(s.smell_type, SmellType::CyclicDependencyCluster))
        .count();

    let files_in_cycles: usize = report
        .smells
        .iter()
        .filter(|(s, _)| matches!(s.smell_type, SmellType::CyclicDependencyCluster))
        .map(|(s, _)| s.files.len())
        .sum();

    let grade = report.grade(config);

    json!({
        "summary": {
            "files_analyzed": report.files_analyzed(),
            "cyclic_dependencies": report.cyclic_dependencies(),
            "cycle_clusters": cycle_clusters_count,
            "files_in_cycles": files_in_cycles,
            "god_modules": report.god_modules(),
            "dead_code": report.dead_code(),
            "dead_symbols": report.dead_symbols(),
            "high_complexity_functions": report.high_complexity_functions(),
            "large_files": report.large_files(),
            "unstable_interfaces": report.unstable_interfaces(),
            "feature_envy": report.feature_envy(),
            "shotgun_surgery": report.shotgun_surgery(),
            "hub_dependencies": report.hub_dependencies(),
            "total_smells": report.smells.len(),
            "architecture_grade": {
                "score": format!("{:.1}", grade.score),
                "level": grade.level.to_string(),
                "density": format!("{:.2}", grade.density),
            }
        },
        "smells": smells_json,
    })
}

pub fn write_report<P: AsRef<Path>>(
    report: &AnalysisReport,
    path: P,
    config: &crate::config::SeverityConfig,
) -> Result<()> {
    let output = generate_json(report, config);
    fs::write(path, serde_json::to_string_pretty(&output)?)?;
    Ok(())
}
