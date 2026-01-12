use crate::config::{Config, RuleConfig, RuleSeverity};
use crate::detectors::{Detector, DetectorCategory};
use crate::framework::presets::FrameworkPreset;
use inventory;
use std::collections::HashMap;
use std::sync::OnceLock;

static REGISTRY_FACTORIES: OnceLock<HashMap<&'static str, &'static dyn DetectorFactory>> =
    OnceLock::new();

/// Metadata for a detector
pub struct DetectorInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub default_enabled: bool,
    pub is_deep: bool,
    pub category: DetectorCategory,
}

/// Factory for creating detector instances
pub trait DetectorFactory: Send + Sync {
    fn info(&self) -> DetectorInfo;
    fn create(&self, config: &Config) -> Box<dyn Detector>;
}

/// A detector with its unique ID
pub type RegisteredDetector = (String, Box<dyn Detector>);

// Submit a factory to the global registry
inventory::collect!(&'static dyn DetectorFactory);

/// Registry of all available detectors
pub struct DetectorRegistry {
    factories: &'static HashMap<&'static str, &'static dyn DetectorFactory>,
}

impl DetectorRegistry {
    pub fn new() -> Self {
        // Force initialization of all detector modules
        crate::detectors::init();

        let factories = REGISTRY_FACTORIES.get_or_init(|| {
            let mut m = HashMap::new();
            for factory in inventory::iter::<&'static dyn DetectorFactory> {
                m.insert(factory.info().id, *factory);
            }
            m
        });

        Self { factories }
    }

    pub fn list_all(&self) -> Vec<DetectorInfo> {
        let mut infos: Vec<_> = self.factories.values().map(|f| f.info()).collect();
        infos.sort_by_key(|i| i.id);
        infos
    }

    pub fn get_enabled(
        &self,
        config: &Config,
        all_detectors: bool,
    ) -> (Vec<Box<dyn Detector>>, bool) {
        self.get_enabled_with_presets(config, &[], all_detectors)
    }

    pub fn get_enabled_with_presets(
        &self,
        config: &Config,
        presets: &[FrameworkPreset],
        all_detectors: bool,
    ) -> (Vec<Box<dyn Detector>>, bool) {
        let (enabled, needs_deep) = self.get_enabled_full(config, presets, all_detectors);
        (enabled.into_iter().map(|(_, d)| d).collect(), needs_deep)
    }

    pub fn get_enabled_full(
        &self,
        config: &Config,
        presets: &[FrameworkPreset],
        all_detectors: bool,
    ) -> (Vec<RegisteredDetector>, bool) {
        let mut detectors = Vec::new();
        let mut needs_deep = false;

        for factory in self.factories.values() {
            let info = factory.info();

            if self.is_detector_enabled(&info, config, presets, all_detectors) {
                if info.is_deep {
                    needs_deep = true;
                }
                detectors.push((info.id.to_string(), factory.create(config)));
            }
        }

        (detectors, needs_deep)
    }

    fn is_detector_enabled(
        &self,
        info: &DetectorInfo,
        config: &Config,
        presets: &[FrameworkPreset],
        all_detectors: bool,
    ) -> bool {
        if all_detectors {
            return true;
        }

        if config.rules.contains_key(info.id) {
            let resolved = crate::rule_resolver::ResolvedRuleConfig::resolve(config, info.id, None);
            return resolved.enabled;
        }

        for preset in presets.iter().rev() {
            if let Some(rule_config) = preset.rules.get(info.id) {
                return match rule_config {
                    RuleConfig::Short(sev) => *sev != RuleSeverity::Off,
                    RuleConfig::Full(full) => {
                        if let Some(enabled) = full.enabled {
                            enabled
                        } else if let Some(severity) = &full.severity {
                            *severity != RuleSeverity::Off
                        } else {
                            info.default_enabled
                        }
                    }
                };
            }
        }

        info.default_enabled
    }

    pub fn get_info(&self, id: &str) -> Option<DetectorInfo> {
        self.factories.get(id).map(|f| f.info())
    }

    pub fn create_detector(&self, id: &str, config: &Config) -> Option<Box<dyn Detector>> {
        self.factories.get(id).map(|f| f.create(config))
    }
}

impl Default for DetectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
