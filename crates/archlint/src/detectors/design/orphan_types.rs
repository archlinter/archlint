use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use crate::parser::{SymbolKind, SymbolName};
use std::collections::HashSet;
use std::path::PathBuf;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(SmellType::OrphanType)]
pub struct OrphanTypesDetector;

impl OrphanTypesDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

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
    crate::impl_detector_report!(
        explain: smell => (
            problem: {
                let name = match &smell.smell_type {
                    crate::detectors::SmellType::OrphanType { name } => name.clone(),
                    _ => "unknown".to_string(),
                };
                format!("Orphan Type `{}` detected", name)
            },
            reason: "The type or interface is exported but never used by any other module in the codebase.",
            risks: [
                "Increased maintenance cost",
                "Confusion about the public API"
            ],
            recommendations: [
                "Remove the unused type or interface"
            ]
        ),
        table: {
            title: "Orphan Types",
            columns: ["File", "Type Name", "pts"],
            row: OrphanType { name } (smell, location, pts) => [location, name, pts]
        }
    );

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
            .flat_map(|(path, name)| {
                let file_rule = ctx.get_rule_for_file("orphan_types", path)?;
                let mut smell = ArchSmell::new_orphan_type(path.clone(), name.to_string());
                smell.severity = file_rule.severity;
                Some(smell)
            })
            .collect()
    }
}
