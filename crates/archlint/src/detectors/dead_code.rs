use crate::config::Config;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub fn init() {}

pub struct DeadCodeDetector {
    entry_patterns: Vec<String>,
    explicit_entry_points: HashSet<PathBuf>,
    dynamic_load_patterns: Vec<String>,
}

pub struct DeadCodeDetectorFactory;

impl DetectorFactory for DeadCodeDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "dead_code",
            name: "Dead Code Detector",
            description: "Detects unused files and modules",
            default_enabled: true,
            is_deep: false,
        }
    }

    fn create(&self, config: &Config) -> Box<dyn Detector> {
        Box::new(DeadCodeDetector::new_default(config))
    }
}

inventory::submit! {
    &DeadCodeDetectorFactory as &dyn DetectorFactory
}

impl Detector for DeadCodeDetector {
    fn name(&self) -> &'static str {
        "DeadCode"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let detector = DeadCodeDetector::new(
            &ctx.config,
            ctx.script_entry_points.clone(),
            ctx.dynamic_load_patterns.clone(),
        );

        let mut dead_files = Vec::new();

        // Build a map of which files import which symbols from which files
        let mut symbol_imports: HashMap<(PathBuf, String), HashSet<PathBuf>> = HashMap::new();
        // Build a map of files that re-export from other files
        let mut reexport_map: HashMap<PathBuf, HashSet<PathBuf>> = HashMap::new();

        for (importer_path, symbols) in &ctx.file_symbols {
            for import in &symbols.imports {
                // Symbols are already resolved to absolute paths in AnalysisEngine
                let source_path = PathBuf::from(import.source.as_str());
                if ctx.file_symbols.contains_key(&source_path) {
                    symbol_imports
                        .entry((source_path, import.name.to_string()))
                        .or_default()
                        .insert(importer_path.clone());
                }
            }

            // Check for re-exports
            for export in &symbols.exports {
                if export.is_reexport {
                    if let Some(ref source) = export.source {
                        let reexported_file = PathBuf::from(source);
                        if ctx.file_symbols.contains_key(&reexported_file) {
                            reexport_map
                                .entry(reexported_file)
                                .or_default()
                                .insert(importer_path.clone());
                        }
                    }
                }
            }
        }

        for node in ctx.graph.nodes() {
            let fan_in = ctx.graph.fan_in(node);

            if let Some(path) = ctx.graph.get_file_path(node) {
                if fan_in == 0
                    && !ctx.is_framework_entry_point(path)
                    && !detector.is_entry_point(path)
                    && !detector.matches_dynamic_load_pattern(path)
                    && !detector.has_used_exports(path, &ctx.file_symbols, &symbol_imports)
                    && !detector.is_reexported(path, &ctx.file_symbols, &reexport_map)
                {
                    dead_files.push(path.clone());
                }
            }
        }

        dead_files
            .into_iter()
            .map(ArchSmell::new_dead_code)
            .collect()
    }
}

impl DeadCodeDetector {
    pub fn new_default(config: &Config) -> Self {
        Self::new(config, HashSet::new(), Vec::new())
    }

    pub fn new(
        config: &Config,
        explicit_entry_points: HashSet<PathBuf>,
        dynamic_load_patterns: Vec<String>,
    ) -> Self {
        let mut patterns = vec![
            "main.ts".to_string(),
            "main.js".to_string(),
            "index.ts".to_string(),
            "index.js".to_string(),
            "app.ts".to_string(),
            "app.js".to_string(),
            "*.module.ts".to_string(),
            "*.module.js".to_string(),
            "*.controller.ts".to_string(),
            "*.controller.js".to_string(),
            "*.decorator.ts".to_string(),
            "*.decorator.js".to_string(),
            "*.dto.ts".to_string(),
            "*.dto.js".to_string(),
            "*.event.ts".to_string(),
            "*.event.js".to_string(),
            "*.entity.ts".to_string(),
            "*.entity.js".to_string(),
            "*.fixture.ts".to_string(),
            "*.fixture.js".to_string(),
            "*.test.ts".to_string(),
            "*.test.js".to_string(),
            "*.spec.ts".to_string(),
            "*.spec.js".to_string(),
            "*.e2e-spec.ts".to_string(),
            "*.e2e-spec.js".to_string(),
            "*.config.ts".to_string(),
            "*.config.js".to_string(),
            "*.setup.ts".to_string(),
            "*.setup.js".to_string(),
            "**/test/**".to_string(),
            "**/tests/**".to_string(),
            "**/__fixtures__/**".to_string(),
            "**/*.mock.ts".to_string(),
            "**/*.mock.js".to_string(),
        ];

        patterns.extend(config.entry_points.clone());

        Self {
            entry_patterns: patterns,
            explicit_entry_points,
            dynamic_load_patterns,
        }
    }

