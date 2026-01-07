use crate::config::Config;
use crate::detectors::DetectorCategory;
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
            id: "barrel_file",
            name: "Barrel File Abuse Detector",
            description:
                "Detects excessive use of barrel files (index.ts) that inflate the dependency graph",
            default_enabled: true,
            is_deep: false,
            category: DetectorCategory::ImportBased,
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

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let rule = ctx.resolve_rule("barrel_file", Some(path));
            if !rule.enabled
                || ctx.is_excluded(path, &rule.exclude)
                || ctx.should_skip_detector(path, "barrel_file")
            {
                continue;
            }

            if !self.is_barrel_file(path) {
                continue;
            }

            let max_reexports: usize = rule.get_option("max_reexports").unwrap_or(10);

            let reexport_count = symbols
                .exports
                .iter()
                .filter(|e| e.source.is_some()) // re-exports have a source
                .count();

            if reexport_count > max_reexports {
                let mut smell = ArchSmell::new_barrel_abuse(path.clone(), reexport_count, false);
                smell.severity = rule.severity;
                smells.push(smell);
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
