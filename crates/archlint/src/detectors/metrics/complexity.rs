use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;

pub fn init() {}

#[detector(
    id = "complexity",
    name = "Complexity Detector",
    description = "Detects functions and files with high cyclomatic complexity",
    category = DetectorCategory::FileLocal
)]
pub struct ComplexityDetector;

impl ComplexityDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    pub fn detect_file(
        file_path: &std::path::Path,
        functions: &[crate::parser::FunctionComplexity],
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

impl Detector for ComplexityDetector {
    crate::impl_detector_report!(
        name: "Complexity",
        explain: smell => (
            problem: {
                if let crate::detectors::SmellType::HighComplexity { name, complexity, .. } = &smell.smell_type {
                    format!("Function `{}` has high cyclomatic complexity ({})", name, complexity)
                } else {
                    "High cyclomatic complexity".into()
                }
            },
            reason: "High cyclomatic complexity indicates that the function has too many decision points (if, for, while, etc.), making it difficult to understand, test, and maintain.",
            risks: [
                "Higher probability of bugs due to complex logic",
                "Difficult to achieve high test coverage",
                "Hard for other developers to read and understand",
                "Refactoring becomes dangerous and difficult"
            ],
            recommendations: [
                "Extract complex nested logic into smaller, focused helper functions",
                "Use early returns to reduce nesting depth",
                "Simplify logical expressions",
                "Consider using design patterns like Strategy or Command for complex branching",
                "Break down large switch statements"
            ]
        ),
        table: {
            title: "High Complexity Functions",
            columns: ["Location", "Function", "Complexity", "Score"],
            row: HighComplexity { name, complexity } (smell, location, pts) => [location, name, complexity, pts]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, functions) in ctx.function_complexity.as_ref() {
            let rule = match ctx.get_rule_for_file("complexity", path) {
                Some(r) => r,
                None => continue,
            };

            let function_threshold: usize = rule
                .get_option("max_complexity")
                .or(rule.get_option("function_threshold"))
                .unwrap_or(15);

            for func in functions {
                if func.complexity >= function_threshold {
                    let mut smell = ArchSmell::new_high_complexity(
                        path.clone(),
                        func.name.to_string(),
                        func.line,
                        func.complexity,
                        function_threshold,
                        Some(func.range),
                    );
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
