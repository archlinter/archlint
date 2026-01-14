use crate::detectors::{ArchSmell, Severity, SmellKind};
use crate::explain::Explanation;
use crate::report::AnalysisReport;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Root structure for SARIF log format v2.1.0
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifLog {
    version: String,
    #[serde(rename = "$schema")]
    schema: String,
    runs: Vec<SarifRun>,
}

/// Represents a single analysis run in SARIF
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifRun {
    tool: SarifTool,
    results: Vec<SarifResult>,
}

/// Information about the tool that produced the SARIF log
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifTool {
    driver: SarifDriver,
}

/// The main tool driver information, including rules and tool version
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifDriver {
    name: String,
    version: String,
    rules: Vec<SarifRule>,
}

/// Definition of a static analysis rule
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifRule {
    id: String,
    short_description: SarifMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    help_uri: Option<String>,
}

/// A single finding/result in the SARIF log
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifResult {
    rule_id: String,
    level: String,
    message: SarifMessage,
    locations: Vec<SarifLocation>,
}

/// A message in SARIF, usually containing plain text
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifMessage {
    text: String,
}

/// Location information for a result
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifLocation {
    physical_location: SarifPhysicalLocation,
}

/// Physical location in a file (artifact)
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifPhysicalLocation {
    artifact_location: SarifArtifactLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<SarifRegion>,
}

/// Location of an artifact (usually a file URI)
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifArtifactLocation {
    uri: String,
}

/// A region within a file (line, column, etc.)
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SarifRegion {
    start_line: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_column: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_column: Option<usize>,
}

/// Generates a SARIF JSON value from an AnalysisReport.
///
/// # Arguments
/// * `report` - The analysis report containing detected smells and explanations.
/// * `_config` - Severity configuration (currently unused, but reserved for future customization).
/// * `scan_root` - Optional root path to make file URIs relative.
pub fn generate_sarif(
    report: &AnalysisReport,
    _config: &crate::config::SeverityConfig,
    scan_root: Option<&Path>,
) -> Result<serde_json::Value> {
    let mut rules = Vec::new();
    let mut results = Vec::new();
    let mut seen_rules = HashSet::new();

    for (smell, explanation) in &report.smells {
        let category = smell.smell_type.category();
        let rule_id = category.to_id();

        if !seen_rules.contains(rule_id) {
            rules.push(create_rule(category));
            seen_rules.insert(rule_id.to_string());
        }

        results.push(SarifResult {
            rule_id: rule_id.to_string(),
            level: map_severity(&smell.severity).to_string(),
            message: SarifMessage {
                text: format_sarif_message(explanation),
            },
            locations: map_locations(smell, scan_root),
        });
    }

    Ok(serde_json::to_value(SarifLog {
        version: "2.1.0".to_string(),
        schema: "https://schemastore.azurewebsites.net/schemas/json/sarif-2.1.0-rtm.5.json"
            .to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "archlint".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    rules,
                },
            },
            results,
        }],
    })?)
}

fn map_severity(severity: &Severity) -> &'static str {
    match severity {
        Severity::Low => "note",
        Severity::Medium => "warning",
        Severity::High | Severity::Critical => "error",
    }
}

fn format_sarif_message(explanation: &Explanation) -> String {
    format!(
        "{}\n\nReason: {}\n\nRecommendations:\n{}",
        explanation.problem,
        explanation.reason,
        explanation.recommendations.join("\n")
    )
}

fn normalize_path(path: &Path, scan_root: Option<&Path>) -> String {
    let path_str = if let Some(root) = scan_root {
        path.strip_prefix(root).unwrap_or(path).to_string_lossy()
    } else {
        path.to_string_lossy()
    };
    path_str.replace('\\', "/")
}

fn create_sarif_location(
    file: &Path,
    start_line: usize,
    start_column: Option<usize>,
    range: Option<&crate::detectors::CodeRange>,
    scan_root: Option<&Path>,
) -> SarifLocation {
    SarifLocation {
        physical_location: SarifPhysicalLocation {
            artifact_location: SarifArtifactLocation {
                uri: normalize_path(file, scan_root),
            },
            region: Some(SarifRegion {
                start_line,
                start_column,
                end_line: range.map(|r| r.end_line),
                end_column: range.map(|r| r.end_column),
            }),
        },
    }
}

fn map_locations(smell: &ArchSmell, scan_root: Option<&Path>) -> Vec<SarifLocation> {
    if !smell.locations.is_empty() {
        smell
            .locations
            .iter()
            .map(|loc| {
                create_sarif_location(
                    &loc.file,
                    if loc.line == 0 { 1 } else { loc.line },
                    loc.column,
                    loc.range.as_ref(),
                    scan_root,
                )
            })
            .collect()
    } else {
        smell
            .files
            .iter()
            .map(|file| create_sarif_location(file, 1, None, None, scan_root))
            .collect()
    }
}

