use crate::args::{Language, ScanArgs};
use crate::cache::hash::file_content_hash;
use crate::cache::AnalysisCache;
use crate::config::{Config, RuleConfig, RuleSeverity};
use crate::detectors::{self, Severity};
use crate::engine::AnalysisContext;
use crate::framework::detector::FrameworkDetector;
use crate::framework::preset_loader::PresetLoader;
use crate::framework::presets::FrameworkPreset;
use crate::git_cache::GitHistoryCache;
use crate::graph::{DependencyGraph, EdgeData};
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::console::{style, Term};
#[cfg(not(feature = "cli"))]
use crate::no_cli_mocks::indicatif::{ProgressBar, ProgressStyle};
use crate::package_json;
use crate::parser::{ImportParser, ParsedFile, ParserConfig};
use crate::project_root::detect_project_root;
use crate::report::AnalysisReport;
use crate::resolver::PathResolver;
use crate::scanner::FileScanner;
use crate::Result;
#[cfg(feature = "cli")]
use console::{style, Term};
#[cfg(feature = "cli")]
use indicatif::{ProgressBar, ProgressStyle};
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

        // Overrides from args
        if let Some(ref detectors) = args.detectors {
            for id in detectors.split(',').map(|s| s.trim()) {
                config
                    .rules
                    .insert(id.to_string(), RuleConfig::Short(RuleSeverity::Error));
            }
        }
        if let Some(ref exclude) = args.exclude_detectors {
            for id in exclude.split(',').map(|s| s.trim()) {
                config
                    .rules
                    .insert(id.to_string(), RuleConfig::Short(RuleSeverity::Off));
            }
        }

        // Overrides for git
        if args.no_git {
            config.enable_git = false;
        }
        if let Some(ref period) = args.git_history_period {
            config.git.history_period = period.clone();
        }

        if let Some(max_size) = args.max_file_size {
            config.max_file_size = max_size;
        }

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

        // 1. Load presets
        let mut presets = Vec::new();

        // From extends (explicit)
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

        // From explicit framework field (legacy)
        if let Some(ref fw) = self.config.framework {
            if !self.config.extends.contains(fw) {
                if let Ok(p) = PresetLoader::load_any(fw) {
                    presets.push(p);
                }
            }
        }

        // Auto-detect frameworks
        let detected_frameworks = if self.config.auto_detect_framework {
            let detected = FrameworkDetector::detect(&self.project_root);
            if !detected.is_empty() {
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

                // Load presets for detected frameworks if they are not already loaded via extends
                for fw in &detected {
                    let name = match fw {
                        crate::framework::Framework::NestJS => "nestjs",
                        crate::framework::Framework::NextJS => "nextjs",
                        crate::framework::Framework::React => "react",
                        crate::framework::Framework::Oclif => "oclif",
                        _ => continue,
                    };
                    if !self.config.extends.contains(&name.to_string()) {
                        if let Ok(p) = PresetLoader::load_builtin(name) {
                            presets.push(p);
                        }
                    }
                }
            }
            detected
        } else {
            Vec::new()
        };

        let final_config = self.apply_presets(&presets);

        let active_ids = self.get_active_detectors(&final_config, &presets);
        let parser_config = ParserConfig::from_active_detectors(&active_ids);

        let mut cache = self.load_cache()?;
        let parsed_files = self.parse_files(&files, &parser_config, use_progress, &cache)?;
        self.update_cache(&mut cache, &parsed_files)?;

        let (file_symbols, function_complexity, file_metrics) =
            self.extract_parsed_data(parsed_files);
        let runtime_files = self.get_runtime_files(&file_symbols);

        self.log_runtime_info(runtime_files.len(), files.len());

        let graph = self.build_graph(&runtime_files, &file_symbols, use_progress)?;
        let churn_map = self.get_churn_map(&files, use_progress, &mut cache);
        let resolved_file_symbols = self.resolve_symbols(file_symbols, use_progress);

        let pkg_config = package_json::PackageJsonParser::parse(&self.project_root)?;

        let ctx = AnalysisContext {
            project_path: self.project_root.clone(),
            graph: Arc::new(graph),
            file_symbols: Arc::new(resolved_file_symbols),
            function_complexity: Arc::new(function_complexity),
            file_metrics: Arc::new(file_metrics),
            churn_map,
            config: final_config.clone(),
            script_entry_points: pkg_config.entry_points,
            dynamic_load_patterns: pkg_config.dynamic_load_patterns,
            detected_frameworks,
            presets: presets.clone(),
        };

        let all_smells = self.run_detectors(&ctx, use_progress, &presets)?;

        // Filter smells: only keep smells that are NOT in ignored files
        let filtered_smells: Vec<_> = all_smells
            .into_iter()
            .filter(|smell| {
                // Clones are special: we want to see them even if they touch ignored files
                if matches!(
                    smell.smell_type,
                    crate::detectors::SmellType::CodeClone { .. }
                ) {
                    return true;
                }
                // Keep the smell if at least one of the files it's associated with is NOT ignored
                smell.files.is_empty() || smell.files.iter().any(|f| !self.is_file_ignored(f))
            })
            .collect();

        let mut report = AnalysisReport::new(
            filtered_smells,
            Some(Arc::try_unwrap(ctx.graph).unwrap_or_else(|arc| (*arc).clone())),
            Arc::try_unwrap(ctx.file_symbols).unwrap_or_else(|arc| (*arc).clone()),
            Arc::try_unwrap(ctx.file_metrics).unwrap_or_else(|arc| (*arc).clone()),
            Arc::try_unwrap(ctx.function_complexity).unwrap_or_else(|arc| (*arc).clone()),
            ctx.churn_map,
            presets,
        );
        report.set_files_analyzed(files.len());

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

        if let Some(c) = cache {
            debug!("Saving cache...");
            c.save()?;
        }

        Ok(report)
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
            // 1. Merge rules from presets
            for (rule_name, preset_rule) in &preset.rules {
                let user_rule = final_config
                    .rules
                    .entry(rule_name.clone())
                    .or_insert_with(|| preset_rule.clone());

                // If user already has this rule, we might want to merge options if it's not overriding everything
                if let (RuleConfig::Full(preset_full), RuleConfig::Full(user_full)) =
                    (preset_rule, user_rule)
                {
                    Self::merge_options(&mut user_full.options, &preset_full.options);
                }
            }

            // 2. Merge entry points
            for pattern in &preset.entry_points {
                if !final_config.entry_points.contains(pattern) {
                    final_config.entry_points.push(pattern.clone());
                }
            }

            // 3. Merge overrides
            for ov in &preset.overrides {
                final_config.overrides.push(ov.clone());
            }
        }
        final_config
    }

    fn merge_options(user_options: &mut serde_yaml::Value, preset_options: &serde_yaml::Value) {
        if let (Some(user_map), Some(preset_map)) =
            (user_options.as_mapping_mut(), preset_options.as_mapping())
        {
            for (key, preset_val) in preset_map {
                if !user_map.contains_key(key) {
                    user_map.insert(key.clone(), preset_val.clone());
                } else {
                    // If both are sequences (like ignore_methods), merge them
                    let user_val = user_map.get_mut(key).unwrap();
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
            }
        }
    }

    fn get_active_detectors(
        &self,
        config: &Config,
        presets: &[FrameworkPreset],
    ) -> HashSet<String> {
        let registry = detectors::registry::DetectorRegistry::new();
        let (enabled_detectors, _) =
            registry.get_enabled_full(config, presets, self.args.all_detectors);

        let active_detectors = self.filter_detectors(enabled_detectors, |(id, _)| id);
        active_detectors.into_iter().map(|(id, _)| id).collect()
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
        if use_progress {
            let pb = ProgressBar::new(files.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                    )
                    .unwrap()
                    .progress_chars("█▉▊▋▌▍▎▏  "),
            );

            let result = files
                .par_iter()
                .map(|file| {
                    let hash = file_content_hash(file)?;
                    if let Some(ref c) = cache {
                        if let Some(cached) = c.get(file, &hash) {
                            pb.inc(1);
                            return Ok((file.clone(), (*cached).clone()));
                        }
                    }
                    let parsed = parser.parse_file_with_config(file, config)?;
                    pb.inc(1);
                    if let Some(name) = file.file_name() {
                        pb.set_message(name.to_string_lossy().to_string());
                    }
                    Ok((file.clone(), parsed))
                })
                .collect::<Result<HashMap<_, _>>>();
            pb.finish_and_clear();
            result
        } else {
            files
                .par_iter()
                .map(|file| {
                    let hash = file_content_hash(file)?;
                    if let Some(ref c) = cache {
                        if let Some(cached) = c.get(file, &hash) {
                            return Ok((file.clone(), (*cached).clone()));
                        }
                    }
                    debug!("Parsing: {}", file.display());
                    let parsed = parser.parse_file_with_config(file, config)?;
                    Ok((file.clone(), parsed))
                })
                .collect::<Result<HashMap<_, _>>>()
        }
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
    ) {
        let mut symbols = HashMap::new();
        let mut complexity = HashMap::new();
        let mut metrics = HashMap::new();
        for (file, parsed) in parsed_files {
            symbols.insert(file.clone(), parsed.symbols);
            complexity.insert(file.clone(), parsed.functions);
            metrics.insert(
                file,
                crate::engine::context::FileMetrics {
                    lines: parsed.lines,
                },
            );
        }
        (symbols, complexity, metrics)
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

    fn build_graph(
        &self,
        runtime_files: &HashSet<PathBuf>,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        use_progress: bool,
    ) -> Result<DependencyGraph> {
        info!(
            "{}  Building dependency graph...",
            style("🕸️").cyan().bold()
        );
        let resolver = PathResolver::new(&self.project_root, &self.config);
        let mut graph = DependencyGraph::new();
        for file in runtime_files {
            graph.add_file(file);
        }

        let pb = if use_progress {
            let pb = ProgressBar::new(runtime_files.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                    )
                    .unwrap()
                    .progress_chars("█▉▊▋▌▍▎▏  "),
            );
            Some(pb)
        } else {
            None
        };

        let mut resolved_count = 0;
        for file in runtime_files {
            if let Some(ref pb) = pb {
                if let Some(name) = file.file_name() {
                    pb.set_message(name.to_string_lossy().to_string());
                }
            }
            let from_node = graph.get_node(file).unwrap();
            let symbols = file_symbols.get(file).unwrap();

            for import in &symbols.imports {
                if let Some(resolved) = resolver.resolve(import.source.as_str(), file)? {
                    if runtime_files.contains(&resolved) {
                        let to_node = graph.add_file(&resolved);
                        let edge_data = EdgeData::with_all(
                            import.line,
                            import.range,
                            vec![import.name.to_string()],
                        );
                        graph.add_dependency(from_node, to_node, edge_data);
                        resolved_count += 1;
                    }
                }
            }
            if let Some(ref pb) = pb {
                pb.inc(1);
            }
        }

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }

        info!(
            "   {} Nodes: {}, Edges: {}, Resolved: {}",
            style("↳").dim(),
            style(graph.node_count()).yellow(),
            style(graph.edge_count()).yellow(),
            style(resolved_count).dim()
        );
        Ok(graph)
    }

    fn get_churn_map(
        &self,
        files: &[PathBuf],
        use_progress: bool,
        cache: &mut Option<AnalysisCache>,
    ) -> HashMap<PathBuf, usize> {
        info!("{} Calculating metrics...", style("📊").blue().bold());
        if !self.config.enable_git {
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

    fn resolve_symbols(
        &self,
        file_symbols: HashMap<PathBuf, crate::parser::FileSymbols>,
        use_progress: bool,
    ) -> HashMap<PathBuf, crate::parser::FileSymbols> {
        info!("{} Resolving symbols...", style("🔗").cyan().bold());
        let resolver = PathResolver::new(&self.project_root, &self.config);
        let mut resolved_file_symbols = HashMap::new();
        let pb = if use_progress {
            let pb = ProgressBar::new(file_symbols.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                    )
                    .unwrap()
                    .progress_chars("█▉▊▋▌▍▎▏  "),
            );
            Some(pb)
        } else {
            None
        };

        for (file, symbols) in file_symbols {
            if let Some(ref pb) = pb {
                if let Some(name) = file.file_name() {
                    pb.set_message(name.to_string_lossy().to_string());
                }
            }
            let mut resolved_symbols = symbols.clone();
            for import in &mut resolved_symbols.imports {
                if let Some(resolved) = resolver
                    .resolve(import.source.as_str(), &file)
                    .ok()
                    .flatten()
                {
                    import.source = resolved.to_string_lossy().to_string().into();
                }
            }
            for export in &mut resolved_symbols.exports {
                if let Some(ref source) = export.source {
                    if let Some(resolved) = resolver.resolve(source.as_str(), &file).ok().flatten()
                    {
                        export.source = Some(resolved.to_string_lossy().to_string().into());
                    }
                }
            }
            resolved_file_symbols.insert(file, resolved_symbols);
            if let Some(ref pb) = pb {
                pb.inc(1);
            }
        }

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        resolved_file_symbols
    }

    fn run_detectors(
        &self,
        ctx: &AnalysisContext,
        use_progress: bool,
        presets: &[FrameworkPreset],
    ) -> Result<Vec<detectors::ArchSmell>> {
        let registry = detectors::registry::DetectorRegistry::new();
        let (final_detectors, _) =
            registry.get_enabled_full(&ctx.config, presets, self.args.all_detectors);

        let final_detectors = self.filter_detectors(final_detectors, |(id, _)| id);

        let needs_deep = final_detectors.iter().any(|(id, _)| {
            registry
                .get_info(id)
                .map(|info| info.is_deep)
                .unwrap_or(false)
        });

        info!(
            "{} Detecting architectural smells...{}",
            style("🧪").green().bold(),
            if needs_deep {
                style(" (deep analysis enabled)").dim().to_string()
            } else {
                "".to_string()
            }
        );
        let mut all_smells = Vec::new();

        let pb = if use_progress {
            let pb = ProgressBar::new(final_detectors.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:40.green/white}] {pos}/{len} {msg}",
                    )
                    .unwrap()
                    .progress_chars("█▉▊▋▌▍▎▏  "),
            );
            Some(pb)
        } else {
            None
        };

        for (_id, detector) in final_detectors {
            if let Some(ref pb) = pb {
                pb.set_message(detector.name());
            }
            let smells = detector.detect(ctx);

            let status = format!(
                "   {} {:<27} found: {}",
                style("↳").dim(),
                detector.name(),
                if smells.is_empty() {
                    style("0".to_string()).dim()
                } else {
                    style(smells.len().to_string()).red().bold()
                }
            );

            if let Some(ref pb) = pb {
                pb.println(status);
                pb.inc(1);
            } else {
                info!("{}", status);
            }
            all_smells.extend(smells);
        }

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        Ok(all_smells)
    }

    fn parse_detector_id_set(&self, ids: &Option<String>) -> Option<HashSet<String>> {
        ids.as_ref().map(|s| {
            s.split(',')
                .map(|id| id.trim())
                .filter(|id| !id.is_empty())
                .map(|id| id.to_string())
                .collect::<HashSet<_>>()
        })
    }

    fn filter_detectors<T, F: Fn(&T) -> &str>(
        &self,
        detectors: impl IntoIterator<Item = T>,
        id_extractor: F,
    ) -> Vec<T> {
        let include = self.parse_detector_id_set(&self.args.detectors);
        let exclude = self
            .parse_detector_id_set(&self.args.exclude_detectors)
            .unwrap_or_default();

        detectors
            .into_iter()
            .filter(|d| match include.as_ref() {
                Some(set) => set.contains(id_extractor(d)),
                None => true,
            })
            .filter(|d| !exclude.contains(id_extractor(d)))
            .collect()
    }
}
