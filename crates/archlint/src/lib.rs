pub mod cache;
pub mod cli;
pub mod config;
pub mod detectors;
pub mod engine;
pub mod error;
pub mod explain;
pub mod framework;
pub mod glob_expand;
pub mod graph;
pub mod metrics;
pub mod package_json;
pub mod parser;
pub mod project_root;
pub mod report;
pub mod resolver;
pub mod scanner;
pub mod watch;

// Public modules
pub mod api;

// Convenient re-exports for common use
pub use api::{clear_cache, get_detectors, load_config, scan};
pub use api::{ExportInfo, ExportKind, FileInfo, FileMetrics, ImportInfo};
pub use api::{ScanOptions, ScanResult, SmellWithExplanation, Summary};

pub use config::Config;
pub use detectors::registry::DetectorInfo;
pub use detectors::{ArchSmell, CodeRange, CycleCluster, LocationDetail, Severity, SmellType};
pub use error::{AnalysisError, Result};
pub use explain::Explanation;
pub use framework::{FileType, Framework};
pub use report::{AnalysisReport, ArchitectureGrade, GradeLevel};
