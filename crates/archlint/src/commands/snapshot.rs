use crate::api::options::ScanOptions;
use crate::api::Analyzer;
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::console::style;
use crate::snapshot::{write_snapshot, SnapshotGenerator};
use crate::Result;
#[cfg(feature = "cli")]
use console::style;
use std::path::PathBuf;

pub fn run_snapshot(
    output: PathBuf,
    include_commit: bool,
    project_path: Option<PathBuf>,
) -> Result<()> {
    let project_path = project_path
        .or_else(|| std::env::current_dir().ok())
        .ok_or(crate::AnalysisError::NoProjectPath)?;

    eprintln!("Analyzing {}...", style(project_path.display()).cyan());

    // Run analysis
    let mut analyzer = Analyzer::new(&project_path, ScanOptions::default())?;
    let scan_result = analyzer.scan()?;

    // Generate snapshot
    let generator = SnapshotGenerator::new(project_path).with_commit(include_commit);
    let snapshot = generator.generate(&scan_result);

    // Write to file
    write_snapshot(&snapshot, &output)?;

    eprintln!(
        "{} Snapshot written to {}",
        style("✔").green(),
        style(output.display()).bold()
    );
    eprintln!(
        "  Smells: {}",
        style(snapshot.smells.len().to_string()).yellow()
    );
    eprintln!("  Grade:  {}", style(snapshot.grade).green());
    if let Some(commit) = &snapshot.commit {
        eprintln!("  Commit: {}", style(commit).dim());
    }

    Ok(())
}
