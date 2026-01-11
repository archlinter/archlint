use crate::args::{Language, ScanArgs};
use crate::cache::hash::file_content_hash;
use crate::cache::AnalysisCache;
use crate::config::{Config, RuleConfig};
use crate::detectors::{self, Severity, SmellType};
use crate::engine::builder::EngineBuilder;
use crate::engine::detector_runner::{apply_arg_overrides, DetectorRunner};
use crate::engine::progress::{
    create_progress_bar, default_progress_chars, default_spinner_template,
};
use crate::engine::AnalysisContext;
use crate::framework::detector::FrameworkDetector;
use crate::framework::preset_loader::PresetLoader;
use crate::framework::presets::FrameworkPreset;
use crate::framework::Framework;
use crate::git_cache::GitHistoryCache;
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::console::{style, Term};
use crate::package_json;
use crate::parser::{FileIgnoredLines, ImportParser, ParsedFile, ParserConfig};
use crate::project_root::detect_project_root;
use crate::report::AnalysisReport;
use crate::scanner::FileScanner;
use crate::Result;
#[cfg(feature = "cli")]
use console::{style, Term};
use log::{debug, info};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct AnalysisEngine {
    pub args: ScanArgs,
    pub config: Config,
    pub project_root: PathBuf,
    pub target_path: PathBuf,
}

impl AnalysisEngine {
    pub fn new(args: ScanArgs, config: Config) -> Result<Self> {
        let target_path = args
            .path
            .canonicalize()
            .unwrap_or_else(|_| args.path.clone());
        let project_root = detect_project_root(&target_path);

        let mut config = config;
        apply_arg_overrides(&args, &mut config);

        Ok(Self {
            args,
            config,
            project_root,
            target_path,
        })
    }

    pub fn new_with_args(args: ScanArgs) -> Result<Self> {
        let target_path = args
            .path
            .canonicalize()
            .unwrap_or_else(|_| args.path.clone());
        let project_root = detect_project_root(&target_path);
        let config = Config::load_or_default(args.config.as_deref(), Some(&project_root))?;
        Self::new(args, config)
    }

    pub fn run(&self) -> Result<AnalysisReport> {
        let is_tty = Term::stdout().is_term();
        let use_progress = is_tty && !self.args.is_quiet();

        self.log_start();

        let files = self.discover_files()?;
        let (presets, detected_frameworks) = self.load_presets_and_detect()?;

        let final_config = self.apply_presets(&presets);

        let detector_runner = DetectorRunner::new(&self.args);
        let active_ids = detector_runner.get_active_detectors(&final_config, &presets);
        let parser_config = ParserConfig::from_active_detectors(&active_ids);

        let mut cache = self.load_cache()?;
        let parsed_files = self.parse_files(&files, &parser_config, use_progress, &cache)?;
        self.update_cache(&mut cache, &parsed_files)?;

        let (file_symbols, function_complexity, file_metrics, ignored_lines) =
            self.extract_parsed_data(parsed_files);
        let runtime_files = self.get_runtime_files(&file_symbols);

        self.log_runtime_info(runtime_files.len(), files.len());

        let builder = EngineBuilder::new(&self.project_root, &final_config);
        let graph = builder.build_graph(&runtime_files, &file_symbols, use_progress)?;
        let churn_map = self.get_churn_map(&files, use_progress, &mut cache);
        let resolved_file_symbols = builder.resolve_symbols(file_symbols, use_progress);

        let pkg_config = package_json::PackageJsonParser::parse(&self.project_root)?;

        let ctx = AnalysisContext {
            project_path: self.project_root.clone(),
            graph: Arc::new(graph),
            file_symbols: Arc::new(resolved_file_symbols),
            function_complexity: Arc::new(function_complexity),
            file_metrics: Arc::new(file_metrics),
            ignored_lines: Arc::new(ignored_lines),
            churn_map,
            config: final_config.clone(),
            script_entry_points: pkg_config.entry_points,
            dynamic_load_patterns: pkg_config.dynamic_load_patterns,
            detected_frameworks,
            presets: presets.clone(),
        };

        let all_smells = detector_runner.run_detectors(&ctx, use_progress, &presets)?;

        let report = self.create_report(ctx, all_smells, files.len(), presets)?;

        if let Some(c) = cache {
            debug!("Saving cache...");
            c.save()?;
        }

        Ok(report)
    }

