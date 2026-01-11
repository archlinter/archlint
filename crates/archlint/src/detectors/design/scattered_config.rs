use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn init() {}

#[detector(
    id = "scattered_config",
    name = "Scattered Configuration Detector",
    description = "Detects environment variables that are accessed from many different modules",
    category = DetectorCategory::Global,
    default_enabled = false
)]
pub struct ScatteredConfigDetector;

impl ScatteredConfigDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for ScatteredConfigDetector {
    fn name(&self) -> &'static str {
        "ScatteredConfiguration"
    }

    fn explain(&self, _smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Scattered Configuration".to_string(),
            reason: "Configuration (like environment variables) is spread across many different files, making it hard to track where settings are used.".to_string(),
            risks: vec!["Difficult to audit configuration".to_string(), "Potential bugs when settings change".to_string()],
            recommendations: vec!["Centralize configuration access in a dedicated module or service".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        crate::define_report_section!("Scattered Configuration", smells, {
            crate::render_table!(
                vec!["Env Var", "Usage Count", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let (env_var, count) = match &smell.smell_type {
                        SmellType::ScatteredConfiguration {
                            env_var,
                            files_count,
                        } => (env_var.clone(), *files_count),
                        _ => ("unknown".to_string(), 0),
                    };
                    let pts = smell.score(severity_config);
                    vec![
                        format!("`{}`", env_var),
                        format!("{} files", count),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let rule = match ctx.get_rule("scattered_config") {
            Some(r) => r,
            None => return Vec::new(),
        };

        let mut var_usage: HashMap<String, Vec<PathBuf>> = HashMap::new();
        let max_files: usize = rule.get_option("max_files").unwrap_or(3);

        for (path, symbols) in ctx.file_symbols.as_ref() {
            for var in &symbols.env_vars {
                var_usage
                    .entry(var.to_string())
                    .or_default()
                    .push(path.clone());
            }
        }

        var_usage
            .into_iter()
            .filter(|(_, files)| files.len() > max_files)
            .map(|(env_var, files)| {
                let mut smell = ArchSmell::new_scattered_configuration(env_var, files);
                smell.severity = rule.severity;
                smell
            })
            .collect()
    }
}
