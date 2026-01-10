use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TsConfig {
    pub compiler_options: Option<CompilerOptions>,
    #[serde(default)]
    pub exclude: Vec<String>,
    pub extends: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    pub paths: Option<HashMap<String, Vec<String>>>,
    pub base_url: Option<String>,
    pub out_dir: Option<String>,
    pub root_dir: Option<String>,
}

impl CompilerOptions {
    fn merge(&mut self, other: CompilerOptions) {
        if let Some(other_paths) = other.paths {
            let paths = self.paths.get_or_insert_with(HashMap::new);
            for (k, v) in other_paths {
                paths.entry(k).or_insert(v);
            }
        }
        if self.base_url.is_none() {
            self.base_url = other.base_url;
        }
        if self.out_dir.is_none() {
            self.out_dir = other.out_dir;
        }
        if self.root_dir.is_none() {
            self.root_dir = other.root_dir;
        }
    }
}

impl TsConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut config: TsConfig = json5::from_str(&contents)?;

        if let Some(extends) = &config.extends {
            let parent_path = if extends.starts_with('.') {
                path.parent().unwrap().join(extends)
            } else {
                // For now, only relative extends are supported easily.
                // In a real TS environment, this could be an npm package.
                // We'll skip non-relative for now to avoid complexity with node_modules.
                PathBuf::from(extends)
            };

            if parent_path.exists() {
                let parent_config = Self::load(&parent_path)?;
                config = config.merge_with_parent(parent_config);
            }
        }

        Ok(config)
    }

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

    fn merge_with_parent(mut self, parent: TsConfig) -> Self {
        // Simple merge: current config overrides parent
        if let Some(parent_opts) = parent.compiler_options {
            self.compiler_options
                .get_or_insert_with(CompilerOptions::default)
                .merge(parent_opts);
        }

        for ex in parent.exclude {
            if !self.exclude.contains(&ex) {
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
}