    fn create_report(
        &self,
        ctx: AnalysisContext,
        all_smells: Vec<detectors::ArchSmell>,
        files_len: usize,
        presets: Vec<FrameworkPreset>,
    ) -> Result<AnalysisReport> {
        let filtered_smells: Vec<_> = all_smells
            .into_iter()
            .filter(|smell| {
                // Check if smell is ignored by inline comments
                if self.is_smell_ignored_by_comments(smell, &ctx.ignored_lines) {
                    return false;
                }

                // Clones are special: we want to see them even if they touch ignored files
                if matches!(smell.smell_type, SmellType::CodeClone { .. }) {
                    return true;
                }

                // Keep the smell if at least one of the files it's associated with is NOT ignored via config
                smell.files.is_empty() || smell.files.iter().any(|f| !self.is_file_ignored(f))
            })
            .collect();

        let mut report = AnalysisReport::new(
            filtered_smells,
            Some(Arc::try_unwrap(ctx.graph).unwrap_or_else(|arc| (*arc).clone())),
            Arc::try_unwrap(ctx.file_symbols).unwrap_or_else(|arc| (*arc).clone()),
            Arc::try_unwrap(ctx.file_metrics).unwrap_or_else(|arc| (*arc).clone()),
            Arc::try_unwrap(ctx.function_complexity).unwrap_or_else(|arc| (*arc).clone()),
            Arc::try_unwrap(ctx.ignored_lines).unwrap_or_else(|arc| (*arc).clone()),
            ctx.churn_map,
            presets,
            &self.config,
        );
        report.set_files_analyzed(files_len);

        if let Some(ref min_sev) = self.args.min_severity {
            use std::str::FromStr;
            if let Ok(s) = Severity::from_str(min_sev) {
                report.set_min_severity(s);
            }
        }
        if let Some(min_score) = self.args.min_score {
            report.set_min_score(min_score);
        }

        report.apply_severity_config(&self.config.scoring);

        Ok(report)
    }

    fn load_presets_and_detect(&self) -> Result<(Vec<FrameworkPreset>, Vec<Framework>)> {
        let mut presets = Vec::new();
        self.load_explicit_presets(&mut presets)?;
        let detected_frameworks = self.auto_detect_and_load_presets(&mut presets);
        Ok((presets, detected_frameworks))
    }

    fn load_explicit_presets(&self, presets: &mut Vec<FrameworkPreset>) -> Result<()> {
        for preset_name in &self.config.extends {
            match PresetLoader::load_any(preset_name) {
                Ok(p) => presets.push(p),
                Err(e) => {
                    return Err(
                        anyhow::anyhow!("Failed to load preset '{}': {}", preset_name, e).into(),
                    );
                }
            }
        }

        if let Some(ref fw) = self.config.framework {
            if !self.config.extends.contains(fw) {
                if let Ok(p) = PresetLoader::load_any(fw) {
                    presets.push(p);
                }
            }
        }

        Ok(())
    }

