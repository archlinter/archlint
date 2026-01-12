use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;
use std::collections::HashMap;
use std::path::PathBuf;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
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
    crate::impl_detector_report!(
        name: "ScatteredConfiguration",
        explain: smell => (
            problem: {
                if let crate::detectors::SmellType::ScatteredConfiguration { env_var, .. } = &smell.smell_type {
                    format!("Scattered Configuration: `{}`", env_var)
                } else {
                    "Scattered Configuration".into()
                }
            },
            reason: "Configuration (like environment variables) is spread across many different files, making it hard to track where settings are used.",
            risks: [
                "Difficult to audit configuration",
                "Potential bugs when settings change"
            ],
            recommendations: [
                "Centralize configuration access in a dedicated module or service"
            ]
        ),
        table: {
            title: "Scattered Configuration",
            columns: ["Env Var", "Usage Count", "pts"],
            row: ScatteredConfiguration { env_var, files_count } (smell, location, pts) => [env_var, format!("{} files", files_count), pts]
        }
    );

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
