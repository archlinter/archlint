//! Public API for archlint library
//!
//! This module provides the stable public interface for programmatic use.

use crate::args::{Language, OutputFormat, ScanArgs};
use crate::config::Config;
use crate::detectors::registry::{DetectorInfo, DetectorRegistry};
use crate::engine::AnalysisEngine;
use crate::error::Result;
use std::path::Path;

pub mod analyzer;
pub mod file_info;
pub mod options;
pub mod result;

pub use analyzer::{Analyzer, StateStats};
pub use file_info::{ExportInfo, ExportKind, FileInfo, FileMetrics, ImportInfo};
pub use options::ScanOptions;
pub use result::{IncrementalResult, ScanResult, SmellWithExplanation, Summary};

/// Scan a project for architectural smells
///
/// # Example
/// ```no_run
/// use archlint::{scan, ScanOptions};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let result = scan("./my-project", ScanOptions::new())?;
/// println!("Found {} smells", result.smells.len());
/// # Ok(())
/// # }
/// ```
pub fn scan<P: AsRef<Path>>(path: P, options: ScanOptions) -> Result<ScanResult> {
    let path_ref = path.as_ref();

    // 1. Load config if needed
    let config = match (options.config.clone(), options.config_path.as_ref()) {
        (Some(cfg), _) => cfg,
        (None, Some(p)) => Config::load(p)?,
        (None, None) => Config::load_or_default(None, Some(path_ref))?,
    };

    // 2. Build ScanArgs from options
    let args = build_scan_args(&options, path_ref);

    // 3. Run analysis engine
    let engine = AnalysisEngine::new(args, config)?;
    let report = engine.run()?;

    // 4. Build file info (for Plugin API)
    let files = build_file_info(&report, path_ref)?;

    // 5. Convert to ScanResult
    Ok(ScanResult::from_report(report, files, path_ref))
}

/// Load configuration from file or use defaults
pub fn load_config<P: AsRef<Path>>(path: Option<P>) -> Result<Config> {
    match path {
        Some(p) => Config::load(p.as_ref()),
        None => Config::load_or_default(None, None),
    }
}

/// Get list of all available detectors
pub fn get_detectors() -> Vec<DetectorInfo> {
    DetectorRegistry::new().list_all()
}

/// Clear the analysis cache for a project
pub fn clear_cache<P: AsRef<Path>>(path: P) -> Result<()> {
    crate::cache::AnalysisCache::clear(path.as_ref())
}

fn build_scan_args(options: &ScanOptions, path: &Path) -> ScanArgs {
    ScanArgs {
        path: path.to_path_buf(),
        lang: Language::TypeScript,
        config: options.config_path.clone(),
        report: None,
        format: OutputFormat::Table,
        json: true, // For structured output
        no_diagram: true,
        all_detectors: false,
        detectors: options.detectors.as_ref().map(|d| d.join(",")),
        exclude_detectors: if options.exclude_detectors.is_empty() {
            None
        } else {
            Some(options.exclude_detectors.join(","))
        },
        quiet: true,
        verbose: false,
        min_severity: options
            .min_severity
            .map(|s| format!("{:?}", s).to_lowercase()),
        min_score: options.min_score,
        severity: None,
        no_cache: !options.enable_cache,
        no_git: !options.enable_git,
        git_history_period: options.git_history_period.clone(),
        files: None,
    }
}

pub(crate) fn build_file_info(
    report: &crate::report::AnalysisReport,
    project_path: &Path,
) -> Result<Vec<FileInfo>> {
    let mut files = Vec::new();
    let graph = report.graph.as_ref();

    for (path, symbols) in &report.file_symbols {
        let relative_path = path
            .strip_prefix(project_path)
            .unwrap_or(path)
            .to_path_buf();

        let metrics = report.file_metrics.get(path);
        let complexities = report.function_complexity.get(path);

        let (fan_in, fan_out) = if let Some(g) = graph {
            if let Some(node) = g.get_node(path) {
                (g.fan_in(node), g.fan_out(node))
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        };

        let file_metrics = FileMetrics {
            lines: metrics.map(|m| m.lines).unwrap_or(0),
            complexity: complexities.and_then(|c| c.iter().map(|f| f.complexity).max()),
            fan_in,
            fan_out,
        };

        let imports = symbols
            .imports
            .iter()
            .map(|i| ImportInfo {
                source: i.source.to_string(),
                names: vec![i.name.to_string()],
                line: i.line,
                is_default: i.alias.is_none(),
                is_namespace: false,
            })
            .collect();

        let exports = symbols
            .exports
            .iter()
            .map(|e| ExportInfo {
                name: e.name.to_string(),
                kind: match e.kind {
                    crate::parser::SymbolKind::Function => ExportKind::Function,
                    crate::parser::SymbolKind::Class => ExportKind::Class,
                    crate::parser::SymbolKind::Variable => ExportKind::Variable,
                    crate::parser::SymbolKind::Type => ExportKind::Type,
                    crate::parser::SymbolKind::Interface => ExportKind::Interface,
                    crate::parser::SymbolKind::Enum => ExportKind::Enum,
                    crate::parser::SymbolKind::Unknown => ExportKind::Variable,
                },
                is_default: false,
                source: e.source.as_ref().map(|s| s.to_string()),
            })
            .collect();

        files.push(FileInfo {
            path: path.clone(),
            relative_path,
            imports,
            exports,
            metrics: file_metrics,
        });
    }

    Ok(files)
}
