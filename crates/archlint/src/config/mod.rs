use crate::tsconfig::{CompilerOptions, TsConfig};
use crate::Result;
use std::collections::hash_map::Entry;
use std::fs;
use std::path::{Path, PathBuf};

pub mod types;
pub use types::*;

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn load_or_default(path: Option<&Path>, project_root: Option<&Path>) -> Result<Self> {
        let mut config = if let Some(p) = path {
            Self::load(p)?
        } else {
            let filenames = [
                ".archlint.yaml",
                ".archlint.yml",
                "archlint.yaml",
                "archlint.yml",
            ];

            let mut found_config = None;
            for filename in filenames {
                let p = project_root
                    .map(|root| root.join(filename))
                    .unwrap_or_else(|| PathBuf::from(filename));

                if p.exists() {
                    found_config = Some(Self::load(p)?);
                    break;
                }
            }

            found_config.unwrap_or_else(Self::default)
        };

        if let Some(tsconfig_opt) = &config.tsconfig {
            if !matches!(tsconfig_opt, TsConfigConfig::Boolean(false)) {
                if let Some(root) = project_root {
                    config.enrich_from_tsconfig(root)?;
                }
            }
        }

        Ok(config)
    }

    /// Enriches the current configuration with settings from a TypeScript configuration file.
    /// This includes loading path aliases, adding `outDir` to ignores, and including `exclude` patterns.
    pub fn enrich_from_tsconfig(&mut self, project_root: &Path) -> Result<()> {
        let explicit_path = match &self.tsconfig {
            Some(TsConfigConfig::Path(p)) => Some(p.as_str()),
            _ => None,
        };

        let tsconfig = match TsConfig::find_and_load(project_root, explicit_path) {
            Ok(config) => config,
            Err(e) => {
                log::warn!("Failed to load tsconfig.json: {}. Path aliases and excludes from tsconfig will not be applied.", e);
                None
            }
        };

        if let Some(tsconfig) = tsconfig {
            if let Some(opts) = tsconfig.compiler_options {
                self.apply_tsconfig_aliases(&opts);
                self.apply_tsconfig_out_dir(&opts);
            }
            self.apply_tsconfig_excludes(tsconfig.exclude);
        }

        Ok(())
    }

    /// Applies path aliases from a `CompilerOptions` to the current configuration.
    /// Aliases already present in the configuration take precedence.
    fn apply_tsconfig_aliases(&mut self, opts: &CompilerOptions) {
        let Some(paths) = &opts.paths else { return };
        let base_url = opts.base_url.as_deref().unwrap_or("").trim_end_matches('/');

        for (alias, targets) in paths {
            if let (Entry::Vacant(e), Some(target)) =
                (self.aliases.entry(alias.clone()), targets.first())
            {
                let actual_path = if base_url.is_empty() {
                    if target.starts_with("./") || target.starts_with("/") {
                        target.clone()
                    } else {
                        format!("./{}", target)
                    }
                } else {
                    format!("{}/{}", base_url, target)
                };
                e.insert(actual_path);
            }
        }
    }

    /// Adds the `outDir` from a `CompilerOptions` to the ignore patterns.
    /// This prevents analyzing compiled artifacts.
    fn apply_tsconfig_out_dir(&mut self, opts: &CompilerOptions) {
        if let Some(out_dir) = &opts.out_dir {
            self.add_ignore_pattern(out_dir);
        }
    }

    /// Adds standard TypeScript exclude patterns to the ignore list.
    /// Patterns containing glob characters are added directly, others are converted to directory globs.
    fn apply_tsconfig_excludes(&mut self, excludes: Vec<String>) {
        for exclude in excludes {
            self.add_ignore_pattern(&exclude);
        }
    }

    /// Helper to add a path to the ignore list, ensuring it's formatted as a glob pattern.
    /// Normalizes path separators and trims common prefixes before creating a `**/{path}/**` pattern.
    fn add_ignore_pattern(&mut self, path: &str) {
        let normalized = path.replace('\\', "/");
        let path = normalized.trim_matches('/').trim_start_matches("./");
        if path.is_empty() || path.split('/').any(|p| p == "..") {
            return;
        }

        let pattern = if path.contains('*') {
            path.to_string()
        } else {
            format!("**/{}/**", path)
        };

        if !self.ignore.contains(&pattern) {
            self.ignore.push(pattern);
        }
    }
}

#[cfg(test)]
mod tests;
