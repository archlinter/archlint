use crate::config::Config;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use std::path::Path;

pub fn init() {}

pub struct BarrelFileAbuseDetector;

pub struct BarrelFileAbuseDetectorFactory;

impl DetectorFactory for BarrelFileAbuseDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "barrel_file_abuse",
            name: "Barrel File Abuse Detector",
            description: "Detects excessive use of barrel files (index.ts) that inflate the dependency graph",
            default_enabled: true,
            is_deep: false,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(BarrelFileAbuseDetector)
    }
}

inventory::submit! {
    &BarrelFileAbuseDetectorFactory as &dyn DetectorFactory
}

impl Detector for BarrelFileAbuseDetector {
    fn name(&self) -> &'static str {
        "BarrelFileAbuse"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.barrel_file;

        for (path, symbols) in &ctx.file_symbols {
            if ctx.should_skip_detector(path, "barrel_file_abuse") {
                continue;
            }

            if !self.is_barrel_file(path) {
                continue;
            }

            let reexport_count = symbols.exports.iter()
                .filter(|e| e.source.is_some()) // re-exports have a source
                .count();

            if reexport_count > thresholds.max_reexports {
                smells.push(ArchSmell::new_barrel_abuse(
                    path.clone(),
                    reexport_count,
                    false,
                ));
            }
        }

        smells
    }
}

impl BarrelFileAbuseDetector {
    fn is_barrel_file(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("index."))
            .unwrap_or(false)
    }
}
