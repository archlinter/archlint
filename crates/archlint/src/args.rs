use std::path::PathBuf;

#[cfg(feature = "cli")]
use clap::{Parser, ValueEnum};

#[cfg_attr(feature = "cli", derive(Parser))]
#[derive(Debug, Clone)]
pub struct ScanArgs {
    /// Path to the project directory
    #[cfg_attr(feature = "cli", arg(value_name = "PATH", default_value = "."))]
    pub path: PathBuf,

    /// Programming language
    #[cfg_attr(feature = "cli", arg(long, default_value = "ts"))]
    pub lang: Language,

    /// Config file path
    #[cfg_attr(feature = "cli", arg(long, value_name = "FILE"))]
    pub config: Option<PathBuf>,

    /// Output report file (defaults to stdout if not specified)
    #[cfg_attr(feature = "cli", arg(long, value_name = "FILE"))]
    pub report: Option<PathBuf>,

    /// Output format
    #[cfg_attr(feature = "cli", arg(long, default_value = "table"))]
    pub format: OutputFormat,

    /// Output in JSON format (shortcut for --format json)
    #[cfg_attr(feature = "cli", arg(long, action = clap::ArgAction::SetTrue))]
    pub json: bool,

    /// Disable dependency diagram in Markdown reports
    #[cfg_attr(feature = "cli", arg(long, default_value = "false"))]
    pub no_diagram: bool,

    /// Run all available detectors (including those disabled by default)
    #[cfg_attr(feature = "cli", arg(long = "all", default_value = "false"))]
    pub all_detectors: bool,

    /// Only run these detectors (comma-separated IDs)
    #[cfg_attr(feature = "cli", arg(long, value_name = "IDS"))]
    pub detectors: Option<String>,

    /// Exclude these detectors (comma-separated IDs)
    #[cfg_attr(feature = "cli", arg(long, value_name = "IDS"))]
    pub exclude_detectors: Option<String>,

    /// Quiet mode (CI-friendly, no progress bars)
    #[cfg_attr(feature = "cli", arg(short, long, default_value = "false"))]
    pub quiet: bool,

    /// Verbose output
    #[cfg_attr(feature = "cli", arg(short, long, default_value = "false"))]
    pub verbose: bool,

    /// Minimum severity to include in report (low, medium, high, critical)
    #[cfg_attr(feature = "cli", arg(long, value_name = "SEVERITY"))]
    pub min_severity: Option<String>,

    /// Minimum score to include in report
    #[cfg_attr(feature = "cli", arg(long, value_name = "SCORE"))]
    pub min_score: Option<u32>,

    /// Override severity for specific smell types (e.g. "DeadCode=low,GodModule=high")
    #[cfg_attr(feature = "cli", arg(long, value_name = "OVERRIDES"))]
    pub severity: Option<String>,

    /// Disable caching
    #[cfg_attr(feature = "cli", arg(long, default_value = "false"))]
    pub no_cache: bool,

    /// Disable git integration (skip churn analysis)
    #[cfg_attr(feature = "cli", arg(long, default_value = "false"))]
    pub no_git: bool,

    /// Git history analysis period (e.g. "90d", "1y", "all")
    #[cfg_attr(feature = "cli", arg(long, value_name = "PERIOD"))]
    pub git_history_period: Option<String>,

    /// Explicit list of files to scan (internal use for glob expansion)
    #[cfg_attr(feature = "cli", arg(skip))]
    pub files: Option<Vec<PathBuf>>,
}

impl ScanArgs {
    /// Get output format, taking into account the --json flag
    pub fn output_format(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else {
            self.format
        }
    }

    /// Get quiet flag, automatically enabled when JSON output is requested
    pub fn is_quiet(&self) -> bool {
        self.quiet || self.json
    }
}

#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    #[cfg_attr(feature = "cli", value(name = "ts"))]
    TypeScript,
    #[cfg_attr(feature = "cli", value(name = "js"))]
    JavaScript,
}

#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OutputFormat {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "table"))]
    Table,
    #[cfg_attr(feature = "cli", value(name = "markdown"))]
    Markdown,
    #[cfg_attr(feature = "cli", value(name = "json"))]
    Json,
}
