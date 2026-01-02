use crate::config::Config;
use crate::detectors::Severity;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Options for scanning a project (library-friendly version of ScanArgs)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanOptions {
    /// Path to config file
    pub config_path: Option<PathBuf>,

    /// Inline config (takes precedence over config_path)
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
}

impl ScanOptions {
    pub fn new() -> Self {
        Self {
            enable_cache: true,
            enable_git: true,
            ..Default::default()
        }
    }
}
