use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use git2::{Commit, Repository};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct ShotgunSurgeryDetector;

pub struct ShotgunSurgeryDetectorFactory;

impl DetectorFactory for ShotgunSurgeryDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "shotgun_surgery",
            name: "Shotgun Surgery Detector",
            description: "Detects files that frequently change together",
            default_enabled: false, // Git analysis can be slow
            is_deep: true,
            category: DetectorCategory::Global,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(ShotgunSurgeryDetector)
    }
}

inventory::submit! {
    &ShotgunSurgeryDetectorFactory as &dyn DetectorFactory
}

struct CoChangeStats {
    total_co_changed: usize,
    commit_count: usize,
    frequently_co_changed: HashMap<PathBuf, usize>,
}

impl ShotgunSurgeryDetector {
    fn get_changed_files(
        &self,
        repo: &Repository,
        commit: &Commit,
    ) -> Result<HashSet<PathBuf>, git2::Error> {
        let mut changed = HashSet::new();
        let tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;
        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    changed.insert(path.to_path_buf());
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(changed)
    }

    fn is_source_code(&self, path: &Path) -> bool {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        matches!(ext, "ts" | "tsx" | "js" | "jsx")
    }

    fn analyze_co_changes(
        &self,
        ctx: &AnalysisContext,
        lookback: usize,
    ) -> Result<HashMap<PathBuf, CoChangeStats>, Box<dyn std::error::Error>> {
        if !ctx.config.enable_git {
            return Ok(HashMap::new());
        }

        let repo = match Repository::discover(&ctx.project_path) {
            Ok(r) => r,
            Err(_) => return Ok(HashMap::new()),
        };

        let mut revwalk = repo.revwalk()?;
        if revwalk.push_head().is_err() {
            return Ok(HashMap::new());
        }

        let mut stats: HashMap<PathBuf, CoChangeStats> = HashMap::new();

        for oid in revwalk.take(lookback) {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            Self::process_commit(&repo, &commit, &mut stats, self)?;
        }

        Ok(stats)
    }

    fn process_commit(
        repo: &Repository,
        commit: &Commit,
        stats: &mut HashMap<PathBuf, CoChangeStats>,
        detector: &ShotgunSurgeryDetector,
    ) -> Result<(), git2::Error> {
        let changed_files = detector.get_changed_files(repo, commit)?;
        let source_files: HashSet<PathBuf> = changed_files
            .into_iter()
            .filter(|p| detector.is_source_code(p))
            .collect();

        if Self::should_process_commit(&source_files) {
            Self::update_stats(stats, &source_files);
        }

        Ok(())
    }

    fn should_process_commit(source_files: &HashSet<PathBuf>) -> bool {
        let count = source_files.len();
        count > 1 && count < 50
    }

    fn update_stats(stats: &mut HashMap<PathBuf, CoChangeStats>, source_files: &HashSet<PathBuf>) {
        for file in source_files {
            let entry = stats.entry(file.clone()).or_insert(CoChangeStats {
                total_co_changed: 0,
                commit_count: 0,
                frequently_co_changed: HashMap::new(),
            });

            entry.commit_count += 1;
            entry.total_co_changed += source_files.len() - 1;

            for other in source_files {
                if file != other {
                    *entry
                        .frequently_co_changed
                        .entry(other.clone())
                        .or_insert(0) += 1;
                }
            }
        }
    }
}

impl Detector for ShotgunSurgeryDetector {
    fn name(&self) -> &'static str {
        "ShotgunSurgery"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let rule = ctx.resolve_rule("shotgun_surgery", None);
        if !rule.enabled {
            return Vec::new();
        }

        let lookback: usize = rule.get_option("lookback_commits").unwrap_or(500);
        let min_frequency: usize = rule.get_option("min_frequency").unwrap_or(5);
        let min_co_changes: usize = rule.get_option("min_co_changes").unwrap_or(3);

        let stats = match self.analyze_co_changes(ctx, lookback) {
            Ok(s) => s,
            Err(_) => return Vec::new(), // If git analysis fails, skip
        };

        stats
            .into_iter()
            .filter_map(|(file, stat)| {
                let file_rule = ctx.resolve_rule("shotgun_surgery", Some(&file));
                if !file_rule.enabled || ctx.is_excluded(&file, &file_rule.exclude) {
                    return None;
                }

                // Only consider source code files that are part of the project
                if !self.is_source_code(&file) || !ctx.file_symbols.contains_key(&file) {
                    return None;
                }

                let avg_co_changes = stat.total_co_changed as f64 / stat.commit_count as f64;

                if stat.commit_count >= min_frequency && avg_co_changes >= min_co_changes as f64 {
                    let mut frequently_co_changed: Vec<(PathBuf, usize)> = stat
                        .frequently_co_changed
                        .into_iter()
                        .filter(|(p, count)| {
                            *count >= min_frequency
                                && self.is_source_code(p)
                                && ctx.file_symbols.contains_key(p)
                        })
                        .collect();

                    frequently_co_changed.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

                    let top_co_changed = frequently_co_changed.into_iter().take(5).collect();

                    let mut smell =
                        ArchSmell::new_shotgun_surgery(file, avg_co_changes, top_co_changed);
                    smell.severity = file_rule.severity;
                    Some(smell)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn init() {}
