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
        let mut entry_points = HashSet::new();
        let mut glob_patterns = Vec::new();

        let path_regex =
            Regex::new(r"(\S*[a-zA-Z0-9_\-\./]+\.(?:ts|js|tsx|jsx)|\bdist/\S+\b|\bbuild/\S+\b)")?;
        let glob_pattern_regex =
            Regex::new(r"([^\s]*(?:/\*\*)?(?:/\*\*)?(?:/\*)?[^\s]*\.(?:ts|js|tsx|jsx|mjs|cjs))")?;

        let walker = WalkBuilder::new(root)
            .standard_filters(true)
            .hidden(false)
            .build();

        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if path.file_name().is_some_and(|n| n == "package.json") {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(json) = serde_json::from_str::<Value>(&content) {
                        if let Some(scripts) = json.get("scripts").and_then(|s| s.as_object()) {
                            let package_dir = path.parent().unwrap_or(root);

                            for value in scripts.values() {
                                if let Some(script_str) = value.as_str() {
                                    // 1. Find script entry points
                                    for cap in path_regex.captures_iter(script_str) {
                                        let matched_path = &cap[1];
                                        let mut candidates = vec![
                                            package_dir.join(matched_path),
                                            PathBuf::from(matched_path),
                                        ];

                                        if matched_path.contains("dist/")
                                            || matched_path.contains("build/")
                                        {
                                            let src_path = matched_path
                                                .replace("dist/", "src/")
                                                .replace("build/", "src/");

                                            if src_path.ends_with(".js") {
                                                candidates.push(
                                                    package_dir
                                                        .join(src_path.replace(".js", ".ts")),
                                                );
                                            } else {
                                                candidates.push(package_dir.join(&src_path));
                                                candidates.push(
                                                    package_dir.join(format!("{}.ts", src_path)),
                                                );
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

                                    // 2. Find dynamic load patterns
                                    for cap in glob_pattern_regex.captures_iter(script_str) {
                                        let pattern = &cap[1];
                                        if !pattern.contains('*') {
                                            continue;
                                        }

                                        let src_pattern = pattern
                                            .replace("build/", "src/")
                                            .replace("dist/", "src/")
                                            .replace(".js", ".ts")
                                            .replace(".jsx", ".tsx")
                                            .replace(".mjs", ".ts")
                                            .replace(".cjs", ".ts");

                                        if src_pattern.contains('*')
                                            && !glob_patterns.contains(&src_pattern)
                                        {
                                            glob_patterns.push(src_pattern);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(PackageConfig {
            entry_points,
            dynamic_load_patterns: glob_patterns,
        })
    }
}
