use super::presets::FrameworkPreset;
use crate::config::DetectorConfig;
use std::collections::HashSet;

pub struct DetectorSelector;

#[derive(Debug, Default)]
pub struct SelectedDetectors {
    pub user_enabled: Option<HashSet<String>>,
    pub preset_enabled: HashSet<String>,
    pub disabled: HashSet<String>,
}

impl DetectorSelector {
    pub fn select(user_config: &DetectorConfig, presets: &[FrameworkPreset]) -> SelectedDetectors {
        let mut result = SelectedDetectors::default();

        // 1. User explicit list takes absolute priority
        if let Some(user_enabled) = &user_config.enabled {
            result.user_enabled = Some(user_enabled.iter().cloned().collect());
        }

        // 2. Collect all disabled (user + presets)
        for d in &user_config.disabled {
            result.disabled.insert(d.clone());
        }
        for preset in presets {
            for d in &preset.disabled_detectors {
                result.disabled.insert(d.to_string());
            }
        }

        // 3. Collect preset enabled
        for preset in presets {
            for d in &preset.enabled_detectors {
                result.preset_enabled.insert(d.to_string());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::presets::get_presets;
    use crate::framework::Framework;

    #[test]
    fn test_selector_with_preset() {
        let user_config = DetectorConfig::default(); // empty
        let presets = get_presets(&[Framework::NestJS]);
        let selection = DetectorSelector::select(&user_config, &presets);

        assert!(selection.preset_enabled.contains("layer_violation"));
        assert!(selection.disabled.contains("scattered_module"));
    }

    #[test]
    fn test_selector_user_priority() {
        let user_config = DetectorConfig {
            enabled: Some(vec!["my_detector".to_string()]),
            ..Default::default()
        };

        let presets = get_presets(&[Framework::NestJS]);
        let selection = DetectorSelector::select(&user_config, &presets);

        assert!(selection.user_enabled.unwrap().contains("my_detector"));
    }
}
