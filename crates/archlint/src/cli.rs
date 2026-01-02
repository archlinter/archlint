use clap::{Parser, ValueEnum};
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

    /// Programming language
    #[arg(long, default_value = "ts")]
    pub lang: Option<Language>,

    /// Config file path
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Output report file (defaults to stdout if not specified)
    #[arg(long, value_name = "FILE")]
    pub report: Option<PathBuf>,

    /// Output format
    #[arg(long, default_value = "table")]
    pub format: Option<OutputFormat>,

    /// Output in JSON format (shortcut for --format json)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub json: bool,

    /// Disable dependency diagram in Markdown reports
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_diagram: bool,

    /// Run all available detectors (including those disabled by default)
    #[arg(long = "all", action = clap::ArgAction::SetTrue)]
    pub all_detectors: bool,

    /// Only run these detectors (comma-separated IDs)
    #[arg(long, value_name = "IDS")]
    pub detectors: Option<String>,

    /// Exclude these detectors (comma-separated IDs)
    #[arg(long, value_name = "IDS")]
    pub exclude_detectors: Option<String>,

    /// Quiet mode (CI-friendly, no progress bars)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub quiet: bool,

    /// Verbose output
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    /// Minimum severity to include in report (low, medium, high, critical)
    #[arg(long, value_name = "SEVERITY")]
    pub min_severity: Option<String>,

    /// Minimum score to include in report
    #[arg(long, value_name = "SCORE")]
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
}

impl Cli {
    /// Convert Cli to ScanArgs when no command is specified (default scan)
    pub fn to_scan_args(&self) -> ScanArgs {
        let format = if self.json {
            OutputFormat::Json
        } else {
            self.format.unwrap_or(OutputFormat::Table)
        };
        // Automatically enable quiet mode when JSON output is requested
        let quiet = self.quiet || self.json;
        ScanArgs {
            path: self.path.clone().unwrap_or_else(|| PathBuf::from(".")),
            lang: self.lang.unwrap_or(Language::TypeScript),
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

#[derive(Parser, Debug, Clone)]
pub struct ScanArgs {
    /// Path to the project directory
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    /// Programming language
    #[arg(long, default_value = "ts")]
    pub lang: Language,

    /// Config file path
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Output report file (defaults to stdout if not specified)
    #[arg(long, value_name = "FILE")]
    pub report: Option<PathBuf>,

    /// Output format
    #[arg(long, default_value = "table")]
    pub format: OutputFormat,

    /// Output in JSON format (shortcut for --format json)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub json: bool,

    /// Disable dependency diagram in Markdown reports
    #[arg(long, default_value = "false")]
    pub no_diagram: bool,

    /// Run all available detectors (including those disabled by default)
    #[arg(long = "all", default_value = "false")]
    pub all_detectors: bool,

    /// Only run these detectors (comma-separated IDs)
    #[arg(long, value_name = "IDS")]
    pub detectors: Option<String>,

    /// Exclude these detectors (comma-separated IDs)
    #[arg(long, value_name = "IDS")]
    pub exclude_detectors: Option<String>,

    /// Quiet mode (CI-friendly, no progress bars)
    #[arg(short, long, default_value = "false")]
    pub quiet: bool,

    /// Verbose output
    #[arg(short, long, default_value = "false")]
    pub verbose: bool,

    /// Minimum severity to include in report (low, medium, high, critical)
    #[arg(long, value_name = "SEVERITY")]
    pub min_severity: Option<String>,

    /// Minimum score to include in report
    #[arg(long, value_name = "SCORE")]
    pub min_score: Option<u32>,

    /// Override severity for specific smell types (e.g. "DeadCode=low,GodModule=high")
    #[arg(long, value_name = "OVERRIDES")]
    pub severity: Option<String>,

    /// Disable caching
    #[arg(long, default_value = "false")]
    pub no_cache: bool,

    /// Disable git integration (skip churn analysis)
    #[arg(long, default_value = "false")]
    pub no_git: bool,

    /// Explicit list of files to scan (internal use for glob expansion)
    #[arg(skip)]
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

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Language {
    #[value(name = "ts")]
    TypeScript,
    #[value(name = "js")]
    JavaScript,
}

#[derive(Debug, Clone, Copy, ValueEnum, Default, PartialEq, Eq)]
pub enum OutputFormat {
    #[default]
    Table,
    Markdown,
    Json,
}
