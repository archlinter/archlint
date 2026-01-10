use crate::config::{Config, RuleConfig, RuleSeverity};
use crate::detectors::Severity;
use glob::Pattern;
use serde::de::DeserializeOwned;
use std::path::Path;

pub struct ResolvedRuleConfig {
    pub enabled: bool,
    pub severity: Severity,
    pub exclude: Vec<String>,
    pub options: serde_yaml::Value,
}

impl ResolvedRuleConfig {
    pub fn resolve(config: &Config, detector_id: &str, file_path: Option<&Path>) -> Self {
        let mut enabled = true;
        let mut severity = Severity::Medium;
        let mut exclude = Vec::new();
        let mut options = serde_yaml::Mapping::new();

        // 1. Apply base rule config
        if let Some(rule_config) = config.rules.get(detector_id) {
            Self::apply_config(
                rule_config,
                &mut enabled,
                &mut severity,
                &mut exclude,
                &mut options,
            );
        }

        // 2. Apply overrides
        if let Some(path) = file_path {
            for ov in &config.overrides {
                if Self::matches_path(path, &ov.files) {
                    if let Some(rule_config) = ov.rules.get(detector_id) {
                        Self::apply_config(
                            rule_config,
                            &mut enabled,
                            &mut severity,
                            &mut exclude,
                            &mut options,
                        );
                    }
                }
            }
        }

        Self {
            enabled,
            severity,
            exclude,
            options: serde_yaml::Value::Mapping(options),
        }
    }

    fn apply_config(
        config: &RuleConfig,
        enabled: &mut bool,
        severity: &mut Severity,
        exclude: &mut Vec<String>,
        options: &mut serde_yaml::Mapping,
    ) {
        match config {
            RuleConfig::Short(s) => Self::apply_severity(*s, enabled, severity),
            RuleConfig::Full(full) => {
                if let Some(e) = full.enabled {
                    *enabled = e;
                }
                if let Some(s) = full.severity {
                    Self::apply_severity(s, enabled, severity);
                }
                if !full.exclude.is_empty() {
                    *exclude = full.exclude.clone();
                }
                if let serde_yaml::Value::Mapping(m) = &full.options {
                    options.extend(m.clone());
                }
            }
        }
    }

    fn apply_severity(rule_severity: RuleSeverity, enabled: &mut bool, severity: &mut Severity) {
        match rule_severity {
            RuleSeverity::Off => *enabled = false,
            RuleSeverity::Info | RuleSeverity::Low => {
                *enabled = true;
                *severity = Severity::Low;
            }
            RuleSeverity::Warn | RuleSeverity::Medium => {
                *enabled = true;
                *severity = Severity::Medium;
            }
            RuleSeverity::Error | RuleSeverity::High => {
                *enabled = true;
                *severity = Severity::High;
            }
            RuleSeverity::Critical => {
                *enabled = true;
                *severity = Severity::Critical;
            }
        }
    }

    fn matches_path(path: &Path, patterns: &[String]) -> bool {
        let path_str = path.to_string_lossy();
        for p in patterns {
            if let Ok(pattern) = Pattern::new(p) {
                if pattern.matches(&path_str) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_option<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        if let serde_yaml::Value::Mapping(m) = &self.options {
            if let Some(v) = m.get(serde_yaml::Value::String(key.to_string())) {
                return serde_yaml::from_value(v.clone()).ok();
            }
        }
        None
    }
}
