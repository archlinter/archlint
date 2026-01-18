use crate::args::{validate_detector_ids, OutputFormat, ScanArgs};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "archlint")]
#[command(version, about = "Detect architectural smells in codebases", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Path to the project directory
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: Option<PathBuf>,

    /// Config file path
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Output report file (defaults to stdout if not specified)
    #[arg(short, long, value_name = "FILE")]
    pub report: Option<PathBuf>,

    /// Output format
    #[arg(short, long, default_value = "table")]
    pub format: Option<OutputFormat>,

    /// Output in JSON format (shortcut for --format json)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub json: bool,

    /// Disable dependency diagram in Markdown reports
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_diagram: bool,

    /// Run all available detectors (including those disabled by default)
    #[arg(short = 'A', long = "all", action = clap::ArgAction::SetTrue)]
    pub all_detectors: bool,

    /// Only run these detectors (comma-separated IDs)
    #[arg(short, long, value_name = "IDS", value_parser = validate_detector_ids)]
    pub detectors: Option<String>,

    /// Exclude these detectors (comma-separated IDs)
    #[arg(short, long, value_name = "IDS", value_parser = validate_detector_ids)]
    pub exclude_detectors: Option<String>,

    /// Quiet mode (CI-friendly, no progress bars)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub quiet: bool,

    /// Verbose output
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    /// Minimum severity to include in report (low, medium, high, critical)
    #[arg(short = 's', long, value_name = "SEVERITY")]
    pub min_severity: Option<String>,

    /// Minimum score to include in report
    #[arg(short = 'S', long, value_name = "SCORE")]
    pub min_score: Option<u32>,

    /// Override severity for specific smell types (e.g. "DeadCode=low,GodModule=high")
    #[arg(long, value_name = "OVERRIDES")]
    pub severity: Option<String>,

    /// Disable caching
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_cache: bool,

    /// Disable git integration (skip churn analysis)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_git: bool,

    /// Git history analysis period (e.g. "90d", "1y", "all")
    #[arg(long, value_name = "PERIOD")]
    pub git_history_period: Option<String>,

    /// Maximum file size in bytes to analyze
    #[arg(long, value_name = "BYTES")]
    pub max_file_size: Option<u64>,
}

impl Cli {
    /// Convert Cli to `ScanArgs` when no command is specified (default scan)
    #[must_use]
    pub fn to_scan_args(&self) -> ScanArgs {
        let format = if self.json {
            OutputFormat::Json
        } else {
            self.format.unwrap_or(OutputFormat::Table)
        };
        // Automatically enable quiet mode when JSON output is requested
        let quiet = self.quiet || self.json || self.format == Some(OutputFormat::Sarif);
        ScanArgs {
            path: self.path.clone().unwrap_or_else(|| PathBuf::from(".")),
            config: self.config.clone(),
            report: self.report.clone(),
            format,
            json: self.json,
            no_diagram: self.no_diagram,
            all_detectors: self.all_detectors,
            detectors: self.detectors.clone(),
            exclude_detectors: self.exclude_detectors.clone(),
            quiet,
            verbose: self.verbose,
            min_severity: self.min_severity.clone(),
            min_score: self.min_score,
            severity: self.severity.clone(),
            no_cache: self.no_cache,
            no_git: self.no_git,
            git_history_period: self.git_history_period.clone(),
            max_file_size: self.max_file_size,
            files: None,
        }
    }
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Scan a project for architectural smells
    Scan(ScanArgs),

    /// Manage detectors
    Detectors(DetectorArgs),

    /// Cache management
    Cache(CacheArgs),

    /// Watch for changes and re-run analysis
    Watch(WatchArgs),

    /// Generate shell completions
    Completions(CompletionsArgs),

    /// Generate architectural snapshot
    Snapshot(SnapshotArgs),

    /// Compare two snapshots for regressions
    Diff(DiffArgs),

    /// Initialize a new configuration file
    Init(InitArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct InitArgs {
    /// Overwrite existing config file
    #[arg(short, long)]
    pub force: bool,

    /// Skip interactive framework selection
    #[arg(long)]
    pub no_interactive: bool,

    /// Explicitly specify framework presets (comma-separated or multiple flags)
    #[arg(long, value_delimiter = ',')]
    pub presets: Vec<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct SnapshotArgs {
    /// Output file path
    #[arg(short, long, default_value = ".archlint-snapshot.json")]
    pub output: PathBuf,

    /// Include git commit in snapshot
    #[arg(long, default_value = "true")]
    pub include_commit: bool,

    /// Project path to analyze
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

#[derive(Parser, Debug, Clone)]
pub struct DiffArgs {
    /// Baseline snapshot (file path or git ref)
    pub baseline: String,

    /// Current snapshot (file path or git ref, default: analyze current)
    #[arg(default_value = "")]
    pub current: String,

    /// Show detailed explanations
    #[arg(long)]
    pub explain: bool,

    /// Output as JSON
    #[arg(short, long)]
    pub json: bool,

    /// Minimum severity to fail on (low, medium, high, critical)
    #[arg(long, default_value = "low")]
    pub fail_on: String,

    /// Project path
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,
}

#[derive(Parser, Debug)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: clap_complete::Shell,
}

#[derive(Parser, Debug, Clone)]
pub struct WatchArgs {
    /// Debounce time in milliseconds
    #[arg(long, default_value = "300")]
    pub debounce: u64,

    /// Clear screen before re-running analysis
    #[arg(long, default_value = "false")]
    pub clear: bool,

    /// Ignore patterns (e.g. "*.test.ts")
    #[arg(long, value_name = "PATTERN")]
    pub ignore: Vec<String>,

    /// Analysis options
    #[command(flatten)]
    pub scan: ScanArgs,
}

#[derive(Parser, Debug)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub command: CacheCommand,
}

#[derive(Parser, Debug)]
pub enum CacheCommand {
    /// Clear analysis cache
    Clear,
}

#[derive(Parser, Debug)]
pub struct DetectorArgs {
    #[command(subcommand)]
    pub command: DetectorCommand,
}

#[derive(Parser, Debug)]
pub enum DetectorCommand {
    /// List all available detectors
    List,
}
