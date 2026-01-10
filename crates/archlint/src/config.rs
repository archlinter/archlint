use crate::detectors::Severity;
use crate::tsconfig::{CompilerOptions, TsConfig};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Main configuration structure for archlint.
/// Defines project settings, rules, and framework extensions.
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

/// Configuration options for TypeScript integration.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum TsConfigConfig {
    /// Enable or disable automatic tsconfig discovery.
    Boolean(bool),
    /// Use a specific tsconfig file path.
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

/// Configuration for Git-based analysis features.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitConfig {
    /// Whether to enable Git analysis (e.g., for calculating churn).
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Time period to look back in Git history (e.g., "1y", "6m").
    #[serde(default = "default_history_period")]
    pub history_period: String,
}

/// Severity levels for architectural rules.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleSeverity {
    /// Informational message (internally mapped to Low).
    Info,
    /// Low severity issue.
    Low,
    /// Warning message (internally mapped to Medium).
    Warn,
    /// Medium severity issue.
    Medium,
    /// Error message (internally mapped to High).
    Error,
    /// High severity issue.
    High,
    /// Critical architectural violation.
    Critical,
    /// Rule is disabled.
    Off,
}

/// Flexible configuration for a single rule.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum RuleConfig {
    /// Short form: just the severity level.
    Short(RuleSeverity),
    /// Full form: includes severity, enabled flag, and custom options.
    Full(RuleFullConfig),
}

/// Detailed configuration for a single rule.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RuleFullConfig {
    /// Override the default severity for this rule.
    #[serde(default)]
    pub severity: Option<RuleSeverity>,
    /// Explicitly enable or disable this rule.
    #[serde(default)]
    pub enabled: Option<bool>,
    /// Patterns to exclude from this specific rule.
    #[serde(default)]
    pub exclude: Vec<String>,
    /// Additional detector-specific options.
    #[serde(flatten)]
    pub options: serde_yaml::Value,
}

/// Configuration overrides for specific file patterns.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Override {
    /// Files or directories to apply these overrides to.
    pub files: Vec<String>,
    /// Rule configurations to override.
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

/// Configuration for the file watcher (watch mode).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchConfig {
    /// Delay in milliseconds before triggering a re-scan after a change.
    #[serde(default = "default_debounce_ms")]
    pub debounce_ms: u64,

    /// Whether to clear the terminal screen before each re-scan.
    #[serde(default = "default_clear_screen")]
    pub clear_screen: bool,

    /// Patterns to ignore during watch mode.
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

/// Configuration for issue scoring and project grading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityConfig {
    /// Weights for each severity level.
    #[serde(default = "default_weights")]
    pub weights: SeverityWeights,

    /// Thresholds for calculating the final project grade.
    #[serde(default)]
    pub grade_thresholds: GradeThresholds,

    /// Minimum severity level to report.
    #[serde(default = "default_min_severity")]
    pub minimum: Option<Severity>,

    /// Minimum score required to report a smell.
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

/// Weights assigned to each severity level for score calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityWeights {
    /// Score weight for Critical issues.
    pub critical: u32,
    /// Score weight for High issues.
    pub high: u32,
    /// Score weight for Medium issues.
    pub medium: u32,
    /// Score weight for Low issues.
    pub low: u32,
}

/// Thresholds for project grades based on smell density.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeThresholds {
    /// Maximum density for 'Excellent' grade.
    pub excellent: f32,
    /// Maximum density for 'Good' grade.
    pub good: f32,
    /// Maximum density for 'Fair' grade.
    pub fair: f32,
    /// Maximum density for 'Moderate' grade.
    pub moderate: f32,
    /// Maximum density for 'Poor' grade.
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

    /// Enriches the current configuration with settings from a TypeScript configuration file.
    /// This includes loading path aliases, adding `outDir` to ignores, and including `exclude` patterns.
    pub fn enrich_from_tsconfig(&mut self, project_root: &Path) -> Result<()> {
        let explicit_path = match &self.tsconfig {
            Some(TsConfigConfig::Path(p)) => Some(p.as_str()),
            _ => None,
        };

        let Some(tsconfig) = TsConfig::find_and_load(project_root, explicit_path)? else {
            return Ok(());
        };

        if let Some(opts) = tsconfig.compiler_options {
            self.apply_tsconfig_aliases(&opts);
            self.apply_tsconfig_out_dir(&opts);
        }

        self.apply_tsconfig_excludes(tsconfig.exclude);

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
                    target.clone()
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
            if exclude.contains('*') {
                if !self.ignore.contains(&exclude) {
                    self.ignore.push(exclude);
                }
            } else {
                self.add_ignore_pattern(&exclude);
            }
        }
    }

    /// Helper to add a path to the ignore list, ensuring it's formatted as a glob pattern.
    /// Trims common prefixes and suffixes before creating a `**/{path}/**` pattern.
    fn add_ignore_pattern(&mut self, path: &str) {
        let path = path.trim_matches('/').trim_start_matches("./");
        if path.is_empty() {
            return;
        }
        let pattern = format!("**/{}/**", path);
        if !self.ignore.contains(&pattern) {
            self.ignore.push(pattern);
        }
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
