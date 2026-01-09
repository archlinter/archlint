use crate::api::options::ScanOptions;
use crate::api::result::{IncrementalResult, ScanResult, SmellWithExplanation};
use crate::args::ScanArgs;
use crate::config::Config;
use crate::detectors::DetectorRegistry;
use crate::engine::context::AnalysisContext;
use crate::engine::AnalysisEngine;
use crate::error::Result;
use crate::framework::presets;
use crate::incremental::IncrementalState;
use crate::parser::{ImportParser, ParserConfig};
use crate::resolver::PathResolver;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

fn compute_config_hash(config: &Config) -> Result<String> {
    let serialized = serde_json::to_string(config)?;
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    Ok(format!("{:x}", hasher.finalize())[..16].to_string())
}

pub struct Analyzer {
    state: IncrementalState,
    args: ScanArgs,
    config: Config,
    project_root: PathBuf,
}

impl Analyzer {
    pub fn new<P: AsRef<Path>>(path: P, options: ScanOptions) -> Result<Self> {
        let path_ref = path.as_ref();
        let config = match (options.config.clone(), options.config_path.as_ref()) {
            (Some(cfg), _) => cfg,
            (None, Some(p)) => Config::load(p)?,
            (None, None) => Config::load_or_default(None, Some(path_ref))?,
        };

        let project_root = crate::project_root::detect_project_root(path_ref);
        let args = options.to_scan_args(path_ref);
        let config_hash = compute_config_hash(&config)?;

        Ok(Self {
            state: IncrementalState::new(project_root.clone(), config_hash),
            args,
            config,
            project_root,
        })
    }

    pub fn scan(&mut self) -> Result<ScanResult> {
        let engine = AnalysisEngine::new(self.args.clone(), self.config.clone())?;
        let report = engine.run()?;

        // Initialize state from report
        self.state.graph = Arc::new(report.graph.clone().unwrap_or_default());
        self.state.file_symbols = Arc::new(report.file_symbols.clone());
        self.state.file_metrics = Arc::new(report.file_metrics.clone());
        self.state.function_complexity = Arc::new(report.function_complexity.clone());
        self.state.churn_map = report.churn_map.clone();
        self.state.presets = report.presets.clone();
        self.state.last_full_scan = Some(Instant::now());

        // Update framework and project info
        let detected_frameworks =
            crate::framework::detector::FrameworkDetector::detect(&self.project_root);
        let pkg_config = crate::package_json::PackageJsonParser::parse(&self.project_root)?;

        self.state.detected_frameworks = detected_frameworks;
        self.state.script_entry_points = pkg_config.entry_points;
        self.state.dynamic_load_patterns = pkg_config.dynamic_load_patterns;

        // Build reverse deps
        self.state.reverse_deps.clear();
        for (from, to) in self.state.graph.edges() {
            if let (Some(from_path), Some(to_path)) = (
                self.state.graph.get_file_path(from),
                self.state.graph.get_file_path(to),
            ) {
                self.state
                    .reverse_deps
                    .entry(to_path.clone())
                    .or_default()
                    .insert(from_path.clone());
            }
        }

        let files = crate::api::build_file_info(&report, &self.project_root)?;
        Ok(ScanResult::from_report(report, files, &self.project_root))
    }

    pub fn scan_incremental(&mut self, changed: Vec<PathBuf>) -> Result<IncrementalResult> {
        self.scan_incremental_inner(changed, None)
    }

    /// Check if config changed and perform full rescan if needed
    fn check_config_and_full_rescan(
        &mut self,
        start: &Instant,
    ) -> Result<Option<IncrementalResult>> {
        let current_hash = compute_config_hash(&self.config)?;
        if current_hash == self.state.config_hash {
            return Ok(None);
        }

        log::info!("Config changed, triggering full rescan");
        self.state.config_hash = current_hash;
        let result = self.scan()?;
        Ok(Some(IncrementalResult {
            smells: result.smells,
            affected_files: result.files.iter().map(|f| f.path.clone()).collect(),
            changed_count: result.summary.files_analyzed,
            affected_count: result.summary.files_analyzed,
            analysis_time_ms: start.elapsed().as_millis() as u64,
        }))
    }

