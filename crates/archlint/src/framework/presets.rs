use super::preset_loader::PresetLoader;
use super::Framework;
use crate::config::{Override, RuleConfig};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FrameworkPreset {
    pub name: String,
    pub rules: HashMap<String, RuleConfig>,
    pub entry_points: Vec<String>,
    pub overrides: Vec<Override>,
}

#[must_use]
pub fn get_presets(frameworks: &[Framework]) -> Vec<FrameworkPreset> {
    frameworks
        .iter()
        .filter_map(|f| PresetLoader::load_builtin(&f.0).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RuleConfig;

    #[test]
    fn test_get_presets_empty() {
        let presets = get_presets(&[]);
        assert!(presets.is_empty());
    }

    #[test]
    fn test_get_presets_known_frameworks() {
        let frameworks = vec![
            Framework("nestjs".to_string()),
            Framework("nextjs".to_string()),
            Framework("react".to_string()),
            Framework("oclif".to_string()),
        ];
        let presets = get_presets(&frameworks);
        assert_eq!(presets.len(), 4);
        assert_eq!(presets[0].name, "nestjs");
        assert_eq!(presets[1].name, "nextjs");
        assert_eq!(presets[2].name, "react");
        assert_eq!(presets[3].name, "oclif");
    }

    #[test]
    fn test_nestjs_preset_rules() {
        let preset = PresetLoader::load_builtin("nestjs").unwrap();
        assert_eq!(preset.name, "nestjs");
        assert!(preset.rules.contains_key("layer_violation"));
    }

    #[test]
    fn test_nextjs_preset_rules() {
        let preset = PresetLoader::load_builtin("nextjs").unwrap();
        assert_eq!(preset.name, "nextjs");
        let rule = preset.rules.get("layer_violation").unwrap();
        if let RuleConfig::Short(sev) = rule {
            assert_eq!(*sev, crate::config::RuleSeverity::Off);
        } else {
            panic!("Expected Short(Off)");
        }
    }
}
