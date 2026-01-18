use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use std::path::Path;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

#[detector(SmellType::TestLeakage, default_enabled = false)]
pub struct TestLeakageDetector;

impl TestLeakageDetector {
    #[must_use]
    pub const fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn check_node_leakage(
        &self,
        ctx: &AnalysisContext,
        node: petgraph::graph::NodeIndex,
        test_patterns: Option<&[String]>,
    ) -> Vec<ArchSmell> {
        let from_path = match ctx.graph.get_file_path(node) {
            Some(p) => p,
            None => return Vec::new(),
        };
        let mut smells = Vec::new();

        for to_node in ctx.graph.dependencies(node) {
            if let Some(to_path) = ctx.graph.get_file_path(to_node) {
                if self.is_test_file(to_path, test_patterns) {
                    let edge_data = ctx.graph.get_edge_data(node, to_node);
                    let (import_line, import_range) =
                        edge_data.map_or((0, None), |e| (e.import_line, e.import_range));

                    smells.push(ArchSmell::new_test_leakage(
                        from_path.clone(),
                        to_path.clone(),
                        import_line,
                        import_range,
                    ));
                }
            }
        }

        smells
    }

    fn is_test_file(&self, path: &Path, patterns: Option<&[String]>) -> bool {
        if let Some(patterns) = patterns {
            return self.matches_custom_patterns(path, patterns);
        }
        self.is_default_test_file(path)
    }

    fn matches_custom_patterns(&self, path: &Path, patterns: &[String]) -> bool {
        let path_str = path.to_string_lossy();
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        for pattern in patterns {
            if self.matches_pattern(&path_str, file_name, pattern) {
                return true;
            }
        }
        false
    }

    fn matches_pattern(&self, path_str: &str, file_name: &str, pattern: &str) -> bool {
        if pattern.contains("**") {
            let cleaned = pattern.replace("**", "");
            let parts: Vec<&str> = cleaned.split('*').filter(|p| !p.is_empty()).collect();
            parts.iter().all(|part| path_str.contains(part))
        } else if let Some(suffix) = pattern.strip_prefix('*') {
            file_name.ends_with(suffix)
        } else {
            path_str.contains(pattern) || file_name == pattern
        }
    }

    fn is_default_test_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        if self.is_test_extension(file_name) {
            return true;
        }

        if self.is_test_directory_path(&path_str) {
            return true;
        }

        self.has_test_component(path)
    }

    fn is_test_extension(&self, file_name: &str) -> bool {
        [
            ".test.ts", ".test.js", ".spec.ts", ".spec.js", ".mock.ts", ".mock.js",
        ]
        .iter()
        .any(|ext| file_name.ends_with(ext))
    }

    fn is_test_directory_path(&self, path_str: &str) -> bool {
        [
            "/__tests__/",
            "/__mocks__/",
            "/test/",
            "/tests/",
            "/__fixtures__/",
        ]
        .iter()
        .any(|p| path_str.contains(p))
            || ["/__tests__", "/__mocks__", "/test", "/tests"]
                .iter()
                .any(|p| path_str.ends_with(p))
    }

    fn has_test_component(&self, path: &Path) -> bool {
        path.components().any(|component| {
            component.as_os_str().to_str().is_some_and(|s| {
                matches!(
                    s,
                    "__tests__" | "__mocks__" | "test" | "tests" | "__fixtures__"
                )
            })
        })
    }
}

impl Detector for TestLeakageDetector {
    crate::impl_detector_report!(
        explain: _smell => (
            problem: "Test-to-Production Leakage",
            reason: "A production module imports a test file, mock, or test utility. This can lead to test code being included in production bundles.",
            risks: [
                "Increased bundle size",
                "Potential security risks if mocks expose internal data",
                "Code fragility: production depends on test helpers"
            ],
            recommendations: [
                "Move shared utilities to a separate non-test module",
                "Check if the import was accidental and remove it"
            ]
        ),
        table: {
            title: "Test Leakage",
            columns: ["Location", "Imported Test File", "pts"],
            row: TestLeakage { test_file } (smell, location, pts) => [
                location,
                test_file.to_string_lossy(),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let rule = match ctx.get_rule("test_leakage") {
            Some(r) => r,
            None => return Vec::new(),
        };

        let test_patterns: Vec<String> = rule.get_option("test_patterns").unwrap_or_else(|| {
            vec![
                "**/*.test.ts".to_string(),
                "**/*.test.js".to_string(),
                "**/*.spec.ts".to_string(),
                "**/*.spec.js".to_string(),
                "**/__tests__/**".to_string(),
                "**/__mocks__/**".to_string(),
            ]
        });

        ctx.graph
            .nodes()
            .flat_map(|node| {
                if let Some(from_path) = ctx.graph.get_file_path(node) {
                    if let Some(file_rule) = ctx.get_rule_for_file("test_leakage", from_path) {
                        if !self.is_test_file(from_path, Some(&test_patterns)) {
                            let mut node_smells =
                                self.check_node_leakage(ctx, node, Some(&test_patterns));
                            for smell in &mut node_smells {
                                smell.severity = file_rule.severity;
                            }
                            return node_smells;
                        }
                    }
                }
                Vec::new()
            })
            .collect()
    }
}
