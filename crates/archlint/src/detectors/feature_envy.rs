use crate::config::Config;
use crate::detectors::DetectorCategory;
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
            category: DetectorCategory::Global,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(FeatureEnvyDetector)
    }
}

inventory::submit! {
    &FeatureEnvyDetectorFactory as &dyn DetectorFactory
}

impl Detector for FeatureEnvyDetector {
    fn name(&self) -> &'static str {
        "FeatureEnvy"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        ctx.file_symbols
            .iter()
            .filter_map(|(path, symbols)| {
                let rule = ctx.get_rule_for_file("feature_envy", path)?;

                let ratio_threshold: f64 = rule.get_option("ratio").unwrap_or(3.0);

                let mut smell =
                    Self::analyze_file_for_envy(path.as_path(), symbols, ratio_threshold)?;
                smell.severity = rule.severity;
                Some(smell)
            })
            .collect()
    }
}

impl FeatureEnvyDetector {
    fn analyze_file_for_envy(
        path: &std::path::Path,
        symbols: &crate::parser::FileSymbols,
        ratio_threshold: f64,
    ) -> Option<ArchSmell> {
        let internal_refs = Self::count_internal_refs(symbols);
        let (external_refs, source_usages) = Self::count_external_refs(symbols);
        let ratio = external_refs as f64 / (internal_refs as f64 + 1.0);

        if ratio >= ratio_threshold && external_refs > 0 {
            let most_envied_path = Self::find_most_envied_path(path, source_usages)?;
            Some(ArchSmell::new_feature_envy(
                path.to_path_buf(),
                most_envied_path,
                ratio,
                internal_refs,
                external_refs,
            ))
        } else {
            None
        }
    }

    fn count_internal_refs(symbols: &crate::parser::FileSymbols) -> usize {
        let def_refs = symbols
            .local_definitions
            .iter()
            .filter(|def| symbols.local_usages.contains(def.as_str()))
            .count();
        let exp_refs = symbols
            .exports
            .iter()
            .filter(|exp| symbols.local_usages.contains(exp.name.as_str()))
            .count();
        def_refs + exp_refs
    }

    fn count_external_refs(
        symbols: &crate::parser::FileSymbols,
    ) -> (usize, HashMap<String, usize>) {
        let mut external_refs = 0;
        let mut source_usages: HashMap<String, usize> = HashMap::new();

        for import in &symbols.imports {
            let name_to_check = import.alias.as_ref().unwrap_or(&import.name);
            if symbols.local_usages.contains(name_to_check) {
                external_refs += 1;
                *source_usages.entry(import.source.to_string()).or_insert(0) += 1;
            }
        }

        (external_refs, source_usages)
    }

    fn find_most_envied_path(
        path: &std::path::Path,
        source_usages: HashMap<String, usize>,
    ) -> Option<PathBuf> {
        let (most_envied_source, _) = source_usages.into_iter().max_by_key(|&(_, count)| count)?;

        if most_envied_source.starts_with('.') {
            let mut p = path.parent()?.to_path_buf();
            p.push(most_envied_source);
            Some(p)
        } else {
            Some(PathBuf::from(most_envied_source))
        }
    }
}

pub fn init() {}
