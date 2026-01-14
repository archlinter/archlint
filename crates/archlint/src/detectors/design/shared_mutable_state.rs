use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use crate::parser::SymbolKind;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(SmellType::SharedMutableState, default_enabled = false)]
pub struct SharedMutableStateDetector;

impl SharedMutableStateDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for SharedMutableStateDetector {
    crate::impl_detector_report!(
        explain: smell => (
            problem: {
                if let crate::detectors::SmellType::SharedMutableState { symbol } = &smell.smell_type {
                    format!("Shared Mutable State: `{}`", symbol)
                } else {
                    "Shared Mutable State".into()
                }
            },
            reason: "Exported mutable state (let/var) can be modified from multiple modules, which often leads to bugs that are hard to trace and race conditions.",
            risks: [
                "Unpredictable side effects",
                "Difficult to debug state changes"
            ],
            recommendations: [
                "Encapsulate state within a class or use a state management pattern with immutable data"
            ]
        ),
        table: {
            title: "Shared Mutable State",
            columns: ["File", "Symbol", "pts"],
            row: SharedMutableState { symbol } (smell, location, pts) => [location, symbol, pts]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let rule = match ctx.get_rule_for_file("shared_mutable_state", path) {
                Some(r) => r,
                None => continue,
            };

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
