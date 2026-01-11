use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use std::path::Path;

pub fn init() {}

#[detector(
    id = "barrel_file",
    name = "Barrel File Abuse Detector",
    description = "Detects excessive use of barrel files (index.ts) that inflate the dependency graph",
    category = DetectorCategory::ImportBased
)]
pub struct BarrelFileAbuseDetector;

impl BarrelFileAbuseDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn is_barrel_file(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("index."))
            .unwrap_or(false)
    }
}

impl Detector for BarrelFileAbuseDetector {
    fn name(&self) -> &'static str {
        "BarrelFileAbuse"
    }

    fn explain(&self, _smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Barrel File Abuse".to_string(),
            reason: "Excessive re-exports in index file. Large barrel files can lead to unnecessary coupling and slower build times.".to_string(),
            risks: vec!["Increased build times".to_string(), "Circular dependencies risk".to_string()],
            recommendations: vec!["Split the barrel file or import directly from sub-modules".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::explain::ExplainEngine;
        crate::define_report_section!("Barrel Files", smells, {
            crate::render_table!(
                vec!["File", "Re-exports", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let file_path = smell.files.first().unwrap();
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let count = smell.dependent_count().unwrap_or(0);
                    let pts = smell.score(severity_config);
                    vec![
                        format!("`{}`", formatted_path),
                        count.to_string(),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let rule = match ctx.get_rule_for_file("barrel_file", path) {
                Some(r) => r,
                None => continue,
            };

            if !self.is_barrel_file(path) {
                continue;
            }

            let max_reexports: usize = rule.get_option("max_reexports").unwrap_or(10);

            let reexport_count = symbols
                .exports
                .iter()
                .filter(|e| e.source.is_some()) // re-exports have a source
                .count();

            if reexport_count > max_reexports {
                let mut smell = ArchSmell::new_barrel_abuse(path.clone(), reexport_count, false);
                smell.severity = rule.severity;
                smells.push(smell);
            }
        }

        smells
    }
}
