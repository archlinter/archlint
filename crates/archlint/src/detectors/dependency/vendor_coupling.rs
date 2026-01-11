use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;
use crate::parser::FileSymbols;
use crate::utils::package::PackageUtils;

pub fn init() {}

#[detector(
    id = "vendor_coupling",
    name = "Vendor Coupling Detector",
    description = "Detects excessive coupling to third-party packages",
    category = DetectorCategory::ImportBased,
    default_enabled = false
)]
pub struct VendorCouplingDetector;

impl VendorCouplingDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn collect_package_usage(&self, ctx: &AnalysisContext) -> HashMap<String, Vec<PathBuf>> {
        let mut package_usage: HashMap<String, Vec<PathBuf>> = HashMap::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            if let Some(packages) = self.get_file_external_packages(ctx, path, symbols) {
                for package in packages {
                    package_usage.entry(package).or_default().push(path.clone());
                }
            }
        }

        package_usage
    }

    fn get_file_external_packages(
        &self,
        ctx: &AnalysisContext,
        path: &Path,
        symbols: &FileSymbols,
    ) -> Option<HashSet<String>> {
        let rule = ctx.get_rule_for_file("vendor_coupling", path)?;

        let ignore_packages: Vec<String> = rule
            .get_option("ignore_packages")
            .unwrap_or_else(|| vec!["react".to_string(), "lodash".to_string()]);

        let packages = symbols
            .imports
            .iter()
            .filter(|import| PackageUtils::is_external_package(&import.source))
            .map(|import| PackageUtils::extract_package_name(&import.source))
            .filter(|package| !PackageUtils::should_ignore_package(package, &ignore_packages))
            .collect();

        Some(packages)
    }
}

impl Detector for VendorCouplingDetector {
    fn name(&self) -> &'static str {
        "VendorCoupling"
    }

    fn explain(&self, smell: &ArchSmell) -> Explanation {
        let package = match &smell.smell_type {
            SmellType::VendorCoupling { package } => package.clone(),
            _ => "unknown".to_string(),
        };
        Explanation {
            problem: "Vendor Coupling".to_string(),
            reason: format!("Direct usage of third-party package `{}` in many files. This makes it difficult to replace the vendor library in the future.", package),
            risks: vec!["Vendor lock-in".to_string(), "Difficulty in upgrading or replacing the library".to_string()],
            recommendations: vec!["Create a wrapper or abstraction layer around the library".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        crate::define_report_section!("Vendor Coupling", smells, {
            crate::render_table!(
                vec!["Package", "Files", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let (package, count): (String, usize) = match &smell.smell_type {
                        SmellType::VendorCoupling { package } => {
                            (package.clone(), smell.files.len())
                        }
                        _ => ("unknown".to_string(), 0),
                    };
                    let pts = smell.score(severity_config);
                    vec![
                        format!("`{}`", package),
                        count.to_string(),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let package_usage = self.collect_package_usage(ctx);
        let rule = match ctx.get_rule("vendor_coupling") {
            Some(r) => r,
            None => return Vec::new(),
        };
        let max_files: usize = rule.get_option("max_files_per_package").unwrap_or(10);

        package_usage
            .into_iter()
            .filter(|(_, files)| files.len() >= max_files)
            .map(|(package, files)| {
                let mut smell = ArchSmell::new_vendor_coupling(package, files);
                smell.severity = rule.severity;
                smell
            })
            .collect()
    }
}
