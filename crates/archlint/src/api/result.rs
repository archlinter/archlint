use super::file_info::FileInfo;
use crate::config::SeverityConfig;
use crate::detectors::{ArchSmell, SmellType};
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

/// Result of an incremental project scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncrementalResult {
    /// All detected smells with explanations (only for affected files).
    pub smells: Vec<SmellWithExplanation>,
    /// Files that were re-analyzed.
    pub affected_files: Vec<PathBuf>,
    /// Number of files that changed on disk.
    pub changed_count: usize,
    /// Total number of files affected (including transitively).
    pub affected_count: usize,
    /// Time taken for analysis in milliseconds.
    pub analysis_time_ms: u64,
}

/// A detected smell bundled with its human-readable explanation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmellWithExplanation {
    /// The detected architectural smell.
    pub smell: ArchSmell,
    /// Human-readable explanation and context for the smell.
    pub explanation: Explanation,
}

/// Summary statistics for a project scan.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    /// Total number of files analyzed.
    pub files_analyzed: usize,
    /// Total number of smells found.
    pub total_smells: usize,
    /// Number of cyclic dependencies found.
    pub cyclic_dependencies: usize,
    /// Number of cycle clusters (groups of interconnected cycles).
    pub cycle_clusters: usize,
    /// Total number of files involved in cycles.
    pub files_in_cycles: usize,
    /// Number of god modules found.
    pub god_modules: usize,
    /// Number of dead code files.
    pub dead_code: usize,
    /// Number of dead symbols (unused exports).
    pub dead_symbols: usize,
    /// Number of functions with high cyclomatic complexity.
    pub high_cyclomatic_complexity_functions: usize,
    /// Number of functions with high cognitive complexity.
    pub high_cognitive_complexity_functions: usize,
    /// @deprecated use `high_cyclomatic_complexity_functions` or `high_cognitive_complexity_functions`
    #[deprecated(
        since = "0.16.0",
        note = "Use high_cyclomatic_complexity_functions or high_cognitive_complexity_functions"
    )]
    pub high_complexity_functions: usize,
    /// Number of unstable interfaces.
    pub unstable_interfaces: usize,
    /// Number of files with feature envy.
    pub feature_envy: usize,
    /// Number of files requiring shotgun surgery.
    pub shotgun_surgery: usize,
    /// Number of hub dependencies.
    pub hub_dependencies: usize,
    /// Number of large files.
    pub large_files: usize,
}

impl ScanResult {
    /// Create a `ScanResult` from an `AnalysisReport` and additional metadata.
    #[must_use]
    pub fn from_report(
        report: AnalysisReport,
        files: Vec<FileInfo>,
        project_path: &Path,
        severity_config: &SeverityConfig,
    ) -> Self {
        let grade = report.grade(severity_config);

        let summary = Summary {
            files_analyzed: report.files_analyzed(),
            total_smells: report.smells.len(),
            cyclic_dependencies: report.cyclic_dependencies(),
            cycle_clusters: report
                .smells
                .iter()
                .filter(|(s, _)| matches!(s.smell_type, SmellType::CyclicDependencyCluster))
                .count(),
            files_in_cycles: report
                .smells
                .iter()
                .filter(|(s, _)| matches!(s.smell_type, SmellType::CyclicDependencyCluster))
                .map(|(s, _)| s.files.len())
                .sum(),
            god_modules: report.god_modules(),
            dead_code: report.dead_code(),
            dead_symbols: report.dead_symbols(),
            high_cyclomatic_complexity_functions: report.high_cyclomatic_complexity_functions(),
            high_cognitive_complexity_functions: report.high_cognitive_complexity_functions(),
            #[allow(deprecated)]
            high_complexity_functions: report
                .high_cyclomatic_complexity_functions()
                .max(report.high_cognitive_complexity_functions()),
            unstable_interfaces: report.unstable_interfaces(),
            feature_envy: report.feature_envy(),
            shotgun_surgery: report.shotgun_surgery(),
            hub_dependencies: report.hub_dependencies(),
            large_files: report.large_files(),
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
