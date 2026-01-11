use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;

pub fn init() {}

#[detector(
    id = "side_effect_import",
    name = "Side-Effect Import Detector",
    description = "Detects imports that execute code on load without binding any symbols",
    category = DetectorCategory::ImportBased
)]
pub struct SideEffectImportDetector;

impl SideEffectImportDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn is_ignored_source(&self, source: &str) -> bool {
        source.ends_with(".css")
            || source.ends_with(".scss")
            || source.ends_with(".sass")
            || source.ends_with(".less")
            || source == "reflect-metadata"
            || source.contains("polyfill")
            || source.contains("setup")
            || source.contains("instrument")
            || source.contains("register")
    }
}

impl Detector for SideEffectImportDetector {
    crate::impl_detector_report!(
        name: "SideEffectImport",
        explain: _smell => {
            crate::detectors::Explanation {
                problem: "Side-Effect Import".into(),
                reason: "Import that executes code on load without binding any symbols. This can make the code's behavior unpredictable and difficult to test.".into(),
                risks: crate::strings![
                    "Global state pollution",
                    "Unpredictable execution order"
                ],
                recommendations: crate::strings![
                    "Explicitly initialize the module or use named imports if possible"
                ]
            }
        },
        table: {
            title: "Side-Effect Imports",
            columns: ["Location", "Source", "pts"],
            row: SideEffectImport { } (smell, location, pts) => [
                location,
                smell.locations.first().map(|l| l.description.replace("Side-effect import of ", "").replace("'", "")).unwrap_or_else(|| "unknown".into()),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let rule = match ctx.get_rule_for_file("side_effect_import", path) {
                Some(r) => r,
                None => continue,
            };

            for import in &symbols.imports {
                if import.name == "*"
                    && import.alias.is_none()
                    && !import.is_reexport
                    && !import.is_dynamic
                {
                    if self.is_ignored_source(&import.source) {
                        continue;
                    }

                    let mut smell = ArchSmell::new_side_effect_import(
                        path.clone(),
                        import.source.to_string(),
                        import.line,
                    );
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
