use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::config::Config;
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
        let threshold = ctx.config.thresholds.large_file.lines;

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                if let Some(metrics) = ctx.file_metrics.get(path) {
                    if metrics.lines >= threshold {
                        smells.push(ArchSmell::new_large_file(path.clone(), metrics.lines));
                    }
                }
            }
        }

        smells
    }
}
