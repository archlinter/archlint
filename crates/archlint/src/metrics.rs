use crate::Result;
use git2::{DiffOptions, Repository};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct GitChurn {
    repo: Option<Repository>,
}

impl GitChurn {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        let repo = Repository::discover(root).ok();
        Self { repo }
    }

    pub fn calculate_churn(
        &self,
        files: &[PathBuf],
        show_progress: bool,
    ) -> Result<HashMap<PathBuf, usize>> {
        let mut churn_map = Self::init_churn_map(files);

        if let Some(repo) = &self.repo {
            let mut revwalk = repo.revwalk()?;
            if revwalk.push_head().is_err() {
                return Ok(churn_map);
            }

            let pb = Self::create_progress_bar(repo, show_progress)?;
            let workdir = repo.workdir();

            for oid in revwalk {
                let oid = oid?;
                if let Some(ref pb) = pb {
                    pb.inc(1);
                }

                Self::process_commit(repo, oid, &mut churn_map, workdir)?;
            }

            if let Some(pb) = pb {
                pb.finish_and_clear();
            }
        }

        Ok(churn_map)
    }

    fn init_churn_map(files: &[PathBuf]) -> HashMap<PathBuf, usize> {
        files.iter().map(|file| (file.clone(), 0)).collect()
    }

    fn create_progress_bar(repo: &Repository, show_progress: bool) -> Result<Option<ProgressBar>> {
        if !show_progress {
            return Ok(None);
        }

        let mut count_revwalk = repo.revwalk()?;
        if count_revwalk.push_head().is_err() {
            return Ok(None);
        }
        let total_commits = count_revwalk.count();

        let pb = ProgressBar::new(total_commits as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏  "),
        );
        pb.set_message("Analyzing commits...");
        Ok(Some(pb))
    }

    fn process_commit(
        repo: &Repository,
        oid: git2::Oid,
        churn_map: &mut HashMap<PathBuf, usize>,
        workdir: Option<&Path>,
    ) -> Result<()> {
        let commit = repo.find_commit(oid)?;

        if commit.parent_count() == 0 {
            return Ok(());
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
                Self::update_churn_for_file(&delta, churn_map, workdir);
                true
            },
            None,
            None,
            None,
        )?;

        Ok(())
    }

    fn update_churn_for_file(
        delta: &git2::DiffDelta<'_>,
        churn_map: &mut HashMap<PathBuf, usize>,
        workdir: Option<&Path>,
    ) {
        if let Some(path) = delta.new_file().path() {
            if let Some(wd) = workdir {
                let full_path = wd.join(path);
                if let Ok(canonical_path) = full_path.canonicalize() {
                    if let Some(count) = churn_map.get_mut(&canonical_path) {
                        *count += 1;
                    }
                }
            }
        }
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
        Self {
            fan_in,
            fan_out,
            churn,
        }
    }
}
