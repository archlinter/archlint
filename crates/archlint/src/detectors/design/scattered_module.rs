use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use petgraph::graph::UnGraph;
use std::collections::HashSet;
use std::path::Path;

pub fn init() {}

#[detector(
    id = "module_cohesion",
    name = "Scattered Module Detector",
    description = "Detects modules where exports are unrelated to each other",
    category = DetectorCategory::Global,
    default_enabled = false
)]
pub struct ScatteredModuleDetector;

impl ScatteredModuleDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn is_barrel_file(&self, path: &Path, symbols: &crate::parser::FileSymbols) -> bool {
        let is_index = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("index."))
            .unwrap_or(false);

        let only_reexports = symbols.exports.iter().all(|e| e.source.is_some());

        is_index || only_reexports
    }

    fn calculate_components(&self, symbols: &crate::parser::FileSymbols) -> usize {
        let mut graph = UnGraph::<(), ()>::new_undirected();
        let mut export_nodes = Vec::new();

        for _ in 0..symbols.exports.len() {
            export_nodes.push(graph.add_node(()));
        }

        for i in 0..symbols.exports.len() {
            for j in (i + 1)..symbols.exports.len() {
                let e1 = &symbols.exports[i];
                let e2 = &symbols.exports[j];

                // Check if they share any used symbols
                let shared: HashSet<_> = e1.used_symbols.intersection(&e2.used_symbols).collect();

                // If they share symbols or one uses the other
                let one_uses_other =
                    e1.used_symbols.contains(&e2.name) || e2.used_symbols.contains(&e1.name);

                if !shared.is_empty() || one_uses_other {
                    graph.add_edge(export_nodes[i], export_nodes[j], ());
                }
            }
        }

        petgraph::algo::connected_components(&graph)
    }
}

impl Detector for ScatteredModuleDetector {
    fn name(&self) -> &'static str {
        "ScatteredModule"
    }

    fn explain(&self, _smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Scattered Module (Low Cohesion)".to_string(),
            reason: "Module exports are not related to each other, which means the module might be a 'catch-all' bucket for unrelated code.".to_string(),
            risks: vec!["Difficult to understand and reuse".to_string(), "Unrelated changes cascade through this module".to_string()],
            recommendations: vec!["Split the module into several smaller, cohesive modules".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::explain::ExplainEngine;
        crate::define_report_section!("Scattered Modules", smells, {
            crate::render_table!(
                vec!["File", "Components", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let file_path = smell.files.first().unwrap();
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let components = match &smell.smell_type {
                        SmellType::ScatteredModule { components } => components.to_string(),
                        _ => "unknown".to_string(),
                    };
                    let pts = smell.score(severity_config);
                    vec![
                        format!("`{}`", formatted_path),
                        components,
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let rule = match ctx.get_rule_for_file("module_cohesion", path) {
                Some(r) => r,
                None => continue,
            };

            let min_exports: usize = rule.get_option("min_exports").unwrap_or(5);
            let max_components: usize = rule.get_option("max_components").unwrap_or(2);

            // Ignore small files and barrels (barrels are handled by BarrelFileAbuseDetector)
            if symbols.exports.len() < min_exports || self.is_barrel_file(path, symbols) {
                continue;
            }

            let components = self.calculate_components(symbols);

            if components > max_components {
                let mut smell = ArchSmell::new_scattered_module(path.clone(), components);
                smell.severity = rule.severity;
                smells.push(smell);
            }
        }

        smells
    }
}
