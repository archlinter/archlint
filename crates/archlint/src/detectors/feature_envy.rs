use crate::config::Config;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct FeatureEnvyDetector;

pub struct FeatureEnvyDetectorFactory;

impl DetectorFactory for FeatureEnvyDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "feature_envy",
            name: "Feature Envy Detector",
            description: "Detects modules that use more external symbols than internal ones",
            default_enabled: false,
            is_deep: false,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(FeatureEnvyDetector)
    }
}

inventory::submit! {
    &FeatureEnvyDetectorFactory as &dyn DetectorFactory
}

impl FeatureEnvyDetector {}

impl Detector for FeatureEnvyDetector {
    fn name(&self) -> &'static str {
        "FeatureEnvy"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let thresholds = &ctx.config.thresholds.feature_envy;

        ctx.file_symbols
            .iter()
            .filter_map(|(path, symbols)| {
                let mut internal_refs = 0;
                for def in &symbols.local_definitions {
                    if symbols.local_usages.contains(def) {
                        internal_refs += 1;
                    }
                }
                for exp in &symbols.exports {
                    if symbols.local_usages.contains(&exp.name) {
                        internal_refs += 1;
                    }
                }

                let mut external_refs = 0;
                let mut source_usages: HashMap<String, usize> = HashMap::new();

                for import in &symbols.imports {
                    let name_to_check = import.alias.as_ref().unwrap_or(&import.name);
                    if symbols.local_usages.contains(name_to_check) {
                        external_refs += 1;
                        *source_usages.entry(import.source.to_string()).or_insert(0) += 1;
                    }
                }

                let ratio = external_refs as f64 / (internal_refs as f64 + 1.0);

                if ratio >= thresholds.ratio && external_refs > 0 {
                    let (most_envied_source, _) =
                        source_usages.into_iter().max_by_key(|&(_, count)| count)?;

                    let most_envied_path = if most_envied_source.starts_with('.') {
                        // Simple resolution, good enough for the report
                        let mut p = path.parent()?.to_path_buf();
                        p.push(most_envied_source);
                        p
                    } else {
                        PathBuf::from(most_envied_source)
                    };

                    Some(ArchSmell::new_feature_envy(
                        path.clone(),
                        most_envied_path,
                        ratio,
                        internal_refs,
                        external_refs,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn init() {}
