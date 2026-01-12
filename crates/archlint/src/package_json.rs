use crate::Result;
use ignore::WalkBuilder;
use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct PackageJsonParser;

#[derive(Debug, Clone)]
pub struct PackageConfig {
    pub entry_points: HashSet<PathBuf>,
    pub dynamic_load_patterns: Vec<String>,
}

impl PackageJsonParser {
    pub fn parse<P: AsRef<Path>>(root: P) -> Result<PackageConfig> {
        let root = root.as_ref();
        let mut config = PackageConfig {
            entry_points: HashSet::new(),
            dynamic_load_patterns: Vec::new(),
        };

        let path_regex =
            Regex::new(r"(\S*[a-zA-Z0-9_\-\./]+\.(?:ts|js|tsx|jsx)|\bdist/\S+\b|\bbuild/\S+\b)")?;
        let glob_regex =
            Regex::new(r"([^\s]*(?:/\*\*)?(?:/\*\*)?(?:/\*)?[^\s]*\.(?:ts|js|tsx|jsx|mjs|cjs))")?;

        let walker = WalkBuilder::new(root)
            .standard_filters(true)
            .hidden(false)
            .build();

        for entry in walker.flatten() {
            let path = entry.path();
            if path.file_name().is_some_and(|n| n == "package.json") {
                let _ =
                    Self::process_package_json(path, root, &mut config, &path_regex, &glob_regex);
            }
        }

        Ok(config)
    }

    fn process_package_json(
        path: &Path,
        root: &Path,
        config: &mut PackageConfig,
        path_regex: &Regex,
        glob_regex: &Regex,
    ) -> Result<()> {
        let content = fs::read_to_string(path)?;
        let json: Value = serde_json::from_str(&content)?;
        let package_dir = path.parent().unwrap_or(root);

        // 1. Process main, module, and browser fields
        for field in ["main", "module", "browser"] {
            if let Some(val) = json.get(field).and_then(|v| v.as_str()) {
                Self::find_entry_point_candidates(val, package_dir, &mut config.entry_points);
            }
        }

        // 2. Process exports field
        if let Some(exports) = json.get("exports") {
            Self::process_exports_value(exports, package_dir, config);
        }

        // 3. Process scripts
        if let Some(scripts) = json.get("scripts").and_then(|s| s.as_object()) {
            for value in scripts.values() {
                if let Some(script_str) = value.as_str() {
                    Self::parse_script(script_str, package_dir, config, path_regex, glob_regex);
                }
            }
        }
        Ok(())
    }

    fn process_exports_value(value: &Value, package_dir: &Path, config: &mut PackageConfig) {
        match value {
            Value::String(s) => {
                Self::find_entry_point_candidates(s, package_dir, &mut config.entry_points);
            }
            Value::Object(map) => {
                for v in map.values() {
                    Self::process_exports_value(v, package_dir, config);
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    Self::process_exports_value(v, package_dir, config);
                }
            }
            _ => {}
        }
    }

    fn parse_script(
        script: &str,
        package_dir: &Path,
        config: &mut PackageConfig,
        path_regex: &Regex,
        glob_regex: &Regex,
    ) {
        // 1. Find script entry points
        for cap in path_regex.captures_iter(script) {
            Self::find_entry_point_candidates(&cap[1], package_dir, &mut config.entry_points);
        }

        // 2. Find dynamic load patterns
        for cap in glob_regex.captures_iter(script) {
            Self::add_dynamic_load_pattern(&cap[1], &mut config.dynamic_load_patterns);
        }
    }

    fn find_entry_point_candidates(
        matched_path: &str,
        package_dir: &Path,
        entry_points: &mut HashSet<PathBuf>,
    ) {
        let mut candidates = vec![package_dir.join(matched_path), PathBuf::from(matched_path)];

        if matched_path.contains("dist/") || matched_path.contains("build/") {
            let src_path = matched_path
                .replace("dist/", "src/")
                .replace("build/", "src/");

            if src_path.ends_with(".js") {
                candidates.push(package_dir.join(src_path.replace(".js", ".ts")));
            } else {
                candidates.push(package_dir.join(&src_path));
                candidates.push(package_dir.join(format!("{}.ts", src_path)));
            }
        }

        for cand in candidates {
            if let Ok(canonical) = cand.canonicalize() {
                if canonical.is_file() {
                    entry_points.insert(canonical);
                }
            }
        }
    }

    fn add_dynamic_load_pattern(pattern: &str, glob_patterns: &mut Vec<String>) {
        if !pattern.contains('*') {
            return;
        }

        let src_pattern = pattern
            .replace("build/", "src/")
            .replace("dist/", "src/")
            .replace(".js", ".ts")
            .replace(".jsx", ".tsx")
            .replace(".mjs", ".ts")
            .replace(".cjs", ".ts");

        if src_pattern.contains('*') && !glob_patterns.contains(&src_pattern) {
            glob_patterns.push(src_pattern);
        }
    }
}
