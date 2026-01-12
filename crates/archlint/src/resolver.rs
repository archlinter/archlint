use crate::config::Config;
use crate::{AnalysisError, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct PathResolver {
    root: PathBuf,
    aliases: HashMap<String, String>,
}

impl PathResolver {
    pub fn new<P: AsRef<Path>>(root: P, config: &Config) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            aliases: config.aliases.clone(),
        }
    }

    pub fn resolve(&self, import_path: &str, from_file: &Path) -> Result<Option<PathBuf>> {
        if import_path.starts_with('.') {
            // Relative import
            return self.resolve_relative(import_path, from_file);
        }

        // Try alias resolution
        if let Some(resolved) = self.resolve_alias(import_path)? {
            return Ok(Some(resolved));
        }

        // Try absolute resolution from root or root/src (baseUrl fallback)
        let root_candidate = self.root.join(import_path);
        if let Some(resolved) = self.try_resolve_with_extensions(&root_candidate)? {
            return Ok(Some(resolved));
        }

        let src_candidate = self.root.join("src").join(import_path);
        if let Some(resolved) = self.try_resolve_with_extensions(&src_candidate)? {
            return Ok(Some(resolved));
        }

        Ok(None)
    }

    fn resolve_relative(&self, import_path: &str, from_file: &Path) -> Result<Option<PathBuf>> {
        let from_dir = from_file.parent().ok_or_else(|| {
            AnalysisError::PathResolution(format!("Invalid file path: {:?}", from_file))
        })?;

        let candidate = from_dir.join(import_path);
        self.try_resolve_with_extensions(&candidate)
    }

    fn resolve_alias(&self, import_path: &str) -> Result<Option<PathBuf>> {
        for (alias_prefix, actual_prefix) in &self.aliases {
            if import_path.starts_with(alias_prefix.trim_end_matches('*')) {
                let relative_path = import_path.replacen(
                    alias_prefix.trim_end_matches('*'),
                    actual_prefix.trim_end_matches('*'),
                    1,
                );
                let candidate = self.root.join(&relative_path);
                return self.try_resolve_with_extensions(&candidate);
            }
        }

        Ok(None)
    }

    fn try_resolve_with_extensions(&self, base: &Path) -> Result<Option<PathBuf>> {
        // Try exact path first
        if base.exists() && base.is_file() {
            return Ok(Some(
                base.canonicalize().unwrap_or_else(|_| base.to_path_buf()),
            ));
        }

        let base_str = base.to_string_lossy();

        // Special case for TS ESM: if importing .js but only .ts exists
        if base_str.ends_with(".js") {
            let ts_base = PathBuf::from(base_str.replace(".js", ".ts"));
            if ts_base.exists() && ts_base.is_file() {
                return Ok(Some(ts_base.canonicalize().unwrap_or(ts_base)));
            }
        }
        if base_str.ends_with(".jsx") {
            let tsx_base = PathBuf::from(base_str.replace(".jsx", ".tsx"));
            if tsx_base.exists() && tsx_base.is_file() {
                return Ok(Some(tsx_base.canonicalize().unwrap_or(tsx_base)));
            }
        }

        let extensions = ["ts", "tsx", "js", "jsx", "mjs", "cjs"];

        // Try adding extensions (don't use with_extension as it replaces existing ones like .service)
        for ext in &extensions {
            let with_ext = PathBuf::from(format!("{}.{}", base_str, ext));
            if with_ext.exists() && with_ext.is_file() {
                return Ok(Some(with_ext.canonicalize().unwrap_or(with_ext)));
            }
        }

        // Try index files
        if base.is_dir() {
            for ext in &extensions {
                let index = base.join(format!("index.{}", ext));
                if index.exists() && index.is_file() {
                    return Ok(Some(index.canonicalize().unwrap_or(index)));
                }
            }
        }

        Ok(None)
    }
}
