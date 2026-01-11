use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// Existing values in `self` take precedence over values from `other`.
    pub fn merge(&mut self, other: CompilerOptions) {
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
