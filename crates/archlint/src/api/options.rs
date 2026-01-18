use crate::args::{OutputFormat, ScanArgs};
use crate::config::Config;
use crate::detectors::Severity;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Options for scanning a project (library-friendly version of `ScanArgs`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanOptions {
    /// Path to config file
    pub config_path: Option<PathBuf>,

    /// Inline config (takes precedence over `config_path`)
    pub config: Option<Config>,

    /// Only run these detectors (by ID)
    pub detectors: Option<Vec<String>>,

    /// Exclude these detectors (by ID)
    pub exclude_detectors: Vec<String>,

    /// Minimum severity to report
    pub min_severity: Option<Severity>,

    /// Minimum score to report
    pub min_score: Option<u32>,

    /// Enable caching (default: true)
    pub enable_cache: bool,

    /// Enable git integration (default: true)
    pub enable_git: bool,

    /// Git history analysis period (e.g. "90d", "1y", "all")
    pub git_history_period: Option<String>,

    /// Maximum file size in bytes to analyze
    pub max_file_size: Option<u64>,
}

impl ScanOptions {
    #[must_use]
    pub fn new() -> Self {
        Self {
            enable_cache: true,
            enable_git: true,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn to_scan_args(&self, path: &Path) -> ScanArgs {
        ScanArgs {
            path: path.to_path_buf(),
            config: self.config_path.clone(),
            report: None,
            format: OutputFormat::Table,
            json: true, // For structured output
            no_diagram: true,
            all_detectors: false,
            detectors: self.detectors.as_ref().map(|d| d.join(",")),
            exclude_detectors: if self.exclude_detectors.is_empty() {
                None
            } else {
                Some(self.exclude_detectors.join(","))
            },
            quiet: true,
            verbose: false,
            min_severity: self.min_severity.map(|s| format!("{s:?}").to_lowercase()),
            min_score: self.min_score,
            severity: None,
            no_cache: !self.enable_cache,
            no_git: !self.enable_git,
            git_history_period: self.git_history_period.clone(),
            max_file_size: self.max_file_size,
            files: None,
        }
    }
}
