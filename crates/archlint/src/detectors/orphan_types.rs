use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::parser::{SymbolKind, SymbolName};
use inventory;
use std::collections::HashSet;
use std::path::PathBuf;

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

impl OrphanTypesDetector {
    fn collect_type_definitions<'a>(
        &self,
        ctx: &'a AnalysisContext,
    ) -> Vec<(&'a PathBuf, &'a SymbolName)> {
        ctx.file_symbols
            .as_ref()
            .iter()
            .flat_map(|(path, symbols)| {
                symbols
                    .exports
                    .iter()
                    .filter(|export| {
                        export.kind == SymbolKind::Type || export.kind == SymbolKind::Interface
                    })
                    .map(move |export| (path, &export.name))
            })
            .collect()
    }

    fn collect_all_usages(&self, ctx: &AnalysisContext) -> HashSet<SymbolName> {
        ctx.file_symbols
            .values()
            .flat_map(|symbols| {
                symbols
                    .local_usages
                    .iter()
                    .cloned()
                    .chain(symbols.imports.iter().map(|import| import.name.clone()))
            })
            .collect()
    }
}

impl Detector for OrphanTypesDetector {
    fn name(&self) -> &'static str {
        "OrphanTypes"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let _rule = match ctx.get_rule("orphan_types") {
            Some(r) => r,
            None => return Vec::new(),
        };

        let type_definitions = self.collect_type_definitions(ctx);
        let all_usages = self.collect_all_usages(ctx);

        type_definitions
            .into_iter()
            .filter(|(_, name)| !all_usages.contains(*name))
            .filter_map(|(path, name)| {
                let file_rule = ctx.get_rule_for_file("orphan_types", path)?;
                let mut smell = ArchSmell::new_orphan_type(path.clone(), name.to_string());
                smell.severity = file_rule.severity;
                Some(smell)
            })
            .collect()
    }
}
