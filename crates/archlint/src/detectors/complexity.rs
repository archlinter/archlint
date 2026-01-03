use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::parser::FunctionComplexity;
use inventory;
use std::path::Path;

pub fn init() {}

pub struct ComplexityDetector;

pub struct ComplexityDetectorFactory;

impl DetectorFactory for ComplexityDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "complexity",
            name: "Complexity Detector",
            description: "Detects functions and files with high cyclomatic complexity",
            default_enabled: true,
            is_deep: false,
            category: DetectorCategory::FileLocal,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(ComplexityDetector)
    }
}

inventory::submit! {
    &ComplexityDetectorFactory as &dyn DetectorFactory
}

impl Detector for ComplexityDetector {
    fn name(&self) -> &'static str {
        "Complexity"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.complexity;

        for (file_path, functions) in ctx.function_complexity.as_ref() {
            for func in functions {
                if func.complexity >= thresholds.function_threshold {
                    smells.push(ArchSmell::new_high_complexity(
                        file_path.clone(),
                        func.name.to_string(),
                        func.line,
                        func.complexity,
                        thresholds.function_threshold,
                        Some(func.range),
                    ));
                }
            }
        }

        smells
    }
}

impl ComplexityDetector {
    pub fn detect_file(
        file_path: &Path,
        functions: &[FunctionComplexity],
        threshold: usize,
    ) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for func in functions {
            if func.complexity >= threshold {
                smells.push(ArchSmell::new_high_complexity(
                    file_path.to_path_buf(),
                    func.name.to_string(),
                    func.line,
                    func.complexity,
                    threshold,
                    Some(func.range),
                ));
            }
        }

        smells
    }
}
