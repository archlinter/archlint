use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use std::collections::HashMap;
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

impl HubDependencyDetector {
    fn extract_package_name(&self, source: &str) -> String {
        // "lodash/get" -> "lodash"
        // "@scope/pkg/utils" -> "@scope/pkg"
        if source.starts_with('@') {
            source.splitn(3, '/').take(2).collect::<Vec<_>>().join("/")
        } else {
            source.split('/').next().unwrap_or(source).to_string()
        }
    }

    fn is_external_package(&self, source: &str) -> bool {
        !source.starts_with('.') && !source.starts_with('/')
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
}

impl Detector for HubDependencyDetector {
    fn name(&self) -> &'static str {
        "HubDependency"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let thresholds = &ctx.config.thresholds.hub_dependency;
        let package_usage = Self::collect_package_usage(ctx, self);
        Self::filter_hub_packages(package_usage, thresholds)
    }
}

impl HubDependencyDetector {
    fn collect_package_usage(
        ctx: &AnalysisContext,
        detector: &HubDependencyDetector,
    ) -> HashMap<String, Vec<PathBuf>> {
        let mut package_usage: HashMap<String, Vec<PathBuf>> = HashMap::new();

        for (file, symbols) in ctx.file_symbols.as_ref() {
            for import in &symbols.imports {
                if detector.is_external_package(&import.source) {
                    let package = detector.extract_package_name(&import.source);

                    if !detector.is_builtin_package(&package) {
                        let files = package_usage.entry(package).or_default();
                        if !files.contains(file) {
                            files.push(file.clone());
                        }
                    }
                }
            }
        }

        package_usage
    }

    fn filter_hub_packages(
        package_usage: HashMap<String, Vec<PathBuf>>,
        thresholds: &crate::config::HubDependencyThresholds,
    ) -> Vec<ArchSmell> {
        package_usage
            .into_iter()
            .filter(|(pkg, files)| {
                !Self::is_ignored_package(pkg, thresholds)
                    && files.len() >= thresholds.min_dependants
            })
            .map(|(pkg, files)| ArchSmell::new_hub_dependency(pkg, files))
            .collect()
    }

    fn is_ignored_package(pkg: &str, thresholds: &crate::config::HubDependencyThresholds) -> bool {
        thresholds
            .ignore_packages
            .iter()
            .any(|pattern_str| Self::matches_ignore_pattern(pkg, pattern_str))
    }

    fn matches_ignore_pattern(pkg: &str, pattern_str: &str) -> bool {
        if pattern_str.ends_with("/*") {
            let prefix = &pattern_str[..pattern_str.len() - 1];
            pkg.starts_with(prefix)
        } else if pattern_str.contains('*') {
            glob::Pattern::new(pattern_str)
                .map(|pattern| pattern.matches(pkg))
                .unwrap_or(false)
        } else {
            pattern_str == pkg
        }
    }
}

pub fn init() {}
