use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use std::path::Path;

pub fn init() {}

pub struct TestLeakageDetector;

pub struct TestLeakageDetectorFactory;

impl DetectorFactory for TestLeakageDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "test_leakage",
            name: "Test to Production Leakage Detector",
            description:
                "Detects when production code imports test files, mocks, or test utilities",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::ImportBased,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(TestLeakageDetector)
    }
}

inventory::submit! {
    &TestLeakageDetectorFactory as &dyn DetectorFactory
}

impl Detector for TestLeakageDetector {
    fn name(&self) -> &'static str {
        "TestLeakage"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let thresholds = &ctx.config.thresholds.test_leakage;
        let test_patterns = Self::get_test_patterns(thresholds);

        ctx.graph
            .nodes()
            .filter_map(|node| {
                let from_path = ctx.graph.get_file_path(node)?;
                if !self.is_test_file(from_path, test_patterns.as_deref()) {
                    Some(Self::check_node_leakage(
                        ctx,
                        node,
                        test_patterns.as_deref(),
                        self,
                    ))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }
}

impl TestLeakageDetector {
    fn get_test_patterns(thresholds: &crate::config::TestLeakageThresholds) -> Option<Vec<String>> {
        if thresholds.test_patterns.is_empty() {
            None
        } else {
            Some(thresholds.test_patterns.clone())
        }
    }

    fn check_node_leakage(
        ctx: &AnalysisContext,
        node: petgraph::graph::NodeIndex,
        test_patterns: Option<&[String]>,
        detector: &TestLeakageDetector,
    ) -> Vec<ArchSmell> {
        let from_path = match ctx.graph.get_file_path(node) {
            Some(p) => p,
            None => return Vec::new(),
        };
        let mut smells = Vec::new();

        for to_node in ctx.graph.dependencies(node) {
            if let Some(to_path) = ctx.graph.get_file_path(to_node) {
                if detector.is_test_file(to_path, test_patterns) {
                    smells.push(ArchSmell::new_test_leakage(
                        from_path.clone(),
                        to_path.clone(),
                    ));
                }
            }
        }

        smells
    }
}

impl TestLeakageDetector {
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
            component
                .as_os_str()
                .to_str()
                .map(|s| {
                    matches!(
                        s,
                        "__tests__" | "__mocks__" | "test" | "tests" | "__fixtures__"
                    )
                })
                .unwrap_or(false)
        })
    }
}
