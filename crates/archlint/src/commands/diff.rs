use super::diff_output::print_diff_result;
use super::git_snapshot::generate_snapshot_from_git_ref;
use crate::api::options::ScanOptions;
use crate::api::Analyzer;
use crate::diff::{DiffEngine, DiffResult};
use crate::snapshot::{read_snapshot, SnapshotGenerator};
use crate::Result;
use std::path::{Path, PathBuf};

pub fn run_diff(
    baseline: String,
    current: String,
    explain: bool,
    json: bool,
    fail_on: String,
    project_path: Option<PathBuf>,
) -> Result<i32> {
    let project_path = project_path
        .or_else(|| std::env::current_dir().ok())
        .ok_or(crate::AnalysisError::NoProjectPath)?;

    // Load baseline
    let baseline_snapshot = if is_file_path(&baseline) {
        read_snapshot(Path::new(&baseline))?
    } else {
        // Git ref - generate snapshot from that ref
        generate_snapshot_from_git_ref(&baseline, &project_path, json)?
    };

    // Load or generate current
    let current_snapshot = if current.is_empty() {
        // Analyze current state
        if !json {
            eprintln!("Analyzing current state...");
        }
        let mut analyzer = Analyzer::new(&project_path, ScanOptions::default())?;
        let scan_result = analyzer.scan()?;
        SnapshotGenerator::new(project_path.clone()).generate(&scan_result)
    } else if is_file_path(&current) {
        read_snapshot(Path::new(&current))?
    } else {
        generate_snapshot_from_git_ref(&current, &project_path, json)?
    };

    // Run diff
    let engine = DiffEngine::default();
    let result = if explain {
        engine.diff_with_explain(&baseline_snapshot, &current_snapshot)
    } else {
        engine.diff(&baseline_snapshot, &current_snapshot)
    };

    // Output
    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        print_diff_result(&result, explain);
    }

    // Exit code
    let exit_code = if should_fail(&result, &fail_on) { 1 } else { 0 };
    Ok(exit_code)
}

fn is_file_path(s: &str) -> bool {
    s.ends_with(".json") || Path::new(s).exists()
}

fn should_fail(result: &DiffResult, fail_on: &str) -> bool {
    if !result.has_regressions {
        return false;
    }

    let min_severity = match fail_on.to_lowercase().as_str() {
        "low" => 0,
        "medium" => 1,
        "high" => 2,
        "critical" => 3,
        _ => 0,
    };

    let severity_order = |s: &str| match s {
        "Low" => 0,
        "Medium" => 1,
        "High" => 2,
        "Critical" => 3,
        _ => 0,
    };

    result
        .regressions
        .iter()
        .any(|r| severity_order(&r.smell.severity) >= min_severity)
}
