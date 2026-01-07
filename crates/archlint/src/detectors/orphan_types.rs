use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::parser::SymbolKind;
use inventory;
use std::collections::HashSet;

pub fn init() {}

pub struct OrphanTypesDetector;

pub struct OrphanTypesDetectorFactory;

impl DetectorFactory for OrphanTypesDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "orphan_types",
            name: "Orphan Types Detector",
            description: "Detects exported types or interfaces that are never used",
            default_enabled: true,
            is_deep: false,
            category: DetectorCategory::Global,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(OrphanTypesDetector)
    }
}

inventory::submit! {
    &OrphanTypesDetectorFactory as &dyn DetectorFactory
}

impl Detector for OrphanTypesDetector {
    fn name(&self) -> &'static str {
        "OrphanTypes"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let rule = ctx.resolve_rule("orphan_types", None);
        if !rule.enabled {
            return Vec::new();
        }

        let mut smells = Vec::new();

        // 1. Collect all exported types/interfaces and where they are
        let mut type_definitions = Vec::new();
        for (path, symbols) in ctx.file_symbols.as_ref() {
            for export in &symbols.exports {
                if export.kind == SymbolKind::Type || export.kind == SymbolKind::Interface {
                    type_definitions.push((path, &export.name));
                }
            }
        }

        // 2. Collect all usages across the whole project
        let mut all_usages = HashSet::new();
        for symbols in ctx.file_symbols.values() {
            for usage in &symbols.local_usages {
                all_usages.insert(usage.clone());
            }
            // Also check imports
            for import in &symbols.imports {
                all_usages.insert(import.name.clone());
            }
        }

        // 3. Flag those that are never used
        for (path, name) in type_definitions {
            if !all_usages.contains(name) {
                let file_rule = ctx.resolve_rule("orphan_types", Some(path));
                if !file_rule.enabled || ctx.is_excluded(path, &file_rule.exclude) {
                    continue;
                }
                let mut smell = ArchSmell::new_orphan_type(path.clone(), name.to_string());
                smell.severity = file_rule.severity;
                smells.push(smell);
            }
        }

        smells
    }
}
