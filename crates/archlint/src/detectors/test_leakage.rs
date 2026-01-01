use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::config::Config;
use std::path::Path;
use inventory;

pub fn init() {}

pub struct TestLeakageDetector;

pub struct TestLeakageDetectorFactory;

impl DetectorFactory for TestLeakageDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "test_leakage",
            name: "Test to Production Leakage Detector",
            description: "Detects when production code imports test files, mocks, or test utilities",
            default_enabled: false,
            is_deep: false,
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
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.test_leakage;

        let test_patterns = if thresholds.test_patterns.is_empty() {
            // Use defaults if not provided
            None
        } else {
            Some(&thresholds.test_patterns)
        };

        for node in ctx.graph.nodes() {
            if let Some(from_path) = ctx.graph.get_file_path(node) {
                // If the source file is NOT a test file
                if !self.is_test_file(from_path, test_patterns) {
                    for to_node in ctx.graph.dependencies(node) {
                        if let Some(to_path) = ctx.graph.get_file_path(to_node) {
                            // If the target file IS a test file
                            if self.is_test_file(to_path, test_patterns) {
                                smells.push(ArchSmell::new_test_leakage(from_path.clone(), to_path.clone()));
                            }
                        }
                    }
                }
            }
        }

        smells
    }
}

impl TestLeakageDetector {
    fn is_test_file(&self, path: &Path, patterns: Option<&Vec<String>>) -> bool {
        let path_str = path.to_string_lossy();
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        if let Some(patterns) = patterns {
            for pattern in patterns {
                if pattern.contains("**") {
                    let cleaned = pattern.replace("**", "");
                    let parts: Vec<&str> = cleaned.split('*').filter(|p| !p.is_empty()).collect();
                    if parts.iter().all(|part| path_str.contains(part)) {
                        return true;
                    }
                } else if let Some(suffix) = pattern.strip_prefix('*') {
                    if file_name.ends_with(suffix) {
                        return true;
                    }
                } else if path_str.contains(pattern) || file_name == pattern {
                    return true;
                }
            }
            return false;
        }

        // Default logic if no patterns provided
        if file_name.ends_with(".test.ts") || file_name.ends_with(".test.js") ||
           file_name.ends_with(".spec.ts") || file_name.ends_with(".spec.js") ||
           file_name.ends_with(".mock.ts") || file_name.ends_with(".mock.js") {
            return true;
        }

        if path_str.contains("/__tests__/") || path_str.contains("/__mocks__/") ||
           path_str.contains("/test/") || path_str.contains("/tests/") ||
           path_str.contains("/__fixtures__/") ||
           path_str.ends_with("/__tests__") || path_str.ends_with("/__mocks__") ||
           path_str.ends_with("/test") || path_str.ends_with("/tests") {
            return true;
        }

        // Also check if path parts contain these directories
        for component in path.components() {
            if let Some(s) = component.as_os_str().to_str() {
                if s == "__tests__" || s == "__mocks__" || s == "test" || s == "tests" || s == "__fixtures__" {
                    return true;
                }
            }
        }

        false
    }
}
