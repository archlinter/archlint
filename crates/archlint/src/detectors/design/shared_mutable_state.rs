use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use crate::parser::SymbolKind;

pub fn init() {}

#[detector(
    id = "shared_mutable_state",
    name = "Shared Mutable State Detector",
    description = "Detects exported mutable state (let/var) that can be modified from multiple places",
    category = DetectorCategory::FileLocal,
    default_enabled = false
)]
pub struct SharedMutableStateDetector;

impl SharedMutableStateDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for SharedMutableStateDetector {
    fn name(&self) -> &'static str {
        "SharedMutableState"
    }

    fn explain(&self, smell: &ArchSmell) -> Explanation {
        let symbol = match &smell.smell_type {
            SmellType::SharedMutableState { symbol } => symbol.clone(),
            _ => "unknown".to_string(),
        };
        Explanation {
            problem: "Shared Mutable State".to_string(),
            reason: format!("Exported mutable state (let/var) `{}` can be modified from multiple modules, which often leads to bugs that are hard to trace and race conditions.", symbol),
            risks: vec!["Unpredictable side effects".to_string(), "Difficult to debug state changes".to_string()],
            recommendations: vec!["Encapsulate state within a class or use a state management pattern with immutable data".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::explain::ExplainEngine;
        crate::define_report_section!("Shared Mutable State", smells, {
            crate::render_table!(
                vec!["File", "Symbol", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let file_path = smell.files.first().unwrap();
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let symbol = match &smell.smell_type {
                        SmellType::SharedMutableState { symbol } => symbol.clone(),
                        _ => "unknown".to_string(),
                    };
                    let pts = smell.score(severity_config);
                    vec![
                        format!("`{}`", formatted_path),
                        format!("`{}`", symbol),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

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
