use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::parser::FileSymbols;
use crate::utils::package::PackageUtils;
use inventory;

pub fn init() {}

pub struct VendorCouplingDetector;

pub struct VendorCouplingDetectorFactory;

impl DetectorFactory for VendorCouplingDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "vendor_coupling",
            name: "Vendor Coupling Detector",
            description: "Detects excessive coupling to third-party packages",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::ImportBased,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(VendorCouplingDetector)
    }
}

inventory::submit! {
    &VendorCouplingDetectorFactory as &dyn DetectorFactory
}

impl Detector for VendorCouplingDetector {
    fn name(&self) -> &'static str {
        "VendorCoupling"
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

impl VendorCouplingDetector {
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
