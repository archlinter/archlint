use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a TypeScript configuration file (tsconfig.json).
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TsConfig {
    /// Compiler options section.
    pub compiler_options: Option<CompilerOptions>,
    /// Patterns to exclude from the project.
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Path to a parent configuration file to extend.
    pub extends: Option<String>,
}

/// Represents the `compilerOptions` section of a tsconfig file.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    /// Path mapping aliases.
    pub paths: Option<HashMap<String, Vec<String>>>,
    /// Base directory to resolve non-relative module names.
    pub base_url: Option<String>,
    /// Output directory for compiled files.
    pub out_dir: Option<String>,
    /// Root directory of source files.
    pub root_dir: Option<String>,
}

impl CompilerOptions {
    /// Merges another `CompilerOptions` into this one.
    /// Existing values take precedence over the ones from `other`.
    fn merge(&mut self, other: CompilerOptions) {
        let CompilerOptions {
            paths,
            base_url,
            out_dir,
            root_dir,
        } = other;

        if let Some(other_paths) = paths {
            let paths = self.paths.get_or_insert_with(HashMap::new);
            for (k, v) in other_paths {
                paths.entry(k).or_insert(v);
            }
        }
        if self.base_url.is_none() {
            self.base_url = base_url;
        }
        if self.out_dir.is_none() {
            self.out_dir = out_dir;
        }
        if self.root_dir.is_none() {
            self.root_dir = root_dir;
        }
    }
}

impl TsConfig {
    /// Loads a tsconfig file from the given path, resolving `extends` recursively.
    pub fn load(path: &Path) -> Result<Self> {
        let mut visited = HashSet::new();
        Self::load_internal(path, &mut visited)
    }

    /// Internal implementation of `load` with cycle detection.
    fn load_internal(path: &Path, visited: &mut HashSet<PathBuf>) -> Result<Self> {
        let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        if !visited.insert(canonical_path) {
            return Err(anyhow::anyhow!("Circular extends detected: {:?}", path).into());
        }

        let contents = fs::read_to_string(path)?;
        let mut config: TsConfig = json5::from_str(&contents)?;

        if let Some(extends) = &config.extends {
            let parent_path = if extends.starts_with('.') {
                path.parent()
                    .ok_or_else(|| anyhow::anyhow!("tsconfig path has no parent directory"))?
                    .join(extends)
            } else {
                // For now, only relative extends are supported easily.
                // In a real TS environment, this could be an npm package.
                // We'll skip non-relative for now to avoid complexity with node_modules.
                PathBuf::from(extends)
            };

            if parent_path.exists() {
                let parent_config = Self::load_internal(&parent_path, visited)?;
                config = config.merge_with_parent(parent_config);
            }
        }

        Ok(config)
    }

    /// Attempts to find and load a tsconfig file in the project root.
    /// If `explicit_path` is provided, it tries to load that specific file.
    /// Otherwise, it looks for the standard `tsconfig.json`.
    pub fn find_and_load(project_root: &Path, explicit_path: Option<&str>) -> Result<Option<Self>> {
        if let Some(p) = explicit_path {
            let path = project_root.join(p);
            if path.exists() {
                return Ok(Some(Self::load(&path)?));
            }
        }

        // Look for standard tsconfig.json only
        // If you need a different file (e.g., tsconfig.build.json), specify it explicitly via config.tsconfig
        let tsconfig_path = project_root.join("tsconfig.json");
        if tsconfig_path.exists() {
            return Ok(Some(Self::load(&tsconfig_path)?));
        }

        Ok(None)
    }

    /// Merges a parent `TsConfig` into this one.
    /// This config's values take precedence over the parent's.
    fn merge_with_parent(mut self, parent: TsConfig) -> Self {
        // Simple merge: current config overrides parent
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
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_basic_tsconfig() -> Result<()> {
        let dir = tempdir()?;
        let tsconfig_path = dir.path().join("tsconfig.json");
        fs::write(
            &tsconfig_path,
            r#"{
                "compilerOptions": {
                    "baseUrl": "./src",
                    "paths": { "@app/*": ["app/*"] },
                    "outDir": "dist"
                },
                "exclude": ["node_modules"]
            }"#,
        )?;

        let config = TsConfig::load(&tsconfig_path)?;
        let opts = config.compiler_options.unwrap();
        assert_eq!(opts.base_url.unwrap(), "./src");
        assert_eq!(opts.out_dir.unwrap(), "dist");
        assert_eq!(opts.paths.unwrap().get("@app/*").unwrap()[0], "app/*");
        assert_eq!(config.exclude, vec!["node_modules"]);

        Ok(())
    }