    /// Create analysis context from current state
    fn create_analysis_context(&self) -> AnalysisContext {
        AnalysisContext {
            project_path: self.project_root.clone(),
            graph: Arc::clone(&self.state.graph),
            file_symbols: Arc::clone(&self.state.file_symbols),
            function_complexity: Arc::clone(&self.state.function_complexity),
            file_metrics: Arc::clone(&self.state.file_metrics),
            churn_map: self.state.churn_map.clone(),
            config: self.config.clone(),
            script_entry_points: self.state.script_entry_points.clone(),
            dynamic_load_patterns: self.state.dynamic_load_patterns.clone(),
            detected_frameworks: self.state.detected_frameworks.clone(),
            presets: self.state.presets.clone(),
        }
    }

    fn cache_file_local_smells(
        &mut self,
        detector_id: &str,
        smells: &[crate::detectors::ArchSmell],
    ) {
        for smell in smells {
            for file in &smell.files {
                let cache_key = (detector_id.to_string(), file.clone());
                self.state
                    .file_local_cache
                    .entry(cache_key)
                    .or_default()
                    .push(smell.clone());
            }
        }
    }

    /// Run detectors and filter results to affected files
    fn run_detectors_incremental(
        &mut self,
        enabled_detectors: &[(String, Box<dyn crate::detectors::Detector>)],
        registry: &DetectorRegistry,
        ctx: &AnalysisContext,
        affected: &HashSet<PathBuf>,
        cache_file_local: bool,
    ) -> Vec<SmellWithExplanation> {
        let mut all_smells = Vec::new();

        for (detector_id, detector) in enabled_detectors {
            let info = registry.get_info(detector_id);
            let is_file_local = info
                .map(|i| i.category == crate::detectors::DetectorCategory::FileLocal)
                .unwrap_or(false);

            let smells = detector.detect(ctx);

            // Cache FileLocal results if requested
            if cache_file_local && is_file_local {
                self.cache_file_local_smells(detector_id, &smells);
            }

            // Filter to affected files
            for smell in smells {
                if smell.files.iter().any(|f| affected.contains(f)) {
                    let explanation = crate::explain::ExplainEngine::explain(&smell);
                    all_smells.push(SmellWithExplanation { smell, explanation });
                }
            }
        }

        all_smells
    }

    pub fn scan_incremental_with_overlays(
        &mut self,
        changed: Vec<PathBuf>,
        overlays: HashMap<PathBuf, String>,
    ) -> Result<IncrementalResult> {
        self.scan_incremental_inner(changed, Some(overlays))
    }

