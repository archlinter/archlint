use crate::config::Config;
use crate::framework::presets::FrameworkPreset;
use crate::framework::Framework;
use crate::graph::DependencyGraph;
use crate::parser::{FileIgnoredLines, FileSymbols, FunctionComplexity};
use crate::path_matcher::PathMatcher;
use crate::rule_resolver::ResolvedRuleConfig;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FileMetrics {
    pub lines: usize,
}

/// Contextual information provided to detectors during analysis.
///
/// This struct contains all the necessary data collected from the source code,
/// including the dependency graph, symbol maps, and project configuration.
pub struct AnalysisContext {
    /// Root directory of the project being analyzed.
    pub project_path: PathBuf,
    /// The full dependency graph of the project.
    pub graph: Arc<DependencyGraph>,
    /// Map of file paths to their extracted symbols (imports, exports, etc.).
    pub file_symbols: Arc<HashMap<PathBuf, FileSymbols>>,
    /// Map of file paths to the complexity details of their functions.
    pub function_complexity: Arc<HashMap<PathBuf, Vec<FunctionComplexity>>>,
    /// Basic metrics for each file (e.g., line count).
    pub file_metrics: Arc<HashMap<PathBuf, FileMetrics>>,
    /// Map of line numbers to ignore rules for each file.
    pub ignored_lines: Arc<FileIgnoredLines>,
    /// Map of file paths to their churn count (number of commits).
    pub churn_map: HashMap<PathBuf, usize>,
    /// Global configuration for the analysis.
    pub config: Config,
    /// Compiled form of the global `ignore:` list from [`Self::config`].
    pub ignored: PathMatcher,
    /// Set of entry points for script analysis.
    pub script_entry_points: HashSet<PathBuf>,
    /// Patterns used for identifying dynamic imports.
    pub dynamic_load_patterns: Vec<String>,
    /// List of frameworks detected in the project.
    pub detected_frameworks: Vec<Framework>,
    /// Active framework presets.
    pub presets: Vec<FrameworkPreset>,
    /// Directories containing package.json files.
    pub package_json_dirs: HashSet<PathBuf>,
}

impl AnalysisContext {
    /// Resolve a rule configuration for a specific detector and optional file path.
    #[must_use]
    pub fn resolve_rule(&self, detector_id: &str, file_path: Option<&Path>) -> ResolvedRuleConfig {
        ResolvedRuleConfig::resolve(&self.config, detector_id, file_path)
    }

    /// Check if a path should be excluded based on the provided patterns.
    #[must_use]
    pub fn is_excluded(&self, path: &Path, exclude_patterns: &[String]) -> bool {
        if exclude_patterns.is_empty() {
            return false;
        }

        let relative_path = path
            .strip_prefix(&self.project_path)
            .unwrap_or(path)
            .to_string_lossy();

        for pattern_str in exclude_patterns {
            if let Ok(pattern) = glob::Pattern::new(pattern_str) {
                if pattern.matches(&relative_path) {
                    return true;
                }
            }
        }

        false
    }

    /// Check if a path is covered by the global `ignore:` list.
    #[must_use]
    pub fn is_ignored(&self, path: &Path) -> bool {
        self.ignored.matches(path)
    }

    /// Resolve the rule for `path`, treating globally ignored files as invisible.
    ///
    /// This is the gate detectors use to decide whether a file participates in
    /// analysis at all, so ignored files never reach a threshold, a metric or a
    /// reported location.
    #[must_use]
    pub fn get_rule_for_file(&self, detector_id: &str, path: &Path) -> Option<ResolvedRuleConfig> {
        if self.is_ignored(path) {
            return None;
        }
        self.get_rule_for_file_keeping_ignored(detector_id, path)
    }

    /// Same as [`Self::get_rule_for_file`], but keeps globally ignored files visible.
    ///
    /// Used by detectors whose smell is a topological fact about a group of files
    /// (cycles): dropping one member turns the group into something that is not a
    /// cycle at all. Such detectors report the whole group and rely on the report
    /// filter to drop groups whose members are all ignored.
    #[must_use]
    pub fn get_rule_for_file_keeping_ignored(
        &self,
        detector_id: &str,
        path: &Path,
    ) -> Option<ResolvedRuleConfig> {
        let rule = self.resolve_rule(detector_id, Some(path));
        if !rule.enabled || self.is_excluded(path, &rule.exclude) {
            None
        } else {
            Some(rule)
        }
    }

