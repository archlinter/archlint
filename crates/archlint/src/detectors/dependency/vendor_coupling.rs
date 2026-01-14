use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use crate::parser::FileSymbols;
use crate::utils::package::PackageUtils;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(SmellType::VendorCoupling, default_enabled = false)]
pub struct VendorCouplingDetector;

impl VendorCouplingDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn collect_package_usage(&self, ctx: &AnalysisContext) -> HashMap<String, Vec<PathBuf>> {
        let mut package_usage: HashMap<String, HashSet<PathBuf>> = HashMap::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            if let Some(packages) = self.get_file_external_packages(ctx, path, symbols) {
                for package in packages {
                    package_usage
                        .entry(package)
                        .or_default()
                        .insert(path.clone());
                }
            }
        }

        package_usage
            .into_iter()
            .map(|(pkg, files)| (pkg, files.into_iter().collect()))
            .collect()
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
    crate::impl_detector_report!(
        explain: smell => (
            problem: {
                let package = if let crate::detectors::SmellType::VendorCoupling { package } = &smell.smell_type {
                    package.as_str()
                } else {
                    "unknown"
                };
                format!("Vendor Coupling: `{}`", package)
            },
            reason: "Direct usage of a third-party package in many files. This makes it difficult to replace the vendor library in the future.",
            risks: [
                "Vendor lock-in",
                "Difficulty in upgrading or replacing the library"
            ],
            recommendations: [
                "Create a wrapper or abstraction layer around the library"
            ]
        ),
        table: {
            title: "Vendor Coupling",
            columns: ["Package", "Files", "pts"],
            row: VendorCoupling { package } (smell, location, pts) => [
                package,
                smell.files.len(),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let package_usage = self.collect_package_usage(ctx);
        if package_usage.is_empty() {
            return Vec::new();
        }

        let rule = ctx.get_rule("vendor_coupling");
        let max_files: usize = rule
            .as_ref()
            .and_then(|r| r.get_option("max_files_per_package"))
            .unwrap_or(10);

        let severity = rule
            .as_ref()
            .map(|r| r.severity)
            .unwrap_or(crate::detectors::Severity::Medium);

        package_usage
            .into_iter()
            .filter(|(_, files)| files.len() > max_files)
            .map(|(package, files)| {
                let mut smell = ArchSmell::new_vendor_coupling(package, files);
                smell.severity = severity;
                smell
            })
            .collect()
    }
}
