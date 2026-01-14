use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(
    smell_type = SmellType::FeatureEnvy,
    name = "Feature Envy Detector",
    description = "Detects modules that use more external symbols than internal ones",
    category = DetectorCategory::Global,
    default_enabled = false
)]
pub struct FeatureEnvyDetector;

impl FeatureEnvyDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn analyze_file_for_envy(
        path: &Path,
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
        path: &Path,
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

impl Detector for FeatureEnvyDetector {
    crate::impl_detector_report!(
        name: "FeatureEnvy",
        explain: smell => (
            problem: format!("Feature Envy (ratio: {:.1}x)", smell.envy_ratio().unwrap_or(0.0)),
            reason: "This module uses more symbols from another module than its own. It seems more interested in the details of another module than its own functionality.",
            risks: [
                "Violation of encapsulation and data hiding",
                "Tight coupling between the two modules",
                "Increased difficulty in testing and refactoring"
            ],
            recommendations: [
                "Move the code that uses the external symbols into the envied module",
                "Extract a new module that contains the logic and data together",
                "Pass only necessary data as arguments instead of accessing many properties"
            ]
        ),
        table: {
            title: "Feature Envy",
            columns: ["File", "Envied Module", "Ratio", "pts"],
            row: FeatureEnvy { most_envied_module } (smell, location, pts) => [
                location,
                crate::explain::ExplainEngine::format_file_path(most_envied_module),
                format!("{:.1}x", smell.envy_ratio().unwrap_or(0.0)),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        ctx.file_symbols
            .as_ref()
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
