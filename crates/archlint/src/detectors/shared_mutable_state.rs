use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::parser::SymbolKind;
use inventory;

pub fn init() {}

pub struct SharedMutableStateDetector;

pub struct SharedMutableStateDetectorFactory;

impl DetectorFactory for SharedMutableStateDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "shared_mutable_state",
            name: "Shared Mutable State Detector",
            description:
                "Detects exported mutable state (let/var) that can be modified from multiple places",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::FileLocal,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(SharedMutableStateDetector)
    }
}

inventory::submit! {
    &SharedMutableStateDetectorFactory as &dyn DetectorFactory
}

impl Detector for SharedMutableStateDetector {
    fn name(&self) -> &'static str {
        "SharedMutableState"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let rule = ctx.resolve_rule("shared_mutable_state", Some(path));
            if !rule.enabled || ctx.is_excluded(path, &rule.exclude) {
                continue;
            }

            for export in &symbols.exports {
                if export.is_mutable && export.kind == SymbolKind::Variable {
                    let mut smell =
                        ArchSmell::new_shared_mutable_state(path.clone(), export.name.to_string());
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
