use crate::config::Config;
use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

#[detector(SmellType::DeadCode)]
pub struct DeadCodeDetector {
    entry_patterns: Vec<String>,
    explicit_entry_points: HashSet<PathBuf>,
    dynamic_load_patterns: Vec<String>,
    exclude: Vec<String>,
    compiled_exclude: Vec<glob::Pattern>,
    project_root: PathBuf,
}

impl Detector for DeadCodeDetector {
    crate::impl_detector_report!(
        explain: _smell => (
            problem: "Unused file detected",
            reason: "This file is not imported by any other module in the codebase. It may be leftover code from refactoring, experimental code that was never integrated, or a genuinely unused module.",
            risks: [
                "Increases codebase size and maintenance burden",
                "Causes confusion about what code is actually in use",
                "May contain outdated patterns or security vulnerabilities",
                "Wastes developer time when searching or refactoring",
                "Can lead to accidental usage of outdated code"
            ],
            recommendations: [
                "Verify the file is truly unused (check dynamic imports, tests, configs)",
                "Remove the file if confirmed as dead code",
                "If keeping for reference, move to an archive or documentation",
                "Add the file to entry_points config if it's an intentional entry point",
                "Review recent refactorings to understand why it became unused"
            ]
        ),
        table: {
            title: "Dead Code",
            columns: ["File", "Directory", "pts"],
            row: DeadCode { } (smell, location, pts) => [
                location,
                smell.files.first()
                    .and_then(|file_path| file_path.parent()).map_or_else(|| ".".into(), |p| p.to_string_lossy().to_string()),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let rule = match ctx.get_rule("dead_code") {
            Some(r) => r,
            None => return Vec::new(),
        };

        // Combine rule-specific exclude and global config ignore.
        // If the detector was manually constructed with its own exclude list, prefer that.
        let mut combined_exclude = if self.exclude.is_empty() {
            rule.exclude.clone()
        } else {
            self.exclude.clone()
        };
        combined_exclude.extend(ctx.config.ignore.clone());

        let project_root = if self.project_root.as_os_str().is_empty() {
            ctx.project_path.clone()
        } else {
            self.project_root.clone()
        };

        let detector = DeadCodeDetector::new(
            &ctx.config,
            ctx.script_entry_points.clone(),
            ctx.dynamic_load_patterns.clone(),
            combined_exclude,
            project_root,
        );

        let symbol_imports = detector.build_symbol_imports_map(ctx.file_symbols.as_ref());
        let reexport_map = detector.build_reexport_map(ctx.file_symbols.as_ref());

        let dead_files = detector.find_dead_files(ctx, &symbol_imports, &reexport_map);

        dead_files
            .into_iter()
            .filter_map(|path| {
                let file_rule = ctx.get_rule_for_file("dead_code", &path)?;
                let mut smell = ArchSmell::new_dead_code(path);
                smell.severity = file_rule.severity;
                Some(smell)
            })
            .collect()
    }
}

impl DeadCodeDetector {
    fn build_symbol_imports_map(
        &self,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
    ) -> HashMap<(PathBuf, String), HashSet<PathBuf>> {
        let mut symbol_imports: HashMap<(PathBuf, String), HashSet<PathBuf>> = HashMap::new();

        for (importer_path, symbols) in file_symbols {
            if self.is_path_excluded(importer_path) {
                continue;
            }
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
        &self,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
    ) -> HashMap<PathBuf, HashSet<PathBuf>> {
        let mut reexport_map: HashMap<PathBuf, HashSet<PathBuf>> = HashMap::new();

        for (importer_path, symbols) in file_symbols {
            if self.is_path_excluded(importer_path) {
                continue;
            }
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
        &self,
        ctx: &AnalysisContext,
        symbol_imports: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        reexport_map: &HashMap<PathBuf, HashSet<PathBuf>>,
    ) -> Vec<PathBuf> {
        let mut dead_files = Vec::new();

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                if self.is_path_excluded(path) {
                    continue;
                }
                if self.is_dead_file(
                    path,
                    ctx.file_symbols.as_ref(),
                    symbol_imports,
                    reexport_map,
                ) {
                    dead_files.push(path.clone());
                }
            }
        }

        dead_files
    }

    fn is_dead_file(
        &self,
        path: &Path,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        symbol_imports: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        reexport_map: &HashMap<PathBuf, HashSet<PathBuf>>,
    ) -> bool {
        // We ignore fan_in from the graph because we've built our own symbol_imports map
        // that respects the detector-specific exclude patterns.
        !self.is_entry_point(path)
            && !self.matches_dynamic_load_pattern(path)
            && !self.has_used_exports(path, file_symbols, symbol_imports)
            && !self.is_reexported(path, file_symbols, reexport_map)
    }

    #[must_use]
    pub fn new_default(config: &Config) -> Self {
        Self::new(
            config,
            HashSet::new(),
            Vec::new(),
            Vec::new(),
            PathBuf::new(),
        )
    }

    #[must_use]
    pub fn new(
        config: &Config,
        explicit_entry_points: HashSet<PathBuf>,
        dynamic_load_patterns: Vec<String>,
        exclude: Vec<String>,
        project_root: PathBuf,
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

        let mut compiled_exclude = Vec::with_capacity(exclude.len());
        for pattern_str in &exclude {
            match glob::Pattern::new(pattern_str) {
                Ok(p) => compiled_exclude.push(p),
                Err(e) => log::warn!("Invalid exclude pattern '{}': {}", pattern_str, e),
            }
        }

        Self {
            entry_patterns: patterns,
            explicit_entry_points,
            dynamic_load_patterns,
            exclude,
            compiled_exclude,
            project_root,
        }
    }

    fn is_path_excluded(&self, path: &Path) -> bool {
        if self.compiled_exclude.is_empty() {
            return false;
        }

        let relative_path = path
            .strip_prefix(&self.project_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        for pattern in &self.compiled_exclude {
            if pattern.matches(&relative_path) {
                return true;
            }
        }

        false
    }

    fn has_used_exports(
        &self,
        path: &Path,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        symbol_imports: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
    ) -> bool {
        let symbols = match file_symbols.get(path) {
            Some(s) => s,
            _ => return false,
        };

        let path_str = path.to_string_lossy();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        Self::check_named_exports(symbols, path, symbol_imports)
            || self.check_default_imports(file_symbols, &path_str, file_name)
            || self.check_reexports(file_symbols, &path_str, file_name)
            || self.check_local_usages(symbols, file_symbols)
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
        &self,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        path_str: &str,
        file_name: &str,
    ) -> bool {
        file_symbols.iter().any(|(importer_path, fs)| {
            if self.is_path_excluded(importer_path) {
                return false;
            }
            fs.imports.iter().any(|import| {
                (import.name == "default" || import.name == "*")
                    && Self::matches_source(&import.source, path_str, file_name)
            })
        })
    }

    fn check_reexports(
        &self,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        path_str: &str,
        file_name: &str,
    ) -> bool {
        file_symbols
            .iter()
            .any(|(importer_path, reexporter_symbols)| {
                if self.is_path_excluded(importer_path) {
                    return false;
                }
                reexporter_symbols.exports.iter().any(|export| {
                    export.is_reexport
                        && export
                            .source
                            .as_ref()
                            .map(|source| Self::matches_source(source, path_str, file_name))
                            .unwrap_or(false)
                })
            })
    }

    fn matches_source(source: &str, path_str: &str, file_name: &str) -> bool {
        if source == path_str || source.ends_with(file_name) {
            return true;
        }

        // Handle extension-less imports (e.g. import './foo' for foo.ts)
        if let Some(dot_pos) = file_name.rfind('.') {
            let base = &file_name[..dot_pos];
            if source == base
                || source.ends_with(&format!("/{}", base))
                || source.ends_with(&format!("\\{}", base))
            {
                return true;
            }
        }

        false
    }

    fn check_local_usages(
        &self,
        symbols: &crate::parser::FileSymbols,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
    ) -> bool {
        for export in &symbols.exports {
            if !export.is_reexport
                && export.name != "default"
                && export.name != "*"
                && file_symbols.iter().any(|(importer_path, fs)| {
                    if self.is_path_excluded(importer_path) {
                        return false;
                    }
                    fs.local_usages.contains(&export.name)
                })
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

        self.check_reexport_map(&path_buf, file_symbols, reexport_map)
            || self.check_direct_reexports(file_symbols, &path_str, file_name)
    }

    fn check_reexport_map(
        &self,
        path_buf: &PathBuf,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        reexport_map: &HashMap<PathBuf, HashSet<PathBuf>>,
    ) -> bool {
        if let Some(reexporters) = reexport_map.get(path_buf) {
            for reexporter in reexporters {
                if self.is_entry_point(reexporter) {
                    return true;
                }

                if self.is_reexporter_imported(file_symbols, reexporter) {
                    return true;
                }
            }
        }
        false
    }

    fn is_reexporter_imported(
        &self,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        reexporter: &Path,
    ) -> bool {
        let reexporter_str = reexporter.to_string_lossy();
        let reexporter_file_name = reexporter
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        file_symbols.iter().any(|(importer_path, fs)| {
            if self.is_path_excluded(importer_path) {
                return false;
            }
            fs.imports.iter().any(|import| {
                Self::matches_source(&import.source, &reexporter_str, reexporter_file_name)
            })
        })
    }

    fn check_direct_reexports(
        &self,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        path_str: &str,
        file_name: &str,
    ) -> bool {
        file_symbols
            .iter()
            .filter(|(reexporter_path, _)| self.is_reexporter_used(file_symbols, reexporter_path))
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
                    .is_some_and(|source| source == path_str || source.ends_with(file_name))
        })
    }

    fn is_reexporter_used(
        &self,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        reexporter_path: &Path,
    ) -> bool {
        !self.is_path_excluded(reexporter_path)
            && (self.is_entry_point(reexporter_path)
                || self.is_reexporter_imported(file_symbols, reexporter_path))
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

    #[must_use]
    pub fn is_entry_point(&self, path: &Path) -> bool {
        if self.is_path_excluded(path) {
            return false;
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_path_excluded_windows_normalization() {
        let detector = DeadCodeDetector::new(
            &Config::default(),
            HashSet::new(),
            Vec::new(),
            vec!["src/ignored/*.ts".to_string()],
            PathBuf::from("/project"),
        );

        // Simulate a Windows-style path. On Unix, this is just a path with backslashes in name.
        // But our logic should normalize it to forward slashes.
        let path = PathBuf::from("/project/src\\ignored\\file.ts");

        assert!(
            detector.is_path_excluded(&path),
            "Should match normalized path"
        );
    }

    #[test]
    fn test_is_path_excluded_basic() {
        let detector = DeadCodeDetector::new(
            &Config::default(),
            HashSet::new(),
            Vec::new(),
            vec!["src/ignored/*.ts".to_string()],
            PathBuf::from("/project"),
        );

        let path = PathBuf::from("/project/src/ignored/file.ts");
        assert!(detector.is_path_excluded(&path));

        let path2 = PathBuf::from("/project/src/used/file.ts");
        assert!(!detector.is_path_excluded(&path2));
    }
}
