use crate::config::Config;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use std::collections::HashMap;
use std::path::PathBuf;

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
        let mut package_usage: HashMap<String, Vec<PathBuf>> = HashMap::new();
        let thresholds = &ctx.config.thresholds.vendor_coupling;

        for (path, symbols) in &ctx.file_symbols {
            for import in &symbols.imports {
                if self.is_external_import(&import.source) {
                    let package = self.extract_package_name(&import.source);

                    if !self.should_ignore_package(&package, thresholds) {
                        let entries = package_usage.entry(package).or_default();
                        if !entries.contains(path) {
                            entries.push(path.clone());
                        }
                    }
                }
            }
        }

        package_usage
            .into_iter()
            .filter(|(_, files)| files.len() >= thresholds.max_files_per_package)
            .map(|(package, files)| ArchSmell::new_vendor_coupling(package, files))
            .collect()
    }
}

impl VendorCouplingDetector {
    fn is_external_import(&self, source: &str) -> bool {
        !source.starts_with('.') && !source.starts_with('/')
    }

    fn should_ignore_package(
        &self,
        package: &str,
        thresholds: &crate::config::VendorCouplingThresholds,
    ) -> bool {
        if self.is_builtin_package(package) {
            return true;
        }

        thresholds
            .ignore_packages
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
