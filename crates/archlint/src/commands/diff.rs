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

    let baseline_snapshot = load_baseline(&baseline, &project_path, json)?;
    let current_snapshot = load_current(&current, &project_path, json)?;

    let config = crate::config::Config::load_or_default(None, Some(&project_path))?;
    let engine = DiffEngine::new()
        .with_threshold(config.diff.metric_threshold_percent)
        .with_line_tolerance(config.diff.line_tolerance);

    let result = if explain {
        engine.diff_with_explain(&baseline_snapshot, &current_snapshot)
    } else {
        engine.diff(&baseline_snapshot, &current_snapshot)
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        print_diff_result(&result, explain);
    }

    Ok(if should_fail(&result, &fail_on) { 1 } else { 0 })
}

fn load_baseline(
    baseline: &str,
    project_path: &Path,
    json: bool,
) -> Result<crate::snapshot::Snapshot> {
    if is_file_path(baseline) {
        read_snapshot(Path::new(baseline)).map_err(Into::into)
    } else {
        generate_snapshot_from_git_ref(baseline, project_path, json)
    }
}

fn load_current(
    current: &str,
    project_path: &Path,
    json: bool,
) -> Result<crate::snapshot::Snapshot> {
    if current.is_empty() {
        if !json {
            eprintln!("Analyzing current state...");
        }
        let mut analyzer = Analyzer::new(project_path, ScanOptions::default())?;
        let scan_result = analyzer.scan()?;
        Ok(SnapshotGenerator::new(project_path.to_path_buf()).generate(&scan_result))
    } else if is_file_path(current) {
        read_snapshot(Path::new(current)).map_err(Into::into)
    } else {
        generate_snapshot_from_git_ref(current, project_path, json)
    }
}

fn is_file_path(s: &str) -> bool {
    s.ends_with(".json") || Path::new(s).exists()
}

fn should_fail(result: &DiffResult, fail_on: &str) -> bool {
    if !result.has_regressions {
        return false;
    }

    let min_severity = parse_severity_threshold(fail_on);

    result
        .regressions
        .iter()
        .any(|r| get_severity_score(&r.smell.severity) >= min_severity)
}

fn parse_severity_threshold(fail_on: &str) -> i32 {
    match fail_on.to_lowercase().as_str() {
        "low" => 0,
        "medium" => 1,
        "high" => 2,
        "critical" => 3,
        _ => 0,
    }
}

fn get_severity_score(severity: &str) -> i32 {
    match severity.to_lowercase().as_str() {
        "low" => 0,
        "medium" => 1,
        "high" => 2,
        "critical" => 3,
        _ => 0,
    }
}
