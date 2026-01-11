use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use crate::parser::{SymbolKind, SymbolName};
use std::collections::HashSet;
use std::path::PathBuf;

pub fn init() {}

#[detector(
    id = "orphan_types",
    name = "Orphan Types Detector",
    description = "Detects exported types or interfaces that are never used",
    category = DetectorCategory::Global
)]
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
    fn name(&self) -> &'static str {
        "OrphanTypes"
    }

    fn explain(&self, smell: &ArchSmell) -> Explanation {
        let name = match &smell.smell_type {
            SmellType::OrphanType { name } => name.clone(),
            _ => "unknown".to_string(),
        };
        Explanation {
            problem: format!("Orphan Type `{}` detected", name),
            reason: format!("The type or interface `{}` is exported but never used by any other module in the codebase.", name),
            risks: vec!["Increased maintenance cost".to_string(), "Confusion about the public API".to_string()],
            recommendations: vec!["Remove the unused type or interface".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::explain::ExplainEngine;
        crate::define_report_section!("Orphan Types", smells, {
            crate::render_table!(
                vec!["File", "Type Name", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let file_path = smell.files.first().unwrap();
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let name = match &smell.smell_type {
                        SmellType::OrphanType { name } => name.clone(),
                        _ => "unknown".to_string(),
                    };
                    let pts = smell.score(severity_config);
                    vec![
                        format!("`{}`", formatted_path),
                        format!("`{}`", name),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
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
