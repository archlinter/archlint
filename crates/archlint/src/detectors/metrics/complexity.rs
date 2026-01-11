use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use std::path::Path;

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
        file_path: &Path,
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
    fn name(&self) -> &'static str {
        "Complexity"
    }

    fn explain(&self, smell: &ArchSmell) -> Explanation {
        let name = match &smell.smell_type {
            SmellType::HighComplexity { name, .. } => name.clone(),
            _ => "unknown".to_string(),
        };

        let complexity = smell.complexity().unwrap_or(0);

        Explanation {
            problem: format!(
                "Function `{}` has high cyclomatic complexity ({})",
                name, complexity
            ),
            reason: "High cyclomatic complexity indicates that the function has too many decision points (if, for, while, etc.), making it difficult to understand, test, and maintain.".to_string(),
            risks: vec![
                "Higher probability of bugs due to complex logic".to_string(),
                "Difficult to achieve high test coverage".to_string(),
                "Hard for other developers to read and understand".to_string(),
                "Refactoring becomes dangerous and difficult".to_string(),
            ],
            recommendations: vec![
                "Extract complex nested logic into smaller, focused helper functions".to_string(),
                "Use early returns to reduce nesting depth".to_string(),
                "Simplify logical expressions".to_string(),
                "Consider using design patterns like Strategy or Command for complex branching".to_string(),
                "Break down large switch statements".to_string(),
            ],
        }
    }

    fn render_markdown(
        &self,
        high_complexity: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::report::format_location_detail;

        crate::define_report_section!("High Complexity Functions", high_complexity, {
            crate::render_table!(
                vec!["Location", "Function", "Complexity", "Score"],
                high_complexity,
                |&(smell, _): &&SmellWithExplanation| {
                    if let SmellType::HighComplexity { name, line, .. } = &smell.smell_type {
                        let file_path = smell.files.first().unwrap();
                        let location = smell
                            .locations
                            .first()
                            .map(format_location_detail)
                            .unwrap_or_else(|| {
                                crate::report::format_location(file_path, *line, None)
                            });
                        let complexity = smell.complexity().unwrap_or(0);
                        let score = smell.score(severity_config);

                        vec![
                            format!("`{}`", location),
                            format!("`{}`", name),
                            complexity.to_string(),
                            format!("{} pts", score),
                        ]
                    } else {
                        vec!["-".into(); 4]
                    }
                }
            )
        })
    }

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