    #[must_use]
    pub fn get_rule(&self, detector_id: &str) -> Option<ResolvedRuleConfig> {
        let rule = self.resolve_rule(detector_id, None);
        if rule.enabled {
            Some(rule)
        } else {
            None
        }
    }

    /// Build an empty context for tests.
    ///
    /// The global ignore list starts empty; a test that sets `config.ignore` must
    /// rebuild `ignored` from it (see [`PathMatcher::from_config`]).
    #[must_use]
    pub fn default_for_test() -> Self {
        Self {
            ignored: PathMatcher::default(),
            project_path: PathBuf::new(),
            graph: Arc::new(DependencyGraph::new()),
            file_symbols: Arc::new(HashMap::new()),
            function_complexity: Arc::new(HashMap::new()),
            file_metrics: Arc::new(HashMap::new()),
            ignored_lines: Arc::new(FileIgnoredLines::default()),
            churn_map: HashMap::new(),
            config: Config::default(),
            script_entry_points: HashSet::new(),
            dynamic_load_patterns: Vec::new(),
            detected_frameworks: Vec::new(),
            presets: Vec::new(),
            package_json_dirs: HashSet::new(),
        }
    }

    pub fn graph_mut(&mut self) -> &mut DependencyGraph {
        Arc::make_mut(&mut self.graph)
    }

    pub fn file_symbols_mut(&mut self) -> &mut HashMap<PathBuf, FileSymbols> {
        Arc::make_mut(&mut self.file_symbols)
    }

    pub fn function_complexity_mut(&mut self) -> &mut HashMap<PathBuf, Vec<FunctionComplexity>> {
        Arc::make_mut(&mut self.function_complexity)
    }

    pub fn file_metrics_mut(&mut self) -> &mut HashMap<PathBuf, FileMetrics> {
        Arc::make_mut(&mut self.file_metrics)
    }

    pub fn ignored_lines_mut(&mut self) -> &mut FileIgnoredLines {
        Arc::make_mut(&mut self.ignored_lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_excluded_empty() {
        let ctx = AnalysisContext::default_for_test();
        assert!(!ctx.is_excluded(Path::new("src/main.rs"), &[]));
    }

    #[test]
    fn test_is_excluded_glob() {
        let ctx = AnalysisContext::default_for_test();
        let patterns = vec!["src/*.test.rs".to_string(), "**/ignored/**".to_string()];

        assert!(ctx.is_excluded(Path::new("src/main.test.rs"), &patterns));
        assert!(ctx.is_excluded(Path::new("some/path/ignored/file.rs"), &patterns));
        assert!(!ctx.is_excluded(Path::new("src/main.rs"), &patterns));
    }

    fn ctx_ignoring(patterns: &[&str]) -> AnalysisContext {
        let mut ctx = AnalysisContext::default_for_test();
        ctx.project_path = PathBuf::from("/proj");
        ctx.config.ignore = patterns.iter().map(|p| (*p).to_string()).collect();
        ctx.ignored = PathMatcher::from_config(&ctx.config, &ctx.project_path);
        ctx
    }

    #[test]
    fn test_ignored_files_have_no_rule() {
        let ctx = ctx_ignoring(&["legacy"]);

        assert!(ctx.is_ignored(Path::new("/proj/legacy/a.ts")));
        assert!(ctx
            .get_rule_for_file("large_file", Path::new("/proj/legacy/a.ts"))
            .is_none());
        assert!(ctx
            .get_rule_for_file("large_file", Path::new("/proj/src/a.ts"))
            .is_some());
    }

    #[test]
    fn test_topological_detectors_still_see_ignored_files() {
        let ctx = ctx_ignoring(&["legacy"]);

        assert!(ctx
            .get_rule_for_file_keeping_ignored("cyclic_dependency", Path::new("/proj/legacy/a.ts"))
            .is_some());
    }
}
