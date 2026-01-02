use crate::cache::hash::file_content_hash;
use crate::cache::AnalysisCache;
use crate::cli::{Language, ScanArgs};
use crate::config::Config;
use crate::detectors::{self, Severity};
use crate::engine::AnalysisContext;
use crate::framework::classifier::FileClassifier;
use crate::framework::detector::FrameworkDetector;
use crate::framework::presets;
use crate::graph::{DependencyGraph, EdgeData};
use crate::metrics::GitChurn;
use crate::package_json;
use crate::parser::{ImportParser, ParsedFile, ParserConfig};
use crate::project_root::detect_project_root;
use crate::report::AnalysisReport;
use crate::resolver::PathResolver;
use crate::scanner::FileScanner;
use crate::Result;
use console::{style, Term};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub struct AnalysisEngine {
    pub args: ScanArgs,
    pub config: Config,
    pub project_root: PathBuf,
    pub target_path: PathBuf,
}

use std::str::FromStr;

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
            config.detectors.enabled =
                Some(detectors.split(',').map(|s| s.trim().to_string()).collect());
        }
        if let Some(ref exclude) = args.exclude_detectors {
            config
                .detectors
                .disabled
                .extend(exclude.split(',').map(|s| s.trim().to_string()));
        }

        // Overrides for severity
        if let Some(ref min_sev) = args.min_severity {
            config.severity.minimum = Some(
                Severity::from_str(min_sev).map_err(crate::error::AnalysisError::InvalidConfig)?,
            );
        }
        if let Some(min_score) = args.min_score {
            config.severity.minimum_score = Some(min_score);
        }

        // Overrides for git
        if args.no_git {
            config.enable_git = false;
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

        let extensions = match self.args.lang {
            Language::TypeScript => vec!["ts".to_string(), "tsx".to_string()],
            Language::JavaScript => vec!["js".to_string(), "jsx".to_string()],
        };

        let files = if let Some(ref explicit_files) = self.args.files {
            // Use explicit files (from glob expansion) and apply ignore patterns
            explicit_files
                .iter()
                .filter(|f| {
                    let is_excluded = if self.config.ignore.is_empty() {
                        false
                    } else {
                        let rel_path = f
                            .strip_prefix(&self.project_root)
                            .unwrap_or(f)
                            .to_string_lossy();
                        self.config.ignore.iter().any(|p| {
                            if let Ok(pattern) = glob::Pattern::new(p) {
                                pattern.matches(&rel_path)
                            } else {
                                false
                            }
                        })
                    };
                    !is_excluded
                })
                .cloned()
                .collect()
        } else {
            let scanner = FileScanner::new(&self.project_root, &self.target_path, extensions);
            scanner.scan(&self.config)?
        };
        info!(
            "{} Found {} files to analyze",
            style("📁").blue().bold(),
            style(files.len()).yellow()
        );

        let detected_frameworks = if self.config.auto_detect_framework {
            FrameworkDetector::detect(&self.project_root)
        } else {
            Vec::new()
        };

        if !detected_frameworks.is_empty() {
            info!(
                "{}  Detected frameworks: {}",
                style("🛠️").magenta().bold(),
                style(
                    detected_frameworks
                        .iter()
                        .map(|f| format!("{:?}", f))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
                .yellow()
            );
        }

        let file_types: HashMap<PathBuf, crate::framework::FileType> = files
            .iter()
            .map(|f| (f.clone(), FileClassifier::classify(f, &detected_frameworks)))
            .collect();

        let presets = presets::get_presets(&detected_frameworks);
        let mut final_config = self.config.clone();
        for preset in &presets {
            for ignore in &preset.vendor_ignore {
                if !final_config
                    .thresholds
                    .vendor_coupling
                    .ignore_packages
                    .contains(ignore)
                {
                    final_config
                        .thresholds
                        .vendor_coupling
                        .ignore_packages
                        .push(ignore.clone());
                }
                if !final_config
                    .thresholds
                    .hub_dependency
                    .ignore_packages
                    .contains(ignore)
                {
                    final_config
                        .thresholds
                        .hub_dependency
                        .ignore_packages
                        .push(ignore.clone());
                }
            }
        }

        let parser = ImportParser::new()?;
        let resolver = PathResolver::new(&self.project_root, &self.config);

        let registry = detectors::registry::DetectorRegistry::new();
        let selection =
            crate::framework::selector::DetectorSelector::select(&final_config.detectors, &presets);
        let active_ids: HashSet<String> = if self.args.all_detectors {
            registry
                .list_all()
                .into_iter()
                .map(|i| i.id.to_string())
                .collect()
        } else if let Some(ref user_enabled) = selection.user_enabled {
            user_enabled.clone()
        } else {
            registry
                .list_all()
                .into_iter()
                .filter(|info| {
                    (info.default_enabled || selection.preset_enabled.contains(info.id))
                        && !selection.disabled.contains(info.id)
                })
                .map(|info| info.id.to_string())
                .collect()
        };

        let parser_config = ParserConfig {
            collect_complexity: active_ids.iter().any(|id| {
                matches!(
                    id.as_str(),
                    "complexity"
                        | "deep_nesting"
                        | "long_params"
                        | "hub_module"
                        | "god_module"
                        | "hub_dependency"
                )
            }),
            collect_primitive_params: active_ids.contains("primitive_obsession"),
            collect_classes: active_ids.contains("lcom"),
            collect_env_vars: active_ids.contains("scattered_config"),
            collect_used_symbols: active_ids.contains("scattered_module"),
        };

        let mut cache = if !self.args.no_cache {
            debug!("Loading cache...");
            Some(AnalysisCache::load(&self.project_root, &self.config)?)
        } else {
            None
        };

        let parsed_files: HashMap<PathBuf, ParsedFile> = if use_progress {
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
                    let parsed = parser.parse_file_with_config(file, &parser_config)?;
                    pb.inc(1);
                    if let Some(name) = file.file_name() {
                        pb.set_message(style(name.to_string_lossy().to_string()).dim().to_string());
                    }
                    Ok((file.clone(), parsed))
                })
                .collect::<Result<HashMap<_, _>>>();
            pb.finish_and_clear();
            result?
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
                    let parsed = parser.parse_file_with_config(file, &parser_config)?;
                    Ok((file.clone(), parsed))
                })
                .collect::<Result<HashMap<_, _>>>()?
        };

        if let Some(ref mut c) = cache {
            for (file, parsed) in &parsed_files {
                let hash = file_content_hash(file)?;
                c.insert(file.clone(), hash, (*parsed).clone());
            }
        }

        let mut file_symbols = HashMap::new();
        let mut function_complexity = HashMap::new();
        let mut file_metrics = HashMap::new();
        for (file, parsed) in parsed_files {
            file_symbols.insert(file.clone(), parsed.symbols);
            function_complexity.insert(file.clone(), parsed.functions);
            file_metrics.insert(
                file,
                crate::engine::context::FileMetrics {
                    lines: parsed.lines,
                },
            );
        }

        let runtime_files: HashSet<PathBuf> = file_symbols
            .iter()
            .filter(|(_, s)| s.has_runtime_code)
            .map(|(p, _)| p.clone())
            .collect();

        info!(
            "{} Runtime code found in {} files (skipped {} type-only)",
            style("💎").magenta().bold(),
            style(runtime_files.len()).cyan(),
            style(files.len() - runtime_files.len()).dim()
        );

        info!(
            "{}  Building dependency graph...",
            style("🕸️").cyan().bold()
        );
        let mut graph = DependencyGraph::new();
        for file in &runtime_files {
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
        for file in &runtime_files {
            if let Some(ref pb) = pb {
                if let Some(name) = file.file_name() {
                    pb.set_message(style(name.to_string_lossy().to_string()).dim().to_string());
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

        info!("{} Calculating metrics...", style("📊").blue().bold());
        let churn_map = if !self.config.enable_git {
            debug!("Git integration disabled, skipping churn calculation");
            HashMap::new()
        } else if let Some(cached_churn) = cache.as_ref().and_then(|c| c.get_churn_map()) {
            debug!("Using cached churn map");
            cached_churn.clone()
        } else {
            let git_churn = GitChurn::new(&self.project_root);
            if !git_churn.is_available() {
                debug!("Git repository not found, skipping churn calculation");
                HashMap::new()
            } else {
                match git_churn.calculate_churn(&files, use_progress) {
                    Ok(map) => {
                        if let Some(ref mut c) = cache {
                            c.insert_churn_map(map.clone());
                        }
                        map
                    }
                    Err(e) => {
                        debug!("Git churn calculation failed: {}, skipping", e);
                        HashMap::new()
                    }
                }
            }
        };

        info!("{} Resolving symbols...", style("🔗").cyan().bold());
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

        for (file, symbols) in &file_symbols {
            if let Some(ref pb) = pb {
                if let Some(name) = file.file_name() {
                    pb.set_message(style(name.to_string_lossy().to_string()).dim().to_string());
                }
            }
            let mut resolved_symbols = symbols.clone();
            for import in &mut resolved_symbols.imports {
                if let Some(resolved) = resolver
                    .resolve(import.source.as_str(), file)
                    .ok()
                    .flatten()
                {
                    import.source = resolved.to_string_lossy().to_string().into();
                }
            }
            for export in &mut resolved_symbols.exports {
                if let Some(ref source) = export.source {
                    if let Some(resolved) = resolver.resolve(source.as_str(), file).ok().flatten() {
                        export.source = Some(resolved.to_string_lossy().to_string().into());
                    }
                }
            }
            resolved_file_symbols.insert(file.clone(), resolved_symbols);
            if let Some(ref pb) = pb {
                pb.inc(1);
            }
        }

        if let Some(pb) = pb {
            pb.finish_and_clear();
        }

        info!(
            "{}  Analyzing configuration and scripts...",
            style("⚙️").dim().bold()
        );
        let pkg_config = package_json::PackageJsonParser::parse(&self.project_root)?;

        let ctx = AnalysisContext {
            project_path: self.project_root.clone(),
            graph,
            file_symbols: resolved_file_symbols,
            function_complexity,
            file_metrics,
            churn_map,
            config: final_config.clone(),
            script_entry_points: pkg_config.entry_points,
            dynamic_load_patterns: pkg_config.dynamic_load_patterns,
            detected_frameworks,
            file_types,
        };

        let (final_detectors, needs_deep) =
            registry.get_enabled_with_presets(&final_config, &presets, self.args.all_detectors);

        let is_deep = needs_deep;

        info!(
            "{} Detecting architectural smells...{}",
            style("🧪").green().bold(),
            if is_deep {
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

        for detector in final_detectors {
            if let Some(ref pb) = pb {
                pb.set_message(style(detector.name()).dim().to_string());
            }
            let smells = detector.detect(&ctx);

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

        let mut report = AnalysisReport::new(
            all_smells,
            Some(ctx.graph),
            ctx.file_symbols,
            ctx.file_metrics,
            ctx.function_complexity,
            ctx.churn_map,
        );
        report.set_files_analyzed(files.len());
        report.apply_severity_config(&self.config.severity);

        if let Some(c) = cache {
            debug!("Saving cache...");
            c.save()?;
        }

        Ok(report)
    }
}