    fn has_used_exports(
        &self,
        path: &Path,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        symbol_imports: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
    ) -> bool {
        if let Some(symbols) = file_symbols.get(path) {
            if !symbols.exports.is_empty() {
                let path_buf = path.to_path_buf();
                let path_str = path.to_string_lossy();
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                for export in &symbols.exports {
                    if export.is_reexport {
                        continue;
                    }

                    if export.name != "default" && export.name != "*" {
                        if let Some(importers) =
                            symbol_imports.get(&(path_buf.clone(), export.name.to_string()))
                        {
                            if !importers.is_empty() {
                                return true;
                            }
                        }
                    }
                }

                for fs in file_symbols.values() {
                    for import in &fs.imports {
                        if (import.name == "default" || import.name == "*")
                            && (import.source == path_str.as_ref()
                                || import.source.ends_with(file_name))
                        {
                            return true;
                        }
                    }
                }

                for reexporter_symbols in file_symbols.values() {
                    for export in &reexporter_symbols.exports {
                        if export.is_reexport {
                            if let Some(ref source) = export.source {
                                if source == path_str.as_ref() || source.ends_with(file_name) {
                                    return true;
                                }
                            }
                        }
                    }
                }

                for export in &symbols.exports {
                    if !export.is_reexport && export.name != "default" && export.name != "*" {
                        for fs in file_symbols.values() {
                            if fs.local_usages.contains(&export.name) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn is_reexported(
        &self,
        path: &Path,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        reexport_map: &HashMap<PathBuf, HashSet<PathBuf>>,
    ) -> bool {
        let path_buf = path.to_path_buf();
        let path_str = path.to_string_lossy();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if let Some(reexporters) = reexport_map.get(&path_buf) {
            for reexporter in reexporters {
                if self.is_entry_point(reexporter) {
                    return true;
                }

                for fs in file_symbols.values() {
                    for import in &fs.imports {
                        let import_path_str = import.source.as_str();
                        if import_path_str == reexporter.to_string_lossy().as_ref()
                            || import_path_str.ends_with(
                                reexporter
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or(""),
                            )
                        {
                            return true;
                        }
                    }
                }
            }
        }

        for (reexporter_path, symbols) in file_symbols {
            let reexporter_used = self.is_entry_point(reexporter_path)
                || file_symbols.iter().any(|(_, fs)| {
                    fs.imports.iter().any(|i| {
                        i.source == reexporter_path.to_string_lossy().as_ref()
                            || i.source.ends_with(
                                reexporter_path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or(""),
                            )
                    })
                });

            if !reexporter_used {
                continue;
            }

            for export in &symbols.exports {
                if export.is_reexport {
                    if let Some(ref source) = export.source {
                        if source == path_str.as_ref() || source.ends_with(file_name) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn matches_dynamic_load_pattern(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        for pattern in &self.dynamic_load_patterns {
            if self.matches_glob_pattern(&path_str, pattern) {
                return true;
            }
        }
        false
    }

    fn matches_glob_pattern(&self, path: &str, pattern: &str) -> bool {
        let pattern_parts: Vec<&str> = pattern
            .split('*')
            .filter(|p| !p.is_empty() && p != &"/")
            .collect();

        if pattern_parts.is_empty() {
            return false;
        }

        pattern_parts.iter().all(|part| path.contains(part))
    }

    pub fn is_entry_point(&self, path: &Path) -> bool {
        if self.explicit_entry_points.contains(path) {
            return true;
        }

        let path_str = path.to_string_lossy();
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        for pattern in &self.entry_patterns {
            if let Some(suffix) = pattern.strip_prefix('*') {
                if file_name.ends_with(suffix) {
                    return true;
                }
            } else if pattern.contains('*') {
                let parts: Vec<&str> = pattern.split('*').collect();
                if parts.iter().all(|part| path_str.contains(part)) {
                    return true;
                }
            } else if file_name == pattern || path_str.ends_with(pattern) {
                return true;
            }
        }

        false
    }
}
