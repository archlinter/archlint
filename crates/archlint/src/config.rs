use crate::detectors::Severity;
use crate::Result;
use serde::{Deserialize, Serialize};
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

    #[serde(default)]
    pub framework: Option<String>,

    #[serde(default = "default_true")]
    pub auto_detect_framework: bool,

    #[serde(default = "default_true")]
    pub enable_git: bool,

    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,

    #[serde(default)]
    pub git: GitConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitConfig {
    #[serde(default = "default_history_period")]
    pub history_period: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleSeverity {
    Info,
    Warn,
    Error,
    Critical,
    Off,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RuleConfig {
    Short(RuleSeverity),
    Full(RuleFullConfig),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Override {
    pub files: Vec<String>,
    pub rules: HashMap<String, RuleConfig>,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
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
            framework: None,
            auto_detect_framework: true,
            enable_git: true,
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
        if let Some(p) = path {
            return Self::load(p);
        }

        let filenames = [
            ".archlint.yaml",
            ".archlint.yml",
            "archlint.yaml",
            "archlint.yml",
        ];

        for filename in filenames {
            let p = project_root
                .map(|root| root.join(filename))
                .unwrap_or_else(|| PathBuf::from(filename));

            if p.exists() {
                return Self::load(p);
            }
        }

        Ok(Self::default())
    }
}
