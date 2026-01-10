use crate::detectors::Severity;
use crate::tsconfig::TsConfig;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub ignore: Vec<String>,

    #[serde(default)]
    pub aliases: HashMap<String, String>,

    #[serde(default)]
    pub entry_points: Vec<String>,

    #[serde(default)]
    pub rules: HashMap<String, RuleConfig>,

    #[serde(default)]
    pub overrides: Vec<Override>,

    #[serde(default)]
    pub scoring: SeverityConfig,

    #[serde(default)]
    pub watch: WatchConfig,

    #[serde(default, deserialize_with = "deserialize_extends")]
    pub extends: Vec<String>,

    #[serde(default)]
    pub framework: Option<String>,

    #[serde(default = "default_true")]
    pub auto_detect_framework: bool,

    #[serde(default = "default_tsconfig_config")]
    pub tsconfig: Option<TsConfigConfig>,

    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,

    #[serde(default)]
    pub git: GitConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum TsConfigConfig {
    Boolean(bool),
    Path(String),
}

impl Default for TsConfigConfig {
    fn default() -> Self {
        TsConfigConfig::Boolean(true)
    }
}

fn default_tsconfig_config() -> Option<TsConfigConfig> {
    Some(TsConfigConfig::default())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_history_period")]
    pub history_period: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleSeverity {
    Info,
    Low,
    Warn,
    Medium,
    Error,
    High,
    Critical,
    Off,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum RuleConfig {
    Short(RuleSeverity),
    Full(RuleFullConfig),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RuleFullConfig {
    #[serde(default)]
    pub severity: Option<RuleSeverity>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(flatten)]
    pub options: serde_yaml::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Override {
    pub files: Vec<String>,
    pub rules: HashMap<String, RuleConfig>,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            history_period: default_history_period(),
        }
    }
}

fn default_history_period() -> String {
    "1y".to_string()
}

fn default_true() -> bool {
    true
}

fn default_max_file_size() -> u64 {
    1024 * 1024 // 1MB
}

fn deserialize_extends<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    let parsed = Option::<StringOrVec>::deserialize(deserializer)?;
    match parsed {
        Some(StringOrVec::String(s)) => Ok(vec![s]),
        Some(StringOrVec::Vec(v)) => Ok(v),
        None => Ok(Vec::new()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchConfig {
    #[serde(default = "default_debounce_ms")]
    pub debounce_ms: u64,

    #[serde(default = "default_clear_screen")]
    pub clear_screen: bool,

    #[serde(default)]
    pub ignore: Vec<String>,
}

fn default_debounce_ms() -> u64 {
    300
}

fn default_clear_screen() -> bool {
    false
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            debounce_ms: default_debounce_ms(),
            clear_screen: default_clear_screen(),
            ignore: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityConfig {
    #[serde(default = "default_weights")]
    pub weights: SeverityWeights,

    #[serde(default)]
    pub grade_thresholds: GradeThresholds,

    #[serde(default = "default_min_severity")]
    pub minimum: Option<Severity>,

    #[serde(default)]
    pub minimum_score: Option<u32>,
}

impl Default for SeverityConfig {
    fn default() -> Self {
        Self {
            weights: default_weights(),
            grade_thresholds: GradeThresholds::default(),
            minimum: Some(Severity::Low),
            minimum_score: None,
        }
    }
}

fn default_min_severity() -> Option<Severity> {
    Some(Severity::Low)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityWeights {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeThresholds {
    pub excellent: f32,
    pub good: f32,
    pub fair: f32,
    pub moderate: f32,
    pub poor: f32,
}

impl Default for GradeThresholds {
    fn default() -> Self {
        Self {
            excellent: 1.0,
            good: 3.0,
            fair: 7.0,
            moderate: 15.0,
            poor: 30.0,
        }
    }
}

fn default_weights() -> SeverityWeights {
    SeverityWeights {
        critical: 100,
        high: 50,
        medium: 20,
        low: 5,
    }
}

impl SeverityWeights {
    pub fn score(&self, severity: &Severity) -> u32 {
        match severity {
            Severity::Critical => self.critical,
            Severity::High => self.high,
            Severity::Medium => self.medium,
            Severity::Low => self.low,
        }
    }
}

impl Default for SeverityWeights {
    fn default() -> Self {
        default_weights()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DetectorConfig {
    #[serde(default)]
    pub enabled: Option<Vec<String>>,
    #[serde(default)]
    pub disabled: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LayerConfig {
    pub name: String,
    #[serde(alias = "paths")]
    pub path: String,
    #[serde(alias = "can_import")]
    pub allowed_imports: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ignore: vec![
                "**/*.test.ts".to_string(),
                "**/*.test.js".to_string(),
                "**/*.spec.ts".to_string(),
                "**/*.spec.js".to_string(),
                "**/__tests__/**".to_string(),
                "**/__mocks__/**".to_string(),
                "**/test/**".to_string(),
                "**/tests/**".to_string(),
                "**/__fixtures__/**".to_string(),
                "**/*.mock.ts".to_string(),
                "**/*.mock.js".to_string(),
            ],
            aliases: HashMap::new(),
            entry_points: Vec::new(),
            rules: HashMap::new(),
            overrides: Vec::new(),
            scoring: SeverityConfig::default(),
            watch: WatchConfig::default(),
            extends: Vec::new(),
            framework: None,
            auto_detect_framework: true,
            tsconfig: Some(TsConfigConfig::default()),
            max_file_size: default_max_file_size(),
            git: GitConfig::default(),
        }
    }
}

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

    pub fn enrich_from_tsconfig(&mut self, project_root: &Path) -> Result<()> {
        let explicit_path = match &self.tsconfig {
            Some(TsConfigConfig::Path(p)) => Some(p.as_str()),
            _ => None,
        };

        if let Some(tsconfig) = TsConfig::find_and_load(project_root, explicit_path)? {
            if let Some(opts) = tsconfig.compiler_options {
                // Enrich aliases
                if let Some(paths) = opts.paths {
                    for (alias, targets) in paths {
                        if let Entry::Vacant(e) = self.aliases.entry(alias) {
                            if let Some(target) = targets.first() {
                                // tsconfig paths are relative to baseUrl
                                let actual_path = if let Some(base_url) = &opts.base_url {
                                    format!("{}/{}", base_url.trim_end_matches('/'), target)
                                } else {
                                    target.clone()
                                };
                                e.insert(actual_path);
                            }
                        }
                    }
                }

                // Add outDir to ignore
                if let Some(out_dir) = opts.out_dir {
                    let ignore_pattern = format!("**/{}/**", out_dir.trim_matches('/'));
                    if !self.ignore.contains(&ignore_pattern) {
                        self.ignore.push(ignore_pattern);
                    }
                }
            }

            // Add excludes to ignore
            for exclude in tsconfig.exclude {
                let pattern = if exclude.contains('*') {
                    exclude
                } else {
                    format!("**/{}/**", exclude.trim_matches('/'))
                };
                if !self.ignore.contains(&pattern) {
                    self.ignore.push(pattern);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_deserialize_extends_single_string() {
        let yaml = "extends: nestjs";
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.extends, vec!["nestjs"]);
    }

    #[test]
    fn test_deserialize_extends_list() {
        let yaml = "extends:\n  - nestjs\n  - react";
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.extends, vec!["nestjs", "react"]);
    }

    #[test]
    fn test_deserialize_extends_missing() {
        let yaml = "{}";
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.extends.is_empty());
    }

    #[test]
    fn test_deserialize_extends_null() {
        let yaml = "extends: null";
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.extends.is_empty());
    }

    #[test]
    fn test_tsconfig_disabled_boolean() {
        let yaml = "tsconfig: false";
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(matches!(
            config.tsconfig,
            Some(TsConfigConfig::Boolean(false))
        ));
    }

    #[test]
    fn test_tsconfig_disabled_null() {
        let yaml = "tsconfig: null";
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.tsconfig.is_none());
    }

    #[test]
    fn test_tsconfig_path() {
        let yaml = "tsconfig: ./custom.json";
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        if let Some(TsConfigConfig::Path(p)) = &config.tsconfig {
            assert_eq!(p, "./custom.json");
        } else {
            panic!("Expected TsConfigConfig::Path");
        }
    }

    #[test]
    fn test_enrich_from_tsconfig() -> Result<()> {
        let dir = tempdir()?;
        fs::write(
            dir.path().join("tsconfig.json"),
            r#"{
                "compilerOptions": {
                    "baseUrl": "./src",
                    "paths": {
                        "@app/*": ["app/*"],
                        "@shared/*": ["shared/*"]
                    },
                    "outDir": "dist"
                },
                "exclude": ["temp"]
            }"#,
        )?;

        let mut config = Config::default();
        config
            .aliases
            .insert("@app/*".to_string(), "custom/app/*".to_string());

        config.enrich_from_tsconfig(dir.path())?;

        // @app/* should be kept from config (priority)
        assert_eq!(config.aliases.get("@app/*").unwrap(), "custom/app/*");
        // @shared/* should be loaded from tsconfig
        assert_eq!(config.aliases.get("@shared/*").unwrap(), "./src/shared/*");

        // dist and temp should be in ignore
        assert!(config.ignore.contains(&"**/dist/**".to_string()));
        assert!(config.ignore.contains(&"**/temp/**".to_string()));

        Ok(())
    }

    #[test]
    fn test_enrich_from_tsconfig_with_extends() -> Result<()> {
        let dir = tempdir()?;

        // Create base tsconfig
        fs::write(
            dir.path().join("tsconfig.base.json"),
            r#"{
                "compilerOptions": {
                    "baseUrl": ".",
                    "paths": {
                        "@core/*": ["core/*"],
                        "@utils/*": ["utils/*"]
                    },
                    "outDir": "build"
                },
                "exclude": ["node_modules"]
            }"#,
        )?;

        // Create main tsconfig that extends base
        fs::write(
            dir.path().join("tsconfig.json"),
            r#"{
                "extends": "./tsconfig.base.json",
                "compilerOptions": {
                    "paths": {
                        "@app/*": ["src/app/*"]
                    }
                },
                "exclude": ["tmp"]
            }"#,
        )?;

        let mut config = Config::default();
        config.enrich_from_tsconfig(dir.path())?;

        // Should have paths from both base and main tsconfig
        assert_eq!(config.aliases.get("@core/*").unwrap(), "./core/*");
        assert_eq!(config.aliases.get("@utils/*").unwrap(), "./utils/*");
        assert_eq!(config.aliases.get("@app/*").unwrap(), "./src/app/*");

        // Should have excludes from both
        assert!(config.ignore.contains(&"**/node_modules/**".to_string()));
        assert!(config.ignore.contains(&"**/tmp/**".to_string()));
        assert!(config.ignore.contains(&"**/build/**".to_string()));

        Ok(())
    }

    #[test]
    fn test_enrich_from_tsconfig_real_project_structure() -> Result<()> {
        let dir = tempdir()?;

        // Simulate real project structure: packages/plugin-api/tsconfig.json extends ../../tsconfig.base.json
        fs::create_dir_all(dir.path().join("packages/plugin-api"))?;

        // Create base tsconfig at root
        fs::write(
            dir.path().join("tsconfig.base.json"),
            r#"{
                "compilerOptions": {
                    "target": "ES2022",
                    "strict": true,
                    "outDir": "dist",
                    "baseUrl": ".",
                    "paths": {
                        "@shared/*": ["shared/*"]
                    }
                },
                "exclude": ["node_modules", "dist"]
            }"#,
        )?;

        // Create package tsconfig that extends base with relative path
        fs::write(
            dir.path().join("packages/plugin-api/tsconfig.json"),
            r#"{
                "extends": "../../tsconfig.base.json",
                "compilerOptions": {
                    "rootDir": "src",
                    "paths": {
                        "@plugin/*": ["src/plugin/*"]
                    }
                }
            }"#,
        )?;

        // Load config from package directory
        let mut config = Config::default();
        let package_dir = dir.path().join("packages/plugin-api");
        config.enrich_from_tsconfig(&package_dir)?;

        // Should have paths from both root base and package tsconfig
        assert_eq!(config.aliases.get("@shared/*").unwrap(), "./shared/*");
        assert_eq!(config.aliases.get("@plugin/*").unwrap(), "./src/plugin/*");

        // Should have excludes from base
        assert!(config.ignore.contains(&"**/node_modules/**".to_string()));
        assert!(config.ignore.contains(&"**/dist/**".to_string()));

        Ok(())
    }
}