    /// Core incremental scan logic shared by both `scan_incremental` and `scan_incremental_with_overlays`
    fn scan_incremental_inner(
        &mut self,
        changed: Vec<PathBuf>,
        overlays: Option<HashMap<PathBuf, String>>,
    ) -> Result<IncrementalResult> {
        let start = Instant::now();
        let use_overlays = overlays.is_some();

        // Check if config changed - requires full rescan
        if let Some(result) = self.check_config_and_full_rescan(&start)? {
            return Ok(result);
        }

        // 1. Prepare parser and resolver
        let parser = ImportParser::new()?;
        let resolver = PathResolver::new(&self.project_root, &self.config);

        // Get active detectors to determine parser config
        let presets = presets::get_presets(&self.state.detected_frameworks);
        let registry = DetectorRegistry::new();
        let (enabled_detectors, _needs_deep) =
            registry.get_enabled_full(&self.config, &presets, self.args.all_detectors);

        let active_ids: HashSet<String> =
            enabled_detectors.iter().map(|(id, _)| id.clone()).collect();
        let parser_config = ParserConfig::from_active_detectors(&active_ids);

        // 2. Update state for changed files
        if let Some(ovl) = overlays {
            self.state.update_files_with_overlays(
                &changed,
                &ovl,
                &parser,
                &parser_config,
                &resolver,
                true, // skip_hash_update = true for overlay files
            )?;
        } else {
            self.state
                .update_files(&changed, &parser, &parser_config, &resolver)?;
        }

        // 3. Get affected files
        let affected = self.state.get_affected_files(&changed);
        let affected_count = affected.len();
        let changed_set: HashSet<PathBuf> = changed.iter().cloned().collect();

        // 4. Invalidate cache for changed files
        self.state
            .file_local_cache
            .retain(|(_, f), _| !changed_set.contains(f));

        // 5. Create context and run detectors (don't cache for overlays)
        let ctx = self.create_analysis_context();
        let cache_file_local = !use_overlays;
        let all_smells = self.run_detectors_incremental(
            &enabled_detectors,
            &registry,
            &ctx,
            &affected,
            cache_file_local,
        );

        Ok(IncrementalResult {
            smells: all_smells,
            affected_files: affected.into_iter().collect(),
            changed_count: changed.len(),
            affected_count,
            analysis_time_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Invalidate files (e.g., deleted files)
    pub fn invalidate(&mut self, files: &[PathBuf]) {
        for file in files {
            self.state.graph_mut().remove_file(file);
            self.state.file_symbols_mut().remove(file);
            self.state.file_metrics_mut().remove(file);
            self.state.function_complexity_mut().remove(file);
            self.state.file_hashes.remove(file);
            self.state.reverse_deps.remove(file);

            // Remove from reverse_deps of other files
            for importers in self.state.reverse_deps.values_mut() {
                importers.remove(file);
            }
        }
    }

    /// Force full rescan
    pub fn rescan(&mut self) -> Result<ScanResult> {
        self.state.clear();
        self.scan()
    }

    /// Get affected files without running detectors
    pub fn get_affected_files(&self, changed: &[PathBuf]) -> Vec<PathBuf> {
        self.state.get_affected_files(changed).into_iter().collect()
    }

    /// Get state statistics
    pub fn get_state_stats(&self) -> StateStats {
        StateStats {
            files_count: self.state.file_symbols.len(),
            graph_nodes: self.state.graph.node_count(),
            graph_edges: self.state.graph.edge_count(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateStats {
    pub files_count: usize,
    pub graph_nodes: usize,
    pub graph_edges: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::options::ScanOptions;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_analyzer_basic_flow() -> Result<()> {
        let dir = tempdir()?;
        let project_path = fs::canonicalize(dir.path())?;

        let a_ts = project_path.join("a.ts");
        let b_ts = project_path.join("b.ts");

        fs::write(&a_ts, "export const a = 1;")?;
        fs::write(&b_ts, "import { a } from './a'; export const b = a + 1;")?;

        let mut analyzer = Analyzer::new(&project_path, ScanOptions::default())?;

        // Initial scan
        let result = analyzer.scan()?;
        assert_eq!(result.summary.files_analyzed, 2);

        // Incremental scan (no changes)
        let inc_result = analyzer.scan_incremental(vec![])?;
        assert_eq!(inc_result.changed_count, 0);
        assert_eq!(inc_result.affected_count, 0);

        // Change a.ts
        fs::write(&a_ts, "export const a = 2;")?;
        let inc_result = analyzer.scan_incremental(vec![a_ts.clone()])?;

        assert_eq!(inc_result.changed_count, 1);
        // a.ts changed, b.ts imports a.ts -> both affected
        assert_eq!(inc_result.affected_count, 2);

        Ok(())
    }

    #[test]
    fn test_scan_incremental_with_overlays() -> Result<()> {
        let dir = tempdir()?;
        let project_path = fs::canonicalize(dir.path())?;

        let a_ts = project_path.join("a.ts");
        fs::write(&a_ts, "export const a = 1;")?;

        let mut analyzer = Analyzer::new(&project_path, ScanOptions::default())?;
        analyzer.scan()?;

        // Get initial hash
        let initial_hash = analyzer.state.file_hashes.get(&a_ts).cloned();

        // Analyze with overlay (different content)
        let mut overlays = HashMap::new();
        overlays.insert(a_ts.clone(), "export const a = 'changed';".to_string());

        let result = analyzer.scan_incremental_with_overlays(vec![a_ts.clone()], overlays)?;

        // Hash should NOT change (overlay doesn't affect cache)
        assert_eq!(analyzer.state.file_hashes.get(&a_ts), initial_hash.as_ref());

        // But analysis should work
        assert_eq!(result.changed_count, 1);
        assert_eq!(result.affected_count, 1);

        Ok(())
    }
}