    fn auto_detect_and_load_presets(&self, presets: &mut Vec<FrameworkPreset>) -> Vec<Framework> {
        if !self.config.auto_detect_framework {
            return Vec::new();
        }

        let detected = FrameworkDetector::detect(&self.project_root);
        if detected.is_empty() {
            return Vec::new();
        }

        info!(
            "{}  Detected frameworks: {}",
            style("🛠️").magenta().bold(),
            style(
                detected
                    .iter()
                    .map(|f| format!("{:?}", f))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .yellow()
        );

        for fw in &detected {
            let name = match fw {
                Framework::NestJS => "nestjs",
                Framework::NextJS => "nextjs",
                Framework::React => "react",
                Framework::Oclif => "oclif",
                _ => continue,
            };
            if !self.config.extends.contains(&name.to_string()) {
                if let Ok(p) = PresetLoader::load_builtin(name) {
                    presets.push(p);
                }
            }
        }

        detected
    }

    fn log_start(&self) {
        info!(
            "{} Scanning target: {}",
            style("🔍").cyan().bold(),
            style(self.target_path.display()).bold()
        );
        debug!(
            "{} Project root: {}",
            style("🏠").dim(),
            style(self.project_root.display()).dim()
        );
    }

    fn discover_files(&self) -> Result<Vec<PathBuf>> {
        let extensions = match self.args.lang {
            Language::TypeScript => vec!["ts".to_string(), "tsx".to_string()],
            Language::JavaScript => vec!["js".to_string(), "jsx".to_string()],
        };

        let all_files = if let Some(ref explicit_files) = self.args.files {
            explicit_files.clone()
        } else {
            let scanner = FileScanner::new(&self.project_root, &self.target_path, extensions);
            scanner.scan()?
        };

        let mut files = Vec::new();
        let mut skipped_large = 0;
        let max_size = self.config.max_file_size;

        for path in all_files {
            if let Ok(metadata) = std::fs::metadata(&path) {
                if metadata.len() > max_size {
                    debug!(
                        "Skipping large file: {} ({} bytes)",
                        path.display(),
                        metadata.len()
                    );
                    skipped_large += 1;
                    continue;
                }
            }
            files.push(path);
        }

        if skipped_large > 0 {
            info!(
                "{} Skipped {} files exceeding max_file_size ({} bytes)",
                style("⚠️").yellow().bold(),
                style(skipped_large).yellow(),
                style(max_size).dim()
            );
        }

        info!(
            "{} Found {} files to analyze",
            style("📁").blue().bold(),
            style(files.len()).yellow()
        );
        Ok(files)
    }

    fn is_file_ignored(&self, path: &Path) -> bool {
        if self.config.ignore.is_empty() {
            return false;
        }
        let rel_path = path
            .strip_prefix(&self.project_root)
            .unwrap_or(path)
            .to_string_lossy();
        self.config.ignore.iter().any(|p| {
            glob::Pattern::new(p)
                .map(|pattern| pattern.matches(&rel_path))
                .unwrap_or(false)
        })
    }

    fn apply_presets(&self, presets: &[FrameworkPreset]) -> Config {
        let mut final_config = self.config.clone();
        for preset in presets {
            self.merge_preset_into_config(&mut final_config, preset);
        }
        final_config
    }

    fn merge_preset_into_config(&self, config: &mut Config, preset: &FrameworkPreset) {
        for (rule_name, preset_rule) in &preset.rules {
            let user_rule = config
                .rules
                .entry(rule_name.clone())
                .or_insert_with(|| preset_rule.clone());

            if let (RuleConfig::Full(preset_full), RuleConfig::Full(user_rule_full)) =
                (preset_rule, user_rule)
            {
                Self::merge_options(&mut user_rule_full.options, &preset_full.options);
            }
        }

        for pattern in &preset.entry_points {
            if !config.entry_points.contains(pattern) {
                config.entry_points.push(pattern.clone());
            }
        }

        for ov in &preset.overrides {
            if !config.overrides.contains(ov) {
                config.overrides.push(ov.clone());
            }
        }
    }

    fn merge_options(user_options: &mut serde_yaml::Value, preset_options: &serde_yaml::Value) {
        let (Some(user_map), Some(preset_map)) =
            (user_options.as_mapping_mut(), preset_options.as_mapping())
        else {
            return;
        };

        for (key, preset_val) in preset_map {
            if !user_map.contains_key(key) {
                user_map.insert(key.clone(), preset_val.clone());
            } else {
                let user_val = user_map.get_mut(key).unwrap();
                Self::merge_sequences(user_val, preset_val);
            }
        }
    }

    fn merge_sequences(user_val: &mut serde_yaml::Value, preset_val: &serde_yaml::Value) {
        if let (Some(user_seq), Some(preset_seq)) =
            (user_val.as_sequence_mut(), preset_val.as_sequence())
        {
            for item in preset_seq {
                if !user_seq.contains(item) {
                    user_seq.push(item.clone());
                }
            }
        }
    }

    fn load_cache(&self) -> Result<Option<AnalysisCache>> {
        if !self.args.no_cache {
            debug!("Loading cache...");
            Ok(Some(AnalysisCache::load(&self.project_root, &self.config)?))
        } else {
            Ok(None)
        }
    }

    fn parse_files(
        &self,
        files: &[PathBuf],
        config: &ParserConfig,
        use_progress: bool,
        cache: &Option<AnalysisCache>,
    ) -> Result<HashMap<PathBuf, ParsedFile>> {
        let parser = ImportParser::new()?;
        let pb = if use_progress {
            Some(create_progress_bar(
                files.len(),
                default_spinner_template(),
                default_progress_chars(),
            ))
        } else {
            None
        };

        let result = files
            .par_iter()
            .map(|file| {
                let hash = file_content_hash(file)?;
                if let Some(ref c) = cache {
                    if let Some(cached) = c.get(file, &hash) {
                        if let Some(ref pb) = pb {
                            pb.inc(1);
                        }
                        return Ok((file.clone(), (*cached).clone()));
                    }
                }
                let parsed = parser.parse_file_with_config(file, config)?;
                if let Some(ref pb) = pb {
                    pb.inc(1);
                }
                Ok((file.clone(), parsed))
            })
            .collect::<Result<HashMap<_, _>>>();

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        result
    }

    fn update_cache(
        &self,
        cache: &mut Option<AnalysisCache>,
        parsed_files: &HashMap<PathBuf, ParsedFile>,
    ) -> Result<()> {
        if let Some(ref mut c) = cache {
            for (file, parsed) in parsed_files {
                let hash = file_content_hash(file)?;
                c.insert(file.clone(), hash, (*parsed).clone());
            }
        }
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn extract_parsed_data(
        &self,
        parsed_files: HashMap<PathBuf, ParsedFile>,
    ) -> (
        HashMap<PathBuf, crate::parser::FileSymbols>,
        HashMap<PathBuf, Vec<crate::parser::FunctionComplexity>>,
        HashMap<PathBuf, crate::engine::context::FileMetrics>,
        FileIgnoredLines,
    ) {
        let mut symbols = HashMap::new();
        let mut complexity = HashMap::new();
        let mut metrics = HashMap::new();
        let mut ignored = FileIgnoredLines::default();
        for (file, parsed) in parsed_files {
            symbols.insert(file.clone(), parsed.symbols);
            complexity.insert(file.clone(), parsed.functions);
            metrics.insert(
                file.clone(),
                crate::engine::context::FileMetrics {
                    lines: parsed.lines,
                },
            );
            ignored.insert(file, parsed.ignored_lines);
        }
        (symbols, complexity, metrics, ignored)
    }

    fn is_smell_ignored_by_comments(
        &self,
        smell: &detectors::ArchSmell,
        ignored_lines: &FileIgnoredLines,
    ) -> bool {
        let rule_id = smell.smell_type.category().to_id();

        for loc in &smell.locations {
            if self.is_ignored(&loc.file, loc.line, rule_id, ignored_lines) {
                return true;
            }
        }

        if smell.locations.is_empty() {
            for file in &smell.files {
                if self.is_ignored(file, 0, rule_id, ignored_lines) {
                    return true;
                }
            }
        }

        false
    }

    fn is_ignored(
        &self,
        file: &PathBuf,
        line: usize,
        rule_id: &str,
        ignored_lines: &FileIgnoredLines,
    ) -> bool {
        let file_ignores = match ignored_lines.get(file) {
            Some(ignores) => ignores,
            None => return false,
        };

        if line == 0 {
            return file_ignores
                .get(&0)
                .is_some_and(|rules| rules.contains("*") || rules.contains(rule_id));
        }

        file_ignores
            .get(&line)
            .is_some_and(|rules| rules.contains("*") || rules.contains(rule_id))
    }

    fn get_runtime_files(
        &self,
        symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
    ) -> HashSet<PathBuf> {
        symbols
            .iter()
            .filter(|(_, s)| s.has_runtime_code)
            .map(|(p, _)| p.clone())
            .collect()
    }

    fn log_runtime_info(&self, runtime_count: usize, total_count: usize) {
        info!(
            "{} Runtime code found in {} files (skipped {} type-only)",
            style("💎").magenta().bold(),
            style(runtime_count).cyan(),
            style(total_count - runtime_count).dim()
        );
    }

    fn get_churn_map(
        &self,
        files: &[PathBuf],
        use_progress: bool,
        cache: &mut Option<AnalysisCache>,
    ) -> HashMap<PathBuf, usize> {
        info!("{} Calculating metrics...", style("📊").blue().bold());
        if !self.config.git.enabled {
            debug!("Git integration disabled, skipping churn calculation");
            return HashMap::new();
        }
        if let Some(cached_churn) = cache.as_ref().and_then(|c| c.get_churn_map()) {
            debug!("Using cached churn map from AnalysisCache");
            return cached_churn.clone();
        }
        match GitHistoryCache::open(&self.project_root) {
            Ok(git_cache) => {
                match git_cache.get_churn_map(files, use_progress, &self.config.git.history_period)
                {
                    Ok(map) => {
                        if let Some(ref mut c) = cache {
                            c.insert_churn_map(map.clone());
                        }
                        map
                    }
                    Err(e) => {
                        debug!("Git history cache calculation failed: {}, skipping", e);
                        HashMap::new()
                    }
                }
            }
            Err(e) => {
                debug!(
                    "Failed to open git history cache: {}, skipping churn calculation",
                    e
                );
                HashMap::new()
            }
        }
    }
}
