use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetYaml {
    pub name: String,
    pub version: u32,
    pub detect: DetectRules,

    #[serde(default)]
    pub rules: HashMap<String, crate::config::RuleConfig>,

    #[serde(default)]
    pub entry_points: Vec<String>,

    #[serde(default)]
    pub overrides: Vec<crate::config::Override>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectRules {
    pub packages: Option<MatchRules>,
    pub files: Option<MatchRules>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MatchRules {
    pub any_of: Option<Vec<String>>,
    pub all_of: Option<Vec<String>>,
}
