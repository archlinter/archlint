use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub struct TsConfigResolver;

impl TsConfigResolver {
    /// Parses a package specifier into a package name and an optional subpath.
    pub fn parse_package_specifier(specifier: &str) -> (String, Option<&str>) {
        if specifier.starts_with('@') {
            let parts: Vec<&str> = specifier.splitn(3, '/').collect();
            if parts.len() >= 2 {
                let pkg = format!("{}/{}", parts[0], parts[1]);
                let sub = if parts.len() == 3 {
                    Some(parts[2])
                } else {
                    None
                };
                return (pkg, sub);
            }
        }

        let parts: Vec<&str> = specifier.splitn(2, '/').collect();
        let pkg = parts[0].to_string();
        let sub = if parts.len() == 2 {
            Some(parts[1])
        } else {
            None
        };
        (pkg, sub)
    }

    /// Attempts to resolve a tsconfig path through the "tsconfig" field in package.json.
    pub fn resolve_via_package_json_field(pkg_dir: &Path) -> Option<PathBuf> {
        let pkg_json_path = pkg_dir.join("package.json");
        let content = fs::read_to_string(pkg_json_path).ok()?;
        let pkg_json: Value = serde_json::from_str(&content).ok()?;

        pkg_json
            .get("tsconfig")
            .and_then(|v| v.as_str())
            .map(|s| pkg_dir.join(s))
    }

    /// Attempts to resolve a tsconfig path through the "exports" field in package.json.
    pub fn resolve_via_exports(pkg_dir: &Path, subpath: &str) -> Option<PathBuf> {
        let pkg_json_path = pkg_dir.join("package.json");
        let content = fs::read_to_string(pkg_json_path).ok()?;
        let pkg_json: Value = serde_json::from_str(&content).ok()?;

        let exports = pkg_json.get("exports")?.as_object()?;
        let sub_with_dot = format!("./{}", subpath);

        let target = exports.get(&sub_with_dot)?;

        // Handle both string and conditional export objects
        let target_str = if let Some(s) = target.as_str() {
            Some(s)
        } else if let Some(obj) = target.as_object() {
            // Try common conditions: types, default, require, import
            obj.get("types")
                .or_else(|| obj.get("default"))
                .or_else(|| obj.get("require"))
                .or_else(|| obj.get("import"))
                .and_then(|v| v.as_str())
        } else {
            None
        }?;

        Some(pkg_dir.join(target_str))
    }
}
