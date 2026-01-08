use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::parser::FileSymbols;
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
            .filter(|import| self.is_external_import(&import.source))
            .map(|import| self.extract_package_name(&import.source))
            .filter(|package| !self.should_ignore_package(package, &ignore_packages))
            .collect();

        Some(packages)
    }

    fn is_external_import(&self, source: &str) -> bool {
        !source.starts_with('.') && !source.starts_with('/')
    }

    fn should_ignore_package(&self, package: &str, ignore_packages: &[String]) -> bool {
        if self.is_builtin_package(package) {
            return true;
        }

        ignore_packages
            .iter()
            .any(|pattern_str| self.matches_ignore_pattern(package, pattern_str))
    }

    fn matches_ignore_pattern(&self, package: &str, pattern_str: &str) -> bool {
        if pattern_str.ends_with("/*") {
            let prefix = &pattern_str[..pattern_str.len() - 1];
            package.starts_with(prefix)
        } else if pattern_str.contains('*') {
            glob::Pattern::new(pattern_str)
                .map(|pattern| pattern.matches(package))
                .unwrap_or(false)
        } else {
            pattern_str == package
        }
    }

    fn is_builtin_package(&self, name: &str) -> bool {
        if name.starts_with("node:") {
            return true;
        }

        let builtins = [
            "assert",
            "async_hooks",
            "buffer",
            "child_process",
            "cluster",
            "console",
            "constants",
            "crypto",
            "dgram",
            "diagnostics_channel",
            "dns",
            "domain",
            "events",
            "fs",
            "http",
            "http2",
            "https",
            "inspector",
            "module",
            "net",
            "os",
            "path",
            "perf_hooks",
            "process",
            "punycode",
            "querystring",
            "readline",
            "repl",
            "stream",
            "string_decoder",
            "timers",
            "tls",
            "trace_events",
            "tty",
            "url",
            "util",
            "v8",
            "vm",
            "wasi",
            "worker_threads",
            "zlib",
        ];

        builtins.contains(&name)
    }

    fn extract_package_name(&self, source: &str) -> String {
        if source.starts_with('@') {
            source.split('/').take(2).collect::<Vec<_>>().join("/")
        } else {
            source.split('/').next().unwrap_or(source).to_string()
        }
    }
}
