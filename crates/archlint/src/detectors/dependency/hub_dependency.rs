use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use crate::utils::package::PackageUtils;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(SmellType::HubDependency, default_enabled = false)]
pub struct HubDependencyDetector;

impl HubDependencyDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

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

impl Detector for HubDependencyDetector {
    crate::impl_detector_report!(
        explain: smell => (
            problem: {
                let count = smell.dependent_count().unwrap_or(0);
                let package = if let crate::detectors::SmellType::HubDependency { package } = &smell.smell_type {
                    package.as_str()
                } else {
                    "unknown"
                };
                format!("Hub Dependency: Too many files ({}) depend on package `{}`", count, package)
            },
            reason: "The package is used by many different files in the project. This makes it a critical dependency that is hard to replace or update.",
            risks: [
                "Difficulty in upgrading the package due to widespread usage",
                "High impact if the package becomes deprecated or has security issues",
                "Tightly coupled to a specific external library's API"
            ],
            recommendations: [
                "Create a wrapper/abstraction around the package to isolate its usage",
                "Evaluate if the dependency is truly necessary in all those files",
                "Use dependency injection to provide the functionality if possible"
            ]
        ),
        table: {
            title: "Hub Dependencies",
            columns: ["Package", "Dependents", "pts"],
            row: HubDependency { package } (smell, location, pts) => [
                package,
                format!("{} files", smell.dependent_count().unwrap_or(0)),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let rule = match ctx.get_rule("hub_dependency") {
            Some(r) => r,
            None => return Vec::new(),
        };

        let min_dependents: usize = rule
            .get_option("min_dependents")
            .or_else(|| rule.get_option("min_dependants"))
            .unwrap_or(20);
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
                    && files.len() >= min_dependents
            })
            .map(|(pkg, files)| {
                let mut smell = ArchSmell::new_hub_dependency(pkg, files);
                // Rule severity intentionally overrides the count-based default from factory
                smell.severity = rule.severity;
                smell
            })
            .collect()
    }
}
