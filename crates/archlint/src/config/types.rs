use crate::detectors::Severity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub(crate) fn default_tsconfig_config() -> Option<TsConfigConfig> {
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

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            history_period: default_history_period(),
        }
    }
}

pub(crate) fn default_history_period() -> String {
    "1y".to_string()
}

pub(crate) fn default_true() -> bool {
    true
}

pub(crate) fn default_max_file_size() -> u64 {
    1024 * 1024 // 1MB
}

/// Severity levels for architectural rules.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleSeverity {
    /// Low severity issue.
    Low,
    /// Medium severity issue.
    Medium,
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

pub(crate) fn deserialize_extends<'de, D>(
    deserializer: D,
) -> std::result::Result<Vec<String>, D::Error>
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
