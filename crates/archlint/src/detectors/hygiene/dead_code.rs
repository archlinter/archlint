use crate::config::{types::TEST_FILE_PATTERNS, Config};
use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

#[detector(SmellType::DeadCode)]
pub struct DeadCodeDetector {
    compiled_entry_patterns: Vec<glob::Pattern>,
    explicit_entry_points: HashSet<PathBuf>,
    dynamic_load_patterns: Vec<String>,
    exclude: Vec<String>,
    compiled_exclude: Vec<glob::Pattern>,
    project_root: PathBuf,
}

/// Maps import/re-export specifiers to the files that use them.
///
/// Specifiers are resolved to absolute paths earlier in the pipeline, so the common
/// case is an exact path key. Specifiers that failed to resolve — unconfigured aliases,
/// virtual modules — keep their raw form and are indexed by their trailing segment
/// instead. That fallback is deliberately lenient: without it, a project whose aliases
/// archlint cannot resolve would light up with false dead-code reports. It applies
/// *only* to unresolved specifiers, so two files that merely share a basename no longer
/// keep each other alive.
#[derive(Default)]
struct SourceIndex {
    by_key: HashMap<String, HashSet<PathBuf>>,
}

impl SourceIndex {
    fn insert(&mut self, source: &str, user: &Path) {
        self.by_key
            .entry(Self::key_for_source(source))
            .or_default()
            .insert(user.to_path_buf());
    }

    fn key_for_source(source: &str) -> String {
        let normalised = source.replace('\\', "/");
        if Path::new(&normalised).is_absolute() {
            return normalised;
        }
        normalised
            .rsplit('/')
            .next()
            .unwrap_or(&normalised)
            .to_string()
    }

    /// Keys a target file can be referenced by: its full path (resolved specifiers),
    /// its name, and its name without extension (extension-less specifiers).
    fn lookup_keys(target: &Path) -> Vec<String> {
        let mut keys = Vec::with_capacity(3);
        keys.push(target.to_string_lossy().replace('\\', "/"));

        if let Some(file_name) = target.file_name().and_then(|name| name.to_str()) {
            keys.push(file_name.to_string());
            if let Some((base, _)) = file_name.rsplit_once('.') {
                keys.push(base.to_string());
            }
        }

        keys
    }

    fn users(&self, target: &Path) -> impl Iterator<Item = &PathBuf> {
        Self::lookup_keys(target)
            .into_iter()
            .filter_map(|key| self.by_key.get(&key))
            .flatten()
    }

    fn is_used(&self, target: &Path) -> bool {
        self.users(target).next().is_some()
    }
}

/// Project-wide usage facts, gathered in a single pass.
///
/// Every lookup here used to be a scan over all files, performed once per candidate
/// file — which made whole-project dead-code analysis grow cubically.
#[derive(Default)]
struct UsageIndex {
    /// Files imported with a `default` or namespace (`*`) binding.
    default_imports: SourceIndex,
    /// Files pulled in by an `export … from` somewhere.
    reexports: SourceIndex,
    /// Every identifier referenced from a non-excluded file.
    local_usages: HashSet<String>,
}