    #[test]
    fn test_load_with_comments() -> Result<()> {
        let dir = tempdir()?;
        let tsconfig_path = dir.path().join("tsconfig.json");
        fs::write(
            &tsconfig_path,
            r#"{
                // This is a comment
                "compilerOptions": {
                    /* Multi-line comment */
                    "baseUrl": "."
                }
            }"#,
        )?;

        let config = TsConfig::load(&tsconfig_path)?;
        assert_eq!(config.compiler_options.unwrap().base_url.unwrap(), ".");
        Ok(())
    }

    #[test]
    fn test_extends_and_merge() -> Result<()> {
        let dir = tempdir()?;

        fs::write(
            dir.path().join("tsconfig.base.json"),
            r#"{
                "compilerOptions": {
                    "baseUrl": ".",
                    "paths": {
                        "@base/*": ["base/*"],
                        "@overridden/*": ["base-overridden/*"]
                    }
                },
                "exclude": ["node_modules", "dist"]
            }"#,
        )?;

        let tsconfig_path = dir.path().join("tsconfig.json");
        fs::write(
            &tsconfig_path,
            r#"{
                "extends": "./tsconfig.base.json",
                "compilerOptions": {
                    "paths": {
                        "@app/*": ["app/*"],
                        "@overridden/*": ["app-overridden/*"]
                    }
                },
                "exclude": ["custom-exclude"]
            }"#,
        )?;

        let config = TsConfig::load(&tsconfig_path)?;
        let opts = config.compiler_options.unwrap();
        let paths = opts.paths.unwrap();

        // Check merged paths
        assert_eq!(paths.get("@base/*").unwrap()[0], "base/*");
        assert_eq!(paths.get("@app/*").unwrap()[0], "app/*");
        assert_eq!(paths.get("@overridden/*").unwrap()[0], "app-overridden/*");

        // Check merged excludes
        assert!(config.exclude.contains(&"node_modules".to_string()));
        assert!(config.exclude.contains(&"dist".to_string()));
        assert!(config.exclude.contains(&"custom-exclude".to_string()));

        Ok(())
    }

    #[test]
    fn test_find_and_load() -> Result<()> {
        let dir = tempdir()?;

        // Should return None if no tsconfig.json exists
        assert!(TsConfig::find_and_load(dir.path(), None)?.is_none());

        // Should find tsconfig.json when it exists
        fs::write(
            dir.path().join("tsconfig.json"),
            r#"{"compilerOptions": {"baseUrl": "."}}"#,
        )?;

        let config = TsConfig::find_and_load(dir.path(), None)?.unwrap();
        assert_eq!(config.compiler_options.unwrap().base_url.unwrap(), ".");

        // Explicit path should take precedence
        fs::write(
            dir.path().join("tsconfig.build.json"),
            r#"{"compilerOptions": {"baseUrl": "build"}}"#,
        )?;

        let config = TsConfig::find_and_load(dir.path(), Some("tsconfig.build.json"))?.unwrap();
        assert_eq!(config.compiler_options.unwrap().base_url.unwrap(), "build");

        Ok(())
    }

    #[test]
    fn test_circular_extends() -> Result<()> {
        let dir = tempdir()?;
        let path1 = dir.path().join("tsconfig.1.json");
        let path2 = dir.path().join("tsconfig.2.json");

        fs::write(&path1, r#"{"extends": "./tsconfig.2.json"}"#)?;
        fs::write(&path2, r#"{"extends": "./tsconfig.1.json"}"#)?;

        let result = TsConfig::load(&path1);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Circular extends"));

        Ok(())
    }

    #[test]
    fn test_invalid_json5() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("tsconfig.json");
        fs::write(&path, r#"{"compilerOptions": { "baseUrl": "." "#)?; // Missing closing braces

        let result = TsConfig::load(&path);
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_non_existent_extends() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("tsconfig.json");
        fs::write(&path, r#"{"extends": "./non-existent.json"}"#)?;

        // Should not error, just skip if it doesn't exist (matching current logic)
        let config = TsConfig::load(&path)?;
        assert!(config.compiler_options.is_none());

        Ok(())
    }

    #[test]
    fn test_missing_parent_directory() -> Result<()> {
        // Path with no parent (just a filename)
        let _path = Path::new("tsconfig.json");
        // This would normally be handled by the fact that we can't read the file,
        // but if it has "extends": "./something", it should fail safely.

        // We can't easily test filesystem root, but we can test a relative path with "extends": "./..."
        // which triggers path.parent() call.

        let dir = tempdir()?;
        let tsconfig_path = dir.path().join("tsconfig.json");
        fs::write(&tsconfig_path, r#"{"extends": "./base.json"}"#)?;

        // This works because join(extends) works on path.parent().
        let config = TsConfig::load(&tsconfig_path)?;
        assert!(config.extends.is_some());

        Ok(())
    }
}
