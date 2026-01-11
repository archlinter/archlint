use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellWithExplanation,
};
use crate::engine::AnalysisContext;

pub fn init() {}

#[detector(
    id = "large_file",
    name = "Large File Detector",
    description = "Detects files that exceed the recommended line count",
    category = DetectorCategory::FileLocal
)]
pub struct LargeFileDetector;

impl LargeFileDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for LargeFileDetector {
    fn name(&self) -> &'static str {
        "LargeFile"
    }

    fn explain(&self, smell: &ArchSmell) -> Explanation {
        let lines = smell.lines().unwrap_or(0);

        Explanation {
            problem: format!("File has {} lines, exceeding the recommended limit", lines),
            reason: "Large files are difficult to understand, navigate, and maintain. They often indicate a violation of the Single Responsibility Principle.".to_string(),
            risks: vec![
                "Difficult to understand and navigate".to_string(),
                "Higher chance of merge conflicts".to_string(),
                "Slower code reviews and longer review times".to_string(),
                "Often indicates mixed responsibilities".to_string(),
                "IDE performance may be impacted".to_string(),
            ],
            recommendations: vec![
                "Split the file by domain or functionality".to_string(),
                "Extract utility functions into separate modules".to_string(),
                "Identify cohesive groups of code and separate them".to_string(),
                "Consider using barrel files to re-export split modules".to_string(),
                "Apply Single Responsibility Principle (SRP)".to_string(),
            ],
        }
    }

    fn render_markdown(
        &self,
        large_files: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::detectors::Severity;
        use crate::explain::ExplainEngine;

        crate::define_report_section!("Large Files", large_files, {
            crate::render_table!(
                vec!["File", "Lines", "Severity"],
                large_files,
                |&(smell, _): &&SmellWithExplanation| {
                    let file_path = smell.files.first().unwrap();
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let lines = smell.lines().unwrap_or(0);
                    let effective_severity = smell.effective_severity();
                    let score = smell.score(severity_config);

                    let severity_str = match effective_severity {
                        Severity::Low => "🔵 Low",
                        Severity::Medium => "🟡 Medium",
                        Severity::High => "🟠 High",
                        Severity::Critical => "🔴 Critical",
                    };

                    vec![
                        format!("`{}`", formatted_path),
                        lines.to_string(),
                        format!("{} ({} pts)", severity_str, score),
                    ]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                let rule = match ctx.get_rule_for_file("large_file", path) {
                    Some(r) => r,
                    None => continue,
                };

                let threshold: usize = rule
                    .get_option("max_lines")
                    .or(rule.get_option("lines"))
                    .unwrap_or(1000);

                if let Some(metrics) = ctx.file_metrics.get(path) {
                    if metrics.lines >= threshold {
                        let mut smell = ArchSmell::new_large_file(path.clone(), metrics.lines);
                        smell.severity = rule.severity;
                        smells.push(smell);
                    }
                }
            }
        }

        smells
    }
}