/// Compiles glob patterns, skipping (and reporting) the ones that do not parse.
fn compile_patterns(patterns: &[String], kind: &str) -> Vec<glob::Pattern> {
    patterns
        .iter()
        .filter_map(|pattern| match glob::Pattern::new(pattern) {
            Ok(compiled) => Some(compiled),
            Err(e) => {
                log::warn!("Invalid {kind} pattern '{pattern}': {e}");
                None
            }
        })
        .collect()
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
        // When false, imports from test files are not counted as usage,
        // so code used only in tests is flagged as dead code.
        let count_test_imports: bool = rule.get_option("count_test_imports").unwrap_or(false);

        // If the detector was manually constructed with its own exclude list, prefer that.
        let mut combined_exclude = Vec::new();
        if self.exclude.is_empty() {
            combined_exclude.extend_from_slice(&rule.exclude);
        } else {
            combined_exclude.extend_from_slice(&self.exclude);
        }

        // Add config.ignore patterns, but filter out test patterns if count_test_imports=true
        // to avoid double-exclusion conflict.
        if count_test_imports {
            // Filter out test file patterns from config.ignore to allow counting test imports
            let test_patterns_set: HashSet<&str> = TEST_FILE_PATTERNS.iter().copied().collect();
            combined_exclude.extend(
                ctx.config
                    .ignore
                    .iter()
                    .filter(|p| !test_patterns_set.contains(p.as_str()))
                    .cloned(),
            );
        } else {
            // Include all ignore patterns
            combined_exclude.extend_from_slice(&ctx.config.ignore);
            // Explicitly add TEST_FILE_PATTERNS to ensure test files are always excluded
            // when count_test_imports=false, even if config.ignore is empty
            combined_exclude.extend(TEST_FILE_PATTERNS.iter().map(ToString::to_string));
        }

        let project_root = if self.project_root.as_os_str().is_empty() {
            ctx.project_path.clone()
        } else {
            self.project_root.clone()
        };

        let detector = Self::new(
            &ctx.config,
            ctx.script_entry_points.clone(),
            ctx.dynamic_load_patterns.clone(),
            &combined_exclude,
            project_root,
        );

        let symbol_imports = detector.build_symbol_imports_map(ctx.file_symbols.as_ref());
        let usage_index = detector.build_usage_index(ctx.file_symbols.as_ref());

        let dead_files = detector.find_dead_files(ctx, &symbol_imports, &usage_index);

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

    /// Gathers, in one pass, everything `is_dead_file` needs to know about the project.
    fn build_usage_index(
        &self,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
    ) -> UsageIndex {
        let mut index = UsageIndex::default();

        for (user_path, symbols) in file_symbols {
            if self.is_path_excluded(user_path) {
                continue;
            }

            for import in &symbols.imports {
                if import.name == "default" || import.name == "*" {
                    index.default_imports.insert(&import.source, user_path);
                }
            }

            for export in &symbols.exports {
                if export.is_reexport {
                    if let Some(source) = export.source.as_ref() {
                        index.reexports.insert(source, user_path);
                    }
                }
            }

            index
                .local_usages
                .extend(symbols.local_usages.iter().map(ToString::to_string));
        }

        index
    }

    fn find_dead_files(
        &self,
        ctx: &AnalysisContext,
        symbol_imports: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        usage_index: &UsageIndex,
    ) -> Vec<PathBuf> {
        let mut dead_files = Vec::new();

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                if self.is_path_excluded(path) {
                    continue;
                }
                if self.is_dead_file(path, ctx.file_symbols.as_ref(), symbol_imports, usage_index) {
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
        usage_index: &UsageIndex,
    ) -> bool {
        // We ignore fan_in from the graph because we've built our own symbol_imports map
        // that respects the detector-specific exclude patterns.
        !self.is_entry_point(path)
            && !self.matches_dynamic_load_pattern(path)
            && !Self::has_used_exports(path, file_symbols, symbol_imports, usage_index)
            && !usage_index.reexports.is_used(path)
    }

    #[must_use]
    pub fn new_default(config: &Config) -> Self {
        Self::new(config, HashSet::new(), Vec::new(), &[], PathBuf::new())
    }

    #[must_use]
    pub fn new(
        config: &Config,
        explicit_entry_points: HashSet<PathBuf>,
        dynamic_load_patterns: Vec<String>,
        exclude: &[String],
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

        Self {
            compiled_entry_patterns: compile_patterns(&patterns, "entry point"),
            explicit_entry_points,
            dynamic_load_patterns,
            exclude: exclude.to_vec(),
            compiled_exclude: compile_patterns(exclude, "exclude"),
            project_root,
        }
    }

    /// Project-relative, forward-slashed path — the form all glob patterns match against.
    fn relative_to_root(&self, path: &Path) -> String {
        path.strip_prefix(&self.project_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn is_path_excluded(&self, path: &Path) -> bool {
        if self.compiled_exclude.is_empty() {
            return false;
        }

        let relative_path = self.relative_to_root(path);

        for pattern in &self.compiled_exclude {
            if pattern.matches(&relative_path) {
                return true;
            }
        }

        false
    }

    fn has_used_exports(
        path: &Path,
        file_symbols: &HashMap<PathBuf, crate::parser::FileSymbols>,
        symbol_imports: &HashMap<(PathBuf, String), HashSet<PathBuf>>,
        usage_index: &UsageIndex,
    ) -> bool {
        let symbols = match file_symbols.get(path) {
            Some(s) => s,
            _ => return false,
        };

        Self::check_named_exports(symbols, path, symbol_imports)
            || usage_index.default_imports.is_used(path)
            || Self::check_local_usages(symbols, &usage_index.local_usages)
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

    fn check_local_usages(
        symbols: &crate::parser::FileSymbols,
        local_usages: &HashSet<String>,
    ) -> bool {
        symbols.exports.iter().any(|export| {
            !export.is_reexport
                && export.name != "default"
                && export.name != "*"
                && local_usages.contains(export.name.as_str())
        })
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

        let relative_path = self.relative_to_root(path);
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        // Matched against both forms so path-shaped patterns (`**/bin/**`, `src/main.ts`)
        // and bare-name patterns (`*.module.ts`, `index.ts`) both work — a glob `*` does
        // not cross directory separators, so neither form alone covers the other.
        self.compiled_entry_patterns
            .iter()
            .any(|pattern| pattern.matches(&relative_path) || pattern.matches(file_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_path_excluded_backslash_normalization() {
        let detector = DeadCodeDetector::new(
            &Config::default(),
            HashSet::new(),
            Vec::new(),
            &["src/ignored/*.ts".to_string()],
            PathBuf::from("/project"),
        );

        // Test that backslashes in paths are normalized to forward slashes for glob matching.
        // This ensures patterns using forward slashes match paths with backslashes.
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
            &["src/ignored/*.ts".to_string()],
            PathBuf::from("/project"),
        );

        let path = PathBuf::from("/project/src/ignored/file.ts");
        assert!(detector.is_path_excluded(&path));

        let path2 = PathBuf::from("/project/src/used/file.ts");
        assert!(!detector.is_path_excluded(&path2));
    }
}
