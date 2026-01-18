use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use std::fs;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

#[detector(SmellType::LargeFile)]
pub struct LargeFileDetector;

impl LargeFileDetector {
    #[must_use]
    pub const fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    /// Check if file contains auto-generated markers in the first few lines
    fn is_auto_generated(&self, path: &std::path::Path) -> bool {
        const CHECK_LINES: usize = 20; // Check first 20 lines
        const AUTO_GEN_PATTERNS: &[&str] = &[
            "auto-generated",
            "auto generated",
            "This file was auto-generated",
            "This file was automatically generated",
            "generated automatically",
            "DO NOT EDIT",
            "do not edit",
            "@generated",
            "# generated",
            "// generated",
            "/* generated",
        ];

        if let Ok(content) = fs::read_to_string(path) {
            let lines: Vec<&str> = content.lines().take(CHECK_LINES).collect();
            let content_start = lines.join("\n").to_lowercase();

            for pattern in AUTO_GEN_PATTERNS {
                if content_start.contains(&pattern.to_lowercase()) {
                    return true;
                }
            }
        }
        false
    }
}

impl Detector for LargeFileDetector {
    crate::impl_detector_report!(
        explain: smell => (
            problem: format!("File has {} lines, exceeding the recommended limit", smell.lines().unwrap_or(0)),
            reason: "Large files are difficult to understand, navigate, and maintain. They often indicate a violation of the Single Responsibility Principle.",
            risks: [
                "Difficult to understand and navigate",
                "Higher chance of merge conflicts",
                "Slower code reviews and longer review times",
                "Often indicates mixed responsibilities",
                "IDE performance may be impacted"
            ],
            recommendations: [
                "Split the file by domain or functionality",
                "Extract utility functions into separate modules",
                "Identify cohesive groups of code and separate them",
                "Consider using barrel files to re-export split modules",
                "Apply Single Responsibility Principle (SRP)"
            ]
        ),
        table: {
            title: "Large Files",
            columns: ["File", "Lines", "pts"],
            row: LargeFile { } (smell, location, pts) => [location, smell.lines().unwrap_or(0), pts]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                // Skip auto-generated files
                if self.is_auto_generated(path) {
                    continue;
                }

                let rule = match ctx.get_rule_for_file("large_file", path) {
                    Some(r) => r,
                    None => continue,
                };

                let threshold: usize = rule
                    .get_option("max_lines")
                    .or(rule.get_option("lines"))
                    .unwrap_or(1000);

                if let Some(metrics) = ctx.file_metrics.get(path) {
                    if metrics.lines > threshold {
                        let mut smell =
                            ArchSmell::new_large_file(path.clone(), metrics.lines, threshold);
                        smell.severity = rule.severity;
                        smells.push(smell);
                    }
                }
            }
        }

        smells
    }
}
