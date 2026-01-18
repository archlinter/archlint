use crate::detectors::ArchSmell;
pub use crate::detectors::Explanation;
use crate::snapshot::SnapshotSmell;
use std::convert::TryFrom;
use std::path::Path;

pub struct ExplainEngine;

impl ExplainEngine {
    #[must_use]
    pub fn explain_snapshot_smell(
        smell: &SnapshotSmell,
        config: &crate::config::Config,
    ) -> Explanation {
        let arch_smell = ArchSmell::try_from(smell).unwrap_or_else(|_| {
            // Fallback to a minimal ArchSmell if conversion fails
            // This should rarely happen in practice
            ArchSmell {
                smell_type: crate::detectors::SmellType::Unknown {
                    raw_type: smell.smell_type.clone(),
                },
                severity: crate::detectors::Severity::Medium,
                files: smell.files.iter().map(std::path::PathBuf::from).collect(),
                metrics: vec![],
                locations: vec![],
                cluster: None,
            }
        });
        Self::explain(&arch_smell, config)
    }

    #[must_use]
    pub fn explain(
        smell: &crate::detectors::ArchSmell,
        config: &crate::config::Config,
    ) -> Explanation {
        // Try dynamic explanation from detectors
        let registry = crate::detectors::DetectorRegistry::new();
        let detector_id = smell.smell_type.category().to_id();
        if let Some(detector) = registry.create_detector(detector_id, config) {
            detector.explain(smell)
        } else {
            Self::simple_explanation("Unknown Smell", "No detailed explanation available")
        }
    }

    fn simple_explanation(problem: &str, reason: &str) -> Explanation {
        Explanation {
            problem: problem.to_string(),
            reason: reason.to_string(),
            risks: vec!["Increased maintenance cost".to_string()],
            recommendations: vec!["Refactor code to improve architecture".to_string()],
        }
    }

    #[must_use]
    pub fn format_file_path(path: &Path) -> String {
        path.to_string_lossy().to_string()
    }
}
