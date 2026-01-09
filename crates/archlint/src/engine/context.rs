use crate::config::Config;
use crate::framework::presets::FrameworkPreset;
use crate::framework::Framework;
use crate::graph::DependencyGraph;
use crate::parser::{FileSymbols, FunctionComplexity};
use crate::rule_resolver::ResolvedRuleConfig;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FileMetrics {
    pub lines: usize,
}

pub struct AnalysisContext {
    pub project_path: PathBuf,
    // Heavy structures wrapped in Arc
    pub graph: Arc<DependencyGraph>,
    pub file_symbols: Arc<HashMap<PathBuf, FileSymbols>>,
    pub function_complexity: Arc<HashMap<PathBuf, Vec<FunctionComplexity>>>,
    pub file_metrics: Arc<HashMap<PathBuf, FileMetrics>>,
    // Small, keep owned
    pub churn_map: HashMap<PathBuf, usize>,
    pub config: Config,
    pub script_entry_points: HashSet<PathBuf>,
    pub dynamic_load_patterns: Vec<String>,
    pub detected_frameworks: Vec<Framework>,
    pub presets: Vec<FrameworkPreset>,
}

impl AnalysisContext {
    pub fn resolve_rule(&self, detector_id: &str, file_path: Option<&Path>) -> ResolvedRuleConfig {
        ResolvedRuleConfig::resolve(&self.config, detector_id, file_path)
    }

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

    pub fn get_rule_for_file(&self, detector_id: &str, path: &Path) -> Option<ResolvedRuleConfig> {
        let rule = self.resolve_rule(detector_id, Some(path));
        if !rule.enabled || self.is_excluded(path, &rule.exclude) {
            None
        } else {
            Some(rule)
        }
    }

    pub fn get_rule(&self, detector_id: &str) -> Option<ResolvedRuleConfig> {
        let rule = self.resolve_rule(detector_id, None);
        if !rule.enabled {
            None
        } else {
            Some(rule)
        }
    }

    pub fn default_for_test() -> Self {
        Self {
            project_path: PathBuf::new(),
            graph: Arc::new(DependencyGraph::new()),
            file_symbols: Arc::new(HashMap::new()),
            function_complexity: Arc::new(HashMap::new()),
            file_metrics: Arc::new(HashMap::new()),
            churn_map: HashMap::new(),
            config: Config::default(),
            script_entry_points: HashSet::new(),
            dynamic_load_patterns: Vec::new(),
            detected_frameworks: Vec::new(),
            presets: Vec::new(),
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
}
