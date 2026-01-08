use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;

pub fn init() {}

pub struct SideEffectImportDetector;

pub struct SideEffectImportDetectorFactory;

impl DetectorFactory for SideEffectImportDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "side_effect_import",
            name: "Side-Effect Import Detector",
            description: "Detects imports that execute code on load without binding any symbols",
            default_enabled: true,
            is_deep: false,
            category: DetectorCategory::ImportBased,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(SideEffectImportDetector)
    }
}

inventory::submit! {
    &SideEffectImportDetectorFactory as &dyn DetectorFactory
}

impl Detector for SideEffectImportDetector {
    fn name(&self) -> &'static str {
        "SideEffectImport"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let rule = match ctx.get_rule_for_file("side_effect_import", path) {
                Some(r) => r,
                None => continue,
            };

            for import in &symbols.imports {
                if import.name == "*" && import.alias.is_none() && !import.is_reexport {
                    // Check if it's a CSS file or similar (should be ignored)
                    if self.is_ignored_source(&import.source) {
                        continue;
                    }

                    let mut smell =
                        ArchSmell::new_side_effect_import(path.clone(), import.source.to_string());
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}

impl SideEffectImportDetector {
    fn is_ignored_source(&self, source: &str) -> bool {
        source.ends_with(".css")
            || source.ends_with(".scss")
            || source.ends_with(".sass")
            || source.ends_with(".less")
            || source == "reflect-metadata"
            || source.contains("polyfill")
            || source.contains("setup")
            || source.contains("instrument")
            || source.contains("register") // common pattern for side-effects that are sometimes intentional
    }
}
