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

        let symbol_imports = Self::build_symbol_imports_map(&ctx.file_symbols);
        let reexport_map = Self::build_reexport_map(&ctx.file_symbols);

        let dead_files = Self::find_dead_files(ctx, &detector, &symbol_imports, &reexport_map);

        dead_files
            .into_iter()
            .map(ArchSmell::new_dead_code)
            .collect()
    }
}

impl DeadCodeDetector {
    fn build_symbol_imports_map(
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
    ) -> HashMap<(PathBuf, String), HashSet<PathBuf>> {
        let mut symbol_imports: HashMap<(PathBuf, String), HashSet<PathBuf>> = HashMap::new();

        for (importer_path, symbols) in file_symbols {
            for import in &symbols.imports {
                let source_path = PathBuf::from(import.source.as_str());
                if file_symbols.contains_key(&source_path) {
                    symbol_imports
                        .entry((source_path, import.name.to_string()))
                        .or_default()
                        .insert(importer_path.clone());
                }
            }
        }

        symbol_imports
    }

    fn build_reexport_map(
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
    ) -> HashMap<PathBuf, HashSet<PathBuf>> {
        let mut reexport_map: HashMap<PathBuf, HashSet<PathBuf>> = HashMap::new();

        for (importer_path, symbols) in file_symbols {
            for export in &symbols.exports {
                if export.is_reexport {
                    if let Some(ref source) = export.source {
                        let reexported_file = PathBuf::from(source);
                        if file_symbols.contains_key(&reexported_file) {
                            reexport_map
                                .entry(reexported_file)
                                .or_default()
                                .insert(importer_path.clone());
                        }
                    }
                }
            }
        }

        reexport_map
    }

    fn find_dead_files(
        ctx: &AnalysisContext,
        detector: &DeadCodeDetector,
        symbol_imports: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        reexport_map: &HashMap<PathBuf, HashSet<PathBuf>>,
    ) -> Vec<PathBuf> {
        let mut dead_files = Vec::new();

        for node in ctx.graph.nodes() {
            let fan_in = ctx.graph.fan_in(node);

            if let Some(path) = ctx.graph.get_file_path(node) {
                if Self::is_dead_file(path, fan_in, ctx, detector, symbol_imports, reexport_map) {
                    dead_files.push(path.clone());
                }
            }
        }

        dead_files
    }

    fn is_dead_file(
        path: &Path,
        fan_in: usize,
        ctx: &AnalysisContext,
        detector: &DeadCodeDetector,
        symbol_imports: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        reexport_map: &HashMap<PathBuf, HashSet<PathBuf>>,
    ) -> bool {
        fan_in == 0
            && !ctx.is_framework_entry_point(path)
            && !detector.is_entry_point(path)
            && !detector.matches_dynamic_load_pattern(path)
            && !detector.has_used_exports(path, &ctx.file_symbols, symbol_imports)
            && !detector.is_reexported(path, &ctx.file_symbols, reexport_map)
    }

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
        let symbols = match file_symbols.get(path) {
            Some(s) if !s.exports.is_empty() => s,
            _ => return false,
        };

        let path_str = path.to_string_lossy();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        Self::check_named_exports(symbols, path, symbol_imports)
            || Self::check_default_imports(file_symbols, &path_str, file_name)
            || Self::check_reexports(file_symbols, &path_str, file_name)
            || Self::check_local_usages(symbols, file_symbols)
    }

    fn check_named_exports(
        symbols: &crate::parser::FileSymbols,
        path: &Path,
        symbol_imports: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
    ) -> bool {
        let path_buf = path.to_path_buf();
        for export in &symbols.exports {
            if !export.is_reexport && export.name != "default" && export.name != "*" {
                if let Some(importers) =
                    symbol_imports.get(&(path_buf.clone(), export.name.to_string()))
                {
                    if !importers.is_empty() {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn check_default_imports(
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        path_str: &str,
        file_name: &str,
    ) -> bool {
        file_symbols.values().any(|fs| {
            fs.imports.iter().any(|import| {
                (import.name == "default" || import.name == "*")
                    && (import.source == path_str || import.source.ends_with(file_name))
            })
        })
    }

    fn check_reexports(
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        path_str: &str,
        file_name: &str,
    ) -> bool {
        file_symbols.values().any(|reexporter_symbols| {
            reexporter_symbols.exports.iter().any(|export| {
                export.is_reexport
                    && export
                        .source
                        .as_ref()
                        .map(|source| source == path_str || source.ends_with(file_name))
                        .unwrap_or(false)
            })
        })
    }

    fn check_local_usages(
        symbols: &crate::parser::FileSymbols,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
    ) -> bool {
        for export in &symbols.exports {
            if !export.is_reexport
                && export.name != "default"
                && export.name != "*"
                && file_symbols
                    .values()
                    .any(|fs| fs.local_usages.contains(&export.name))
            {
                return true;
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

        Self::check_reexport_map(self, path, &path_buf, file_symbols, reexport_map)
            || Self::check_direct_reexports(self, file_symbols, &path_str, file_name)
    }

    fn check_reexport_map(
        detector: &DeadCodeDetector,
        _path: &Path,
        path_buf: &PathBuf,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        reexport_map: &HashMap<PathBuf, HashSet<PathBuf>>,
    ) -> bool {
        if let Some(reexporters) = reexport_map.get(path_buf) {
            for reexporter in reexporters {
                if detector.is_entry_point(reexporter) {
                    return true;
                }

                if Self::is_reexporter_imported(file_symbols, reexporter) {
                    return true;
                }
            }
        }
        false
    }

    fn is_reexporter_imported(
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        reexporter: &Path,
    ) -> bool {
        let reexporter_str = reexporter.to_string_lossy();
        let reexporter_file_name = reexporter
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        file_symbols.values().any(|fs| {
            fs.imports.iter().any(|import| {
                import.source == reexporter_str.as_ref()
                    || import.source.ends_with(reexporter_file_name)
            })
        })
    }

    fn check_direct_reexports(
        detector: &DeadCodeDetector,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        path_str: &str,
        file_name: &str,
    ) -> bool {
        file_symbols
            .iter()
            .filter(|(reexporter_path, _)| {
                Self::is_reexporter_used(detector, file_symbols, reexporter_path)
            })
            .any(|(_, symbols)| Self::has_reexport_to_path(symbols, path_str, file_name))
    }

    fn has_reexport_to_path(
        symbols: &crate::parser::FileSymbols,
        path_str: &str,
        file_name: &str,
    ) -> bool {
        symbols.exports.iter().any(|export| {
            export.is_reexport
                && export
                    .source
                    .as_ref()
                    .map(|source| source == path_str || source.ends_with(file_name))
                    .unwrap_or(false)
        })
    }

    fn is_reexporter_used(
        detector: &DeadCodeDetector,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        reexporter_path: &Path,
    ) -> bool {
        detector.is_entry_point(reexporter_path)
            || Self::is_reexporter_imported(file_symbols, reexporter_path)
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
