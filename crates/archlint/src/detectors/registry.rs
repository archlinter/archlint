use crate::config::Config;
use crate::detectors::Detector;
use crate::framework::presets::FrameworkPreset;
use crate::framework::selector::DetectorSelector;
use inventory;
use std::collections::HashMap;

/// Metadata for a detector
pub struct DetectorInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub default_enabled: bool,
    pub is_deep: bool,
}

/// Factory for creating detector instances
pub trait DetectorFactory: Send + Sync {
    fn info(&self) -> DetectorInfo;
    fn create(&self, config: &Config) -> Box<dyn Detector>;
}

// Submit a factory to the global registry
inventory::collect!(&'static dyn DetectorFactory);

/// Registry of all available detectors
pub struct DetectorRegistry {
    factories: HashMap<&'static str, &'static dyn DetectorFactory>,
}

impl DetectorRegistry {
    pub fn new() -> Self {
        // Force initialization of all detector modules
        crate::detectors::init();

        let mut factories = HashMap::new();
        for factory in inventory::iter::<&'static dyn DetectorFactory> {
            factories.insert(factory.info().id, *factory);
        }
        Self { factories }
    }

    pub fn list_all(&self) -> Vec<DetectorInfo> {
        let mut infos: Vec<_> = self.factories.values().map(|f| f.info()).collect();
        infos.sort_by_key(|i| i.id);
        infos
    }

    pub fn get_enabled(&self, config: &Config, all_detectors: bool) -> (Vec<Box<dyn Detector>>, bool) {
        self.get_enabled_with_presets(config, &[], all_detectors)
    }

    pub fn get_enabled_with_presets(
        &self,
        config: &Config,
        presets: &[FrameworkPreset],
        all_detectors: bool,
    ) -> (Vec<Box<dyn Detector>>, bool) {
        let mut detectors = Vec::new();
        let mut needs_deep = false;

        let selection = DetectorSelector::select(&config.detectors, presets);

        for factory in self.factories.values() {
            let info = factory.info();
            let is_enabled = if all_detectors {
                true
            } else if let Some(ref user_enabled) = selection.user_enabled {
                user_enabled.contains(info.id)
            } else {
                (info.default_enabled || selection.preset_enabled.contains(info.id))
                    && !selection.disabled.contains(info.id)
            };

            if is_enabled {
                if info.is_deep {
                    needs_deep = true;
                }
                detectors.push(factory.create(config));
            }
        }

        (detectors, needs_deep)
    }
}

impl Default for DetectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}
