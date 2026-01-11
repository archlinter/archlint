use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use crate::utils::package::PackageUtils;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub fn init() {}

#[detector(
    id = "hub_dependency",
    name = "Hub Dependency Detector",
    description = "Detects over-reliance on external packages",
    category = DetectorCategory::GraphBased,
    default_enabled = false
)]
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
    fn name(&self) -> &'static str {
        "HubDependency"
    }

    fn explain(&self, smell: &ArchSmell) -> Explanation {
        let count = smell.dependent_count().unwrap_or(0);

        let package = match &smell.smell_type {
            SmellType::HubDependency { package } => package.clone(),
            _ => "unknown".to_string(),
        };

        Explanation {
            problem: format!(
                "Hub Dependency: Too many files ({}) depend on package `{}`",
                count, package
            ),
            reason: format!(
                "The package `{}` is used by {} different files in the project. This makes it a critical dependency that is hard to replace or update.",
                package, count
            ),
            risks: vec![
                "Difficulty in upgrading the package due to widespread usage".to_string(),
                "High impact if the package becomes deprecated or has security issues".to_string(),
                "Tightly coupled to a specific external library's API".to_string(),
            ],
            recommendations: vec![
                "Create a wrapper/abstraction around the package to isolate its usage".to_string(),
                "Evaluate if the dependency is truly necessary in all those files".to_string(),
                "Use dependency injection to provide the functionality if possible".to_string(),
            ],
        }
    }

    fn render_markdown(
        &self,
        hub_dependencies: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        crate::define_report_section!("Hub Dependencies", hub_dependencies, {
            crate::render_table!(
                vec!["Package", "Dependants", "pts"],
                hub_dependencies,
                |&(smell, _): &&SmellWithExplanation| {
                    if let SmellType::HubDependency { package } = &smell.smell_type {
                        let count = smell.dependent_count().unwrap_or(0);
                        let pts = smell.score(severity_config);

                        vec![
                            format!("`{}`", package),
                            format!("{} files", count),
                            format!("{} pts", pts),
                        ]
                    } else {
                        vec!["-".into(); 3]
                    }
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let rule = match ctx.get_rule("hub_dependency") {
            Some(r) => r,
            None => return Vec::new(),
        };

        // Default threshold of 20 dependants is a heuristic for "hub" status in medium-sized projects.
        let min_dependants: usize = rule.get_option("min_dependants").unwrap_or(20);
        // Standard libraries and frameworks are ignored by default as they are expected hubs.
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
