use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::utils::package::PackageUtils;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub struct HubDependencyDetector;

pub struct HubDependencyDetectorFactory;

impl DetectorFactory for HubDependencyDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "hub_dependency",
            name: "Hub Dependency Detector",
            description: "Detects over-reliance on external packages",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::GraphBased,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(HubDependencyDetector)
    }
}

inventory::submit! {
    &HubDependencyDetectorFactory as &dyn DetectorFactory
}

impl Detector for HubDependencyDetector {
    fn name(&self) -> &'static str {
        "HubDependency"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let rule = match ctx.get_rule("hub_dependency") {
            Some(r) => r,
            None => return Vec::new(),
        };

        let min_dependants: usize = rule.get_option("min_dependants").unwrap_or(20);
        let ignore_packages: Vec<String> =
            rule.get_option("ignore_packages").unwrap_or_else(|| {
                vec![
                    "react".to_string(),
                    "lodash".to_string(),
                    "typescript".to_string(),
                ]
            });

        let package_usage = Self::collect_package_usage(ctx);

        package_usage
            .into_iter()
            .filter(|(pkg, files)| {
                !PackageUtils::should_ignore_package(pkg, &ignore_packages)
                    && files.len() >= min_dependants
            })
            .map(|(pkg, files)| {
                let mut smell = ArchSmell::new_hub_dependency(pkg, files);
                smell.severity = rule.severity;
                smell
            })
            .collect()
    }
}

impl HubDependencyDetector {
    fn collect_package_usage(ctx: &AnalysisContext) -> HashMap<String, Vec<PathBuf>> {
        let mut package_usage: HashMap<String, HashSet<PathBuf>> = HashMap::new();

        for (file, symbols) in ctx.file_symbols.as_ref() {
            for import in &symbols.imports {
                if PackageUtils::is_external_package(&import.source) {
                    let package = PackageUtils::extract_package_name(&import.source);

                    if !PackageUtils::is_builtin_package(&package) {
                        package_usage
                            .entry(package)
                            .or_default()
                            .insert(file.clone());
                    }
                }
            }
        }

        package_usage
            .into_iter()
            .map(|(pkg, files)| (pkg, files.into_iter().collect()))
            .collect()
    }
}

pub fn init() {}
