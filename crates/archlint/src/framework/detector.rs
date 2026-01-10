use super::preset_loader::PresetLoader;
use super::Framework;
use ignore::WalkBuilder;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct FrameworkDetector;

impl FrameworkDetector {
    pub fn detect<P: AsRef<Path>>(root: P) -> Vec<Framework> {
        let root = root.as_ref();
        let mut frameworks = HashSet::new();

        // 1. Detect by config files (fast)
        Self::detect_by_files(root, &mut frameworks);

        // 2. Detect by package.json (requires walking)
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
                        Self::detect_from_json(&json, &mut frameworks);
                    }
                }
            }
        }

        frameworks.into_iter().collect()
    }

    fn detect_by_files(root: &Path, frameworks: &mut HashSet<Framework>) {
        for name in PresetLoader::get_all_builtin_names() {
            if let Some(preset) = PresetLoader::get_builtin_yaml(name) {
                if let Some(files_rules) = preset.detect.files {
                    if Self::matches_rules(&files_rules, |p| root.join(p).exists()) {
                        if let Some(fw) = Self::map_name_to_framework(&preset.name) {
                            frameworks.insert(fw);
                        }
                    }
                }
            }
        }
    }

    fn detect_from_json(json: &Value, frameworks: &mut HashSet<Framework>) {
        let dependencies = ["dependencies", "devDependencies", "peerDependencies"];

        for name in PresetLoader::get_all_builtin_names() {
            if let Some(preset) = PresetLoader::get_builtin_yaml(name) {
                if let Some(pkg_rules) = preset.detect.packages {
                    if Self::matches_rules(&pkg_rules, |p| {
                        Self::has_package(json, p, &dependencies)
                    }) {
                        if let Some(fw) = Self::map_name_to_framework(&preset.name) {
                            frameworks.insert(fw);
                        }
                    }
                }
            }
        }
    }

    fn matches_rules<F>(rules: &crate::framework::preset_types::MatchRules, mut check: F) -> bool
    where
        F: FnMut(&str) -> bool,
    {
        if let Some(any_of) = &rules.any_of {
            if any_of.iter().any(|p| check(p)) {
                return true;
            }
        }
        if let Some(all_of) = &rules.all_of {
            if !all_of.is_empty() && all_of.iter().all(|p| check(p)) {
                return true;
            }
        }
        false
    }

    fn has_package(json: &Value, name: &str, dep_types: &[&str]) -> bool {
        for dep_type in dep_types {
            if let Some(deps) = json.get(*dep_type).and_then(|d| d.as_object()) {
                if deps.contains_key(name) {
                    return true;
                }
                // Support simple glob for package name (e.g. "@nestjs/*")
                if let Some(prefix) = name.strip_suffix('*') {
                    if deps.keys().any(|k| k.starts_with(prefix)) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn map_name_to_framework(name: &str) -> Option<Framework> {
        match name.to_lowercase().as_str() {
            "nestjs" => Some(Framework::NestJS),
            "nextjs" | "next.js" => Some(Framework::NextJS),
            "react" => Some(Framework::React),
            "oclif" => Some(Framework::Oclif),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_detect_nestjs() {
        let json = json!({
            "dependencies": {
                "@nestjs/core": "^10.0.0"
            }
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.contains(&Framework::NestJS));
    }

    #[test]
    fn test_detect_nextjs() {
        let json = json!({
            "dependencies": {
                "next": "latest"
            }
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.contains(&Framework::NextJS));
    }

    #[test]
    fn test_detect_multiple() {
        let json = json!({
            "dependencies": {
                "next": "latest",
                "@nestjs/common": "latest"
            }
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.contains(&Framework::NextJS));
        assert!(frameworks.contains(&Framework::NestJS));
    }

    #[test]
    fn test_detect_dev_deps() {
        let json = json!({
            "devDependencies": {
                "react": "latest"
            }
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.contains(&Framework::React));
    }

    #[test]
    fn test_detect_peer_deps() {
        let json = json!({
            "peerDependencies": {
                "@nestjs/common": "latest"
            }
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.contains(&Framework::NestJS));
    }

    #[test]
    fn test_detect_no_deps() {
        let json = json!({
            "name": "no-deps"
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.is_empty());
    }
}
