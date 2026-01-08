use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;

pub fn init() {}

pub struct LargeFileDetector;

pub struct LargeFileDetectorFactory;

impl DetectorFactory for LargeFileDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "large_file",
            name: "Large File Detector",
            description: "Detects files that exceed the recommended line count",
            default_enabled: true,
            is_deep: false,
            category: DetectorCategory::FileLocal,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(LargeFileDetector)
    }
}

inventory::submit! {
    &LargeFileDetectorFactory as &dyn DetectorFactory
}

impl Detector for LargeFileDetector {
    fn name(&self) -> &'static str {
        "LargeFile"
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