fn create_rule(category: SmellKind) -> SarifRule {
    let rule_id = category.to_id();
    SarifRule {
        id: rule_id.to_string(),
        short_description: SarifMessage {
            text: category.display_name().to_string(),
        },
        help_uri: Some(format!(
            "https://archlinter.github.io/archlint/detectors/{}.html",
            rule_id
        )),
    }
}

/// Writes a SARIF report to the specified file path.
///
/// # Arguments
/// * `report` - The analysis report.
/// * `path` - Destination file path.
/// * `config` - Severity configuration.
/// * `scan_root` - Optional root path for relative file URIs.
pub fn write_report<P: AsRef<Path>>(
    report: &AnalysisReport,
    path: P,
    config: &crate::config::SeverityConfig,
    scan_root: Option<&Path>,
) -> Result<()> {
    let output = generate_sarif(report, config, scan_root)?;
    let content = serde_json::to_string_pretty(&output)?;
    fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detectors::smell::{ArchSmell, LocationDetail};
    use crate::detectors::types::{Severity, SmellType};
    use crate::report::AnalysisReportBuilder;
    use std::path::PathBuf;

    #[test]
    fn test_generate_sarif() {
        let smell = ArchSmell {
            smell_type: SmellType::CyclicDependency,
            severity: Severity::High,
            files: vec![PathBuf::from("a.ts"), PathBuf::from("b.ts")],
            metrics: vec![],
            locations: vec![LocationDetail::new(
                PathBuf::from("a.ts"),
                1,
                "import b".to_string(),
            )],
            cluster: None,
        };

        let report = AnalysisReportBuilder::new()
            .with_smells(vec![smell])
            .build();

        let config = crate::config::SeverityConfig::default();
        let sarif_val = generate_sarif(&report, &config, None).unwrap();
        let sarif: SarifLog = serde_json::from_value(sarif_val).unwrap();
        assert_eq!(sarif.version, "2.1.0");
        assert_eq!(sarif.runs.len(), 1);
        assert_eq!(sarif.runs[0].results.len(), 1);
        assert_eq!(sarif.runs[0].results[0].rule_id, "cyclic_dependency");
        assert_eq!(sarif.runs[0].results[0].level, "error");
        assert!(sarif.runs[0].results[0].message.text.contains("Reason:"));
    }

    #[test]
    fn test_sarif_relative_paths_and_windows_separators() {
        let file_path = if cfg!(windows) {
            PathBuf::from("C:\\project\\src\\main.ts")
        } else {
            PathBuf::from("/project/src/main.ts")
        };

        let smell = ArchSmell {
            smell_type: SmellType::GodModule,
            severity: Severity::Medium,
            files: vec![file_path.clone()],
            metrics: vec![],
            locations: vec![],
            cluster: None,
        };

        let report = AnalysisReportBuilder::new()
            .with_smells(vec![smell])
            .build();

        let config = crate::config::SeverityConfig::default();
        // Mock scan_root as /project (or C:\project)
        let scan_root = if cfg!(windows) {
            PathBuf::from("C:\\project")
        } else {
            PathBuf::from("/project")
        };

        let sarif_val = generate_sarif(&report, &config, Some(&scan_root)).unwrap();
        let sarif: SarifLog = serde_json::from_value(sarif_val).unwrap();

        assert_eq!(
            sarif.runs[0].results[0].locations[0]
                .physical_location
                .artifact_location
                .uri,
            "src/main.ts"
        );
    }

    #[test]
    fn test_sarif_severity_mapping() {
        let smells = vec![
            ArchSmell {
                smell_type: SmellType::DeadCode,
                severity: Severity::Low,
                files: vec![PathBuf::from("low.ts")],
                metrics: vec![],
                locations: vec![],
                cluster: None,
            },
            ArchSmell {
                smell_type: SmellType::GodModule,
                severity: Severity::Medium,
                files: vec![PathBuf::from("med.ts")],
                metrics: vec![],
                locations: vec![],
                cluster: None,
            },
            ArchSmell {
                smell_type: SmellType::CyclicDependency,
                severity: Severity::Critical,
                files: vec![PathBuf::from("crit.ts")],
                metrics: vec![],
                locations: vec![],
                cluster: None,
            },
        ];

        let report = AnalysisReportBuilder::new().with_smells(smells).build();
        let config = crate::config::SeverityConfig::default();
        let sarif_val = generate_sarif(&report, &config, None).unwrap();
        let sarif: SarifLog = serde_json::from_value(sarif_val).unwrap();

        let results = &sarif.runs[0].results;
        assert_eq!(results[0].level, "note");
        assert_eq!(results[1].level, "warning");
        assert_eq!(results[2].level, "error");
    }

    #[test]
    fn test_sarif_empty_report() {
        let report = AnalysisReportBuilder::new().with_smells(vec![]).build();
        let config = crate::config::SeverityConfig::default();
        let sarif_val = generate_sarif(&report, &config, None).unwrap();
        let sarif: SarifLog = serde_json::from_value(sarif_val).unwrap();

        assert_eq!(sarif.runs[0].results.len(), 0);
    }
}
