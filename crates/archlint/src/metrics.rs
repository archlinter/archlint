use git2::{Repository, DiffOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::Result;
use indicatif::{ProgressBar, ProgressStyle};

pub struct GitChurn {
    repo: Option<Repository>,
}

impl GitChurn {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        let repo = Repository::discover(root).ok();
        Self { repo }
    }

    pub fn calculate_churn(&self, files: &[PathBuf], show_progress: bool) -> Result<HashMap<PathBuf, usize>> {
        let mut churn_map = HashMap::new();

        if let Some(repo) = &self.repo {
            let mut revwalk = repo.revwalk()?;
            // Skip if no HEAD (empty repository)
            if revwalk.push_head().is_err() {
                return Ok(churn_map);
            }

            // Initialize churn for all files
            for file in files {
                churn_map.insert(file.clone(), 0);
            }

            let workdir = repo.workdir();

            let pb = if show_progress {
                // Count commits for progress bar
                let mut count_revwalk = repo.revwalk()?;
                if count_revwalk.push_head().is_err() {
                    return Ok(churn_map);
                }
                let total_commits = count_revwalk.count();

                let pb = ProgressBar::new(total_commits as u64);
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                        .unwrap()
                        .progress_chars("█▉▊▋▌▍▎▏  "),
                );
                pb.set_message("Analyzing commits...");
                Some(pb)
            } else {
                None
            };

            for oid in revwalk {
                let oid = oid?;
                if let Some(ref pb) = pb {
                    pb.inc(1);
                }
                let commit = repo.find_commit(oid)?;

                if commit.parent_count() == 0 {
                    continue;
                }

                let parent = commit.parent(0)?;
                let current_tree = commit.tree()?;
                let parent_tree = parent.tree()?;

                let diff = repo.diff_tree_to_tree(
                    Some(&parent_tree),
                    Some(&current_tree),
                    Some(DiffOptions::new().patience(true)),
                )?;

                diff.foreach(
                    &mut |delta, _| {
                        if let Some(path) = delta.new_file().path() {
                            if let Some(wd) = workdir {
                                let full_path = wd.join(path);

                                // Canonicalize path to match scanner's canonical paths
                                if let Ok(canonical_path) = full_path.canonicalize() {
                                    if let Some(count) = churn_map.get_mut(&canonical_path) {
                                        *count += 1;
                                    }
                                }
                            }
                        }
                        true
                    },
                    None,
                    None,
                    None,
                )?;
            }
            if let Some(pb) = pb {
                pb.finish_and_clear();
            }
        }

        Ok(churn_map)
    }

    pub fn is_available(&self) -> bool {
        self.repo.is_some()
    }
}

pub struct FileMetrics {
    pub fan_in: usize,
    pub fan_out: usize,
    pub churn: usize,
}

impl FileMetrics {
    pub fn new(fan_in: usize, fan_out: usize, churn: usize) -> Self {
        Self { fan_in, fan_out, churn }
    }
}
