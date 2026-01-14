use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;
use std::path::Path;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(
    smell_type = BarrelFileAbuse,
    name = "Barrel File Abuse Detector",
    description = "Detects excessive use of barrel files (index.ts) that inflate the dependency graph",
    category = DetectorCategory::ImportBased
)]
pub struct BarrelFileAbuseDetector;

impl BarrelFileAbuseDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn is_barrel_file(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("index."))
            .unwrap_or(false)
    }
}

impl Detector for BarrelFileAbuseDetector {
    crate::impl_detector_report!(
        name: "BarrelFileAbuse",
        explain: _smell => (
            problem: "Barrel File Abuse",
            reason: "Excessive re-exports in index file. Large barrel files can lead to unnecessary coupling and slower build times.",
            risks: [
                "Increased build times",
                "Circular dependencies risk"
            ],
            recommendations: [
                "Split the barrel file or import directly from sub-modules"
            ]
        ),
        table: {
            title: "Barrel Files",
            columns: ["File", "Re-exports", "pts"],
            row: BarrelFileAbuse { } (smell, location, pts) => [
                location,
                smell.dependent_count().unwrap_or(0),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        ctx.file_symbols
            .as_ref()
            .iter()
            .filter_map(|(path, symbols)| {
                let rule = ctx.get_rule_for_file("barrel_file", path)?;

                if !self.is_barrel_file(path) {
                    return None;
                }

                let max_reexports: usize = rule.get_option("max_reexports").unwrap_or(10);

                let reexport_count = symbols
                    .exports
                    .iter()
                    .filter(|e| e.source.is_some()) // re-exports have a source
                    .count();

                if reexport_count > max_reexports {
                    let mut smell =
                        ArchSmell::new_barrel_abuse(path.clone(), reexport_count, false);
                    smell.severity = rule.severity;
                    Some(smell)
                } else {
                    None
                }
            })
            .collect()
    }
}
