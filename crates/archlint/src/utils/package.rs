pub struct PackageUtils;

impl PackageUtils {
    pub fn extract_package_name(source: &str) -> String {
        // "lodash/get" -> "lodash"
        // "@scope/pkg/utils" -> "@scope/pkg"
        if source.starts_with('@') {
            source.split('/').take(2).collect::<Vec<_>>().join("/")
        } else {
            source.split('/').next().unwrap_or(source).to_string()
        }
    }

    pub fn is_external_package(source: &str) -> bool {
        !source.starts_with('.') && !source.starts_with('/')
    }

    pub fn matches_ignore_pattern(pkg: &str, pattern_str: &str) -> bool {
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

    pub fn is_builtin_package(name: &str) -> bool {
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

    pub fn should_ignore_package(package: &str, ignore_packages: &[String]) -> bool {
        if Self::is_builtin_package(package) {
            return true;
        }

        ignore_packages
            .iter()
            .any(|pattern_str| Self::matches_ignore_pattern(package, pattern_str))
    }
}
