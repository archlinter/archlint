use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn init() {}

pub struct ScatteredConfigDetector;

pub struct ScatteredConfigDetectorFactory;

impl DetectorFactory for ScatteredConfigDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "scattered_config",
            name: "Scattered Configuration Detector",
            description:
                "Detects environment variables that are accessed from many different modules",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::Global,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(ScatteredConfigDetector)
    }
}

inventory::submit! {
    &ScatteredConfigDetectorFactory as &dyn DetectorFactory
}

impl Detector for ScatteredConfigDetector {
    fn name(&self) -> &'static str {
        "ScatteredConfiguration"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut var_usage: HashMap<String, Vec<PathBuf>> = HashMap::new();
        let thresholds = &ctx.config.thresholds.scattered_config;

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
            .filter(|(_, files)| files.len() > thresholds.max_files)
            .map(|(env_var, files)| ArchSmell::new_scattered_configuration(env_var, files))
            .collect()
    }
}
