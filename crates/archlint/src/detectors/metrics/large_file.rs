use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(
    smell_type = SmellType::LargeFile,
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
    crate::impl_detector_report!(
        name: "LargeFile",
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
