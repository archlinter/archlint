use crate::Result;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub mod types;
pub use types::*;

impl TsConfig {
    /// Loads a tsconfig file from the given path, resolving `extends` recursively.
    pub fn load(path: &Path) -> Result<Self> {
        let mut visited = HashSet::new();
        Self::load_internal(path, &mut visited)
    }

    /// Internal implementation of `load` with cycle detection.
    /// Recursively follows `extends` fields and merges the configurations.
    fn load_internal(path: &Path, visited: &mut HashSet<PathBuf>) -> Result<Self> {
        let canonical_path = path.canonicalize().map_err(|e| {
            anyhow::anyhow!("Failed to canonicalize tsconfig path {:?}: {}", path, e)
        })?;
        if !visited.insert(canonical_path) {
            return Err(anyhow::anyhow!("Circular extends detected: {:?}", path).into());
        }

        let contents = fs::read_to_string(path)?;
        let mut config: TsConfig = json5::from_str(&contents)?;

        if let Some(extends) = &config.extends {
            let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
            let parent_path = Self::resolve_extends_path(base_dir, extends)?;
            let parent_config = Self::load_internal(&parent_path, visited)?;
            config = config.merge_with_parent(parent_config);
        }

        Ok(config)
    }

    /// Attempts to find and load a tsconfig file in the project root.
    /// If `explicit_path` is provided, it tries to load that specific file.
    /// Otherwise, it looks for the standard `tsconfig.json`.
    pub fn find_and_load(project_root: &Path, explicit_path: Option<&str>) -> Result<Option<Self>> {
        if let Some(p) = explicit_path {
            let path = project_root.join(p);
            if !path.exists() {
                return Err(anyhow::anyhow!("tsconfig path not found: {}", path.display()).into());
            }
            return Ok(Some(Self::load(&path)?));
        }

        // Look for standard tsconfig.json only
        let tsconfig_path = project_root.join("tsconfig.json");
        if tsconfig_path.exists() {
            return Ok(Some(Self::load(&tsconfig_path)?));
        }

        Ok(None)
    }

    /// Resolves a tsconfig path from node_modules by searching upwards.
    fn resolve_path_with_fallbacks(path: PathBuf) -> Option<PathBuf> {
        if path.is_file() {
            return Some(path);
        }
        let with_json = path.with_extension("json");
        if with_json.is_file() {
            return Some(with_json);
        }
        let tsconfig_json = path.join("tsconfig.json");
        if tsconfig_json.is_file() {
            return Some(tsconfig_json);
        }
        None
    }

    fn resolve_extends_path(base_dir: &Path, extends: &str) -> Result<PathBuf> {
        if extends.starts_with('.') {
            Self::resolve_path_with_fallbacks(base_dir.join(extends))
        } else if Path::new(extends).is_absolute() {
            Self::resolve_path_with_fallbacks(PathBuf::from(extends))
        } else {
            Self::resolve_node_modules_path(base_dir, extends)
        }
        .ok_or_else(|| anyhow::anyhow!("Could not resolve tsconfig extends: {}", extends).into())
    }

    fn resolve_node_modules_path(base_dir: &Path, specifier: &str) -> Option<PathBuf> {
        let mut current = base_dir.to_path_buf();
        loop {
            let node_modules = current.join("node_modules");
            if node_modules.is_dir() {
                if let Some(resolved) =
                    Self::resolve_path_with_fallbacks(node_modules.join(specifier))
                {
                    return Some(resolved);
                }
            }
            if !current.pop() {
                break;
            }
        }
        None
    }

    /// Merges a parent `TsConfig` into this one (the child config).
    /// This configuration's values take precedence over the parent's values.
    fn merge_with_parent(mut self, parent: TsConfig) -> Self {
        if let Some(parent_opts) = parent.compiler_options {
            self.compiler_options
                .get_or_insert_with(CompilerOptions::default)
                .merge(parent_opts);
        }

        let mut seen: HashSet<_> = self.exclude.iter().cloned().collect();
        for ex in parent.exclude {
            if seen.insert(ex.clone()) {
                self.exclude.push(ex);
            }
        }

        self
    }
}

#[cfg(test)]
mod tests;
