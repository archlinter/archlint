mod storage;

use crate::Result;
use git2::{DiffOptions, Repository};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
pub use storage::{CommitData, GitStorage};

#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::indicatif::{ProgressBar, ProgressStyle};
#[cfg(feature = "cli")]
use indicatif::{ProgressBar, ProgressStyle};

pub struct GitHistoryCache {
    storage: GitStorage,
    repo: Repository,
    project_root: PathBuf,
}

impl GitHistoryCache {
    const CACHE_FILE: &'static str = "git-history.redb";

    pub fn open(project_root: &Path) -> Result<Self> {
        let repo = Repository::discover(project_root).map_err(|e| {
            crate::AnalysisError::Anyhow(anyhow::anyhow!("Git repository not found: {}", e))
        })?;

        let cache_dir = resolve_cache_dir(project_root);
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir)?;
        }
        let cache_file = cache_dir.join(Self::CACHE_FILE);

        let storage = GitStorage::open(&cache_file)?;

        Ok(Self {
            storage,
            repo,
            project_root: project_root.to_path_buf(),
        })
    }

    pub fn get_churn_map(
        &self,
        files: &[PathBuf],
        show_progress: bool,
    ) -> Result<HashMap<PathBuf, usize>> {
        let mut churn_map: HashMap<PathBuf, usize> = files.iter().map(|f| (f.clone(), 0)).collect();

        let mut revwalk = self.repo.revwalk()?;
        if revwalk.push_head().is_err() {
            return Ok(churn_map);
        }

        let pb = if show_progress {
            self.create_progress_bar()?
        } else {
            None
        };

        let workdir = self.repo.workdir().unwrap_or(&self.project_root);

        for oid in revwalk {
            let oid = oid?;
            self.process_single_oid(oid, &mut churn_map, workdir, pb.as_ref())?;
        }

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }

        Ok(churn_map)
    }

    fn process_single_oid(
        &self,
        oid: git2::Oid,
        churn_map: &mut HashMap<PathBuf, usize>,
        workdir: &Path,
        pb: Option<&ProgressBar>,
    ) -> Result<()> {
        let oid_bytes: [u8; 20] = oid.as_bytes().try_into().map_err(|_| {
            crate::AnalysisError::Anyhow(anyhow::anyhow!("Failed to convert OID to bytes"))
        })?;

        if let Some(pb) = pb {
            pb.inc(1);
        }

        let commit_data = if let Some(data) = self.storage.get_commit_data(&oid_bytes)? {
            data
        } else {
            let data = self.process_commit(oid)?;
            self.storage.insert_commit_data(&oid_bytes, &data)?;
            data
        };

        for file_path_str in commit_data.files_changed {
            let full_path = workdir.join(file_path_str);
            if let Ok(canonical_path) = full_path.canonicalize() {
                if let Some(count) = churn_map.get_mut(&canonical_path) {
                    *count += 1;
                }
            }
        }

        Ok(())
    }

    fn process_commit(&self, oid: git2::Oid) -> Result<CommitData> {
        let commit = self.repo.find_commit(oid)?;
        let mut files_changed = Vec::new();

        if commit.parent_count() == 0 {
            return Ok(CommitData { files_changed });
        }

        let parent = commit.parent(0)?;
        let current_tree = commit.tree()?;
        let parent_tree = parent.tree()?;

        let diff = self.repo.diff_tree_to_tree(
            Some(&parent_tree),
            Some(&current_tree),
            Some(DiffOptions::new().patience(true)),
        )?;

        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    if let Some(path_str) = path.to_str() {
                        files_changed.push(path_str.to_string());
                    }
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(CommitData { files_changed })
    }

    fn create_progress_bar(&self) -> Result<Option<ProgressBar>> {
        let mut count_revwalk = self.repo.revwalk()?;
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
        pb.set_message("Analyzing commits (cached)...");
        Ok(Some(pb))
    }

    pub fn clear(project_root: &Path) -> Result<()> {
        let cache_dir = resolve_cache_dir(project_root);
        let cache_file = cache_dir.join(Self::CACHE_FILE);
        if cache_file.exists() {
            std::fs::remove_file(cache_file)?;
        }
        Ok(())
    }
}

fn resolve_cache_dir(project_root: &Path) -> PathBuf {
    let node_modules = project_root.join("node_modules");
    if node_modules.exists() && node_modules.is_dir() {
        node_modules.join(".cache").join("archlint")
    } else {
        project_root.join(".archlint-cache")
    }
}
