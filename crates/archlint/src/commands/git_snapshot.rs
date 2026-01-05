use crate::api::options::ScanOptions;
use crate::api::Analyzer;
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::console::style;
use crate::snapshot::{Snapshot, SnapshotGenerator};
use crate::Result;
#[cfg(feature = "cli")]
use console::style;
use std::path::{Path, PathBuf};
use std::process::Command;

/// RAII wrapper for git worktree
struct TempWorktree {
    path: PathBuf,
    repo_path: PathBuf,
}

impl TempWorktree {
    fn create(repo_path: &Path, commit: &str) -> Result<Self> {
        // Generate unique temp path
        let temp_dir = std::env::temp_dir();
        let worktree_name = format!("archlint-{}-{}", &commit[..7], std::process::id());
        let worktree_path = temp_dir.join(&worktree_name);

        // Create detached worktree (doesn't create branch)
        let output = Command::new("git")
            .args([
                "worktree",
                "add",
                "--detach", // Don't create branch
                "--quiet",
                worktree_path.to_str().unwrap(),
                commit,
            ])
            .current_dir(repo_path)
            .output()?;

        if !output.status.success() {
            return Err(crate::AnalysisError::GitCommand(format!(
                "Failed to create worktree: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(Self {
            path: worktree_path,
            repo_path: repo_path.to_path_buf(),
        })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempWorktree {
    fn drop(&mut self) {
        // Remove worktree (git worktree remove --force)
        let _ = Command::new("git")
            .args(["worktree", "remove", "--force", self.path.to_str().unwrap()])
            .current_dir(&self.repo_path)
            .output();

        // Fallback: remove directory if git failed
        if self.path.exists() {
            let _ = std::fs::remove_dir_all(&self.path);
        }

        // Prune stale worktree entries
        let _ = Command::new("git")
            .args(["worktree", "prune"])
            .current_dir(&self.repo_path)
            .output();
    }
}

pub fn generate_snapshot_from_git_ref(
    git_ref: &str,
    project_path: &Path,
    silent: bool,
) -> Result<Snapshot> {
    // Resolve ref to commit hash using git2
    let repo = git2::Repository::discover(project_path)?;
    let obj = repo.revparse_single(git_ref).map_err(|e| {
        crate::AnalysisError::GitCommand(format!("Cannot resolve '{}': {}", git_ref, e))
    })?;
    let commit = obj.id().to_string();

    // Create temporary worktree
    let worktree = TempWorktree::create(project_path, &commit)?;

    if !silent {
        eprintln!(
            "Analyzing {} ({}) in temporary worktree...",
            style(git_ref).cyan(),
            style(&commit[..7]).dim()
        );
    }

    // Analyze the worktree
    let mut analyzer = Analyzer::new(worktree.path(), ScanOptions::default())?;
    let scan_result = analyzer.scan()?;

    // Generate snapshot (paths relative to worktree)
    let snapshot = SnapshotGenerator::new(worktree.path().to_path_buf())
        .with_commit(true)
        .generate(&scan_result);

    Ok(snapshot)
}
