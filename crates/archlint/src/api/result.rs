use super::file_info::FileInfo;
use crate::config::SeverityConfig;
use crate::detectors::ArchSmell;
use crate::explain::Explanation;
use crate::report::AnalysisReport;
use crate::report::ArchitectureGrade;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Complete scan result with smells and file information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    /// All detected smells with explanations
    pub smells: Vec<SmellWithExplanation>,

    /// Summary statistics
    pub summary: Summary,

    /// Information about analyzed files (for Plugin API)
    pub files: Vec<FileInfo>,

    /// Architecture grade
    pub grade: ArchitectureGrade,

    /// Project path that was scanned
    pub project_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmellWithExplanation {
    pub smell: ArchSmell,
    pub explanation: Explanation,
}

/// Summary statistics matching current JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub files_analyzed: usize,
    pub total_smells: usize,
    pub cyclic_dependencies: usize,
    pub cycle_clusters: usize,
    pub files_in_cycles: usize,
    pub god_modules: usize,
    pub dead_code: usize,
    pub dead_symbols: usize,
    pub high_complexity_functions: usize,
    pub unstable_interfaces: usize,
    pub feature_envy: usize,
    pub shotgun_surgery: usize,
    pub hub_dependencies: usize,
}

impl ScanResult {
    pub fn from_report(report: AnalysisReport, files: Vec<FileInfo>, project_path: &Path) -> Self {
        // Use default severity config for grade calculation if not provided
        let grade = report.grade(&SeverityConfig::default());

        let summary = Summary {
            files_analyzed: report.files_analyzed,
            total_smells: report.smells.len(),
            cyclic_dependencies: report.cyclic_dependencies,
            cycle_clusters: report
                .smells
                .iter()
                .filter(|(s, _)| {
                    matches!(
                        s.smell_type,
                        crate::detectors::SmellType::CyclicDependencyCluster
                    )
                })
                .count(),
            files_in_cycles: report
                .smells
                .iter()
                .filter(|(s, _)| {
                    matches!(
                        s.smell_type,
                        crate::detectors::SmellType::CyclicDependencyCluster
                    )
                })
                .map(|(s, _)| s.files.len())
                .sum(),
            god_modules: report.god_modules,
            dead_code: report.dead_code,
            dead_symbols: report.dead_symbols,
            high_complexity_functions: report.high_complexity_functions,
            unstable_interfaces: report.unstable_interfaces,
            feature_envy: report.feature_envy,
            shotgun_surgery: report.shotgun_surgery,
            hub_dependencies: report.hub_dependencies,
        };

        Self {
            smells: report
                .smells
                .into_iter()
                .map(|(smell, explanation)| SmellWithExplanation { smell, explanation })
                .collect(),
            summary,
            files,
            grade,
            project_path: project_path.to_path_buf(),
        }
    }
}
