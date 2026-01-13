pub mod detector;
pub mod preset_loader;
pub mod preset_types;
pub mod presets;

use serde::{Deserialize, Serialize};

/// Supported web and backend frameworks for specialized analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Framework(pub String);

impl std::fmt::Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Framework {
    pub fn as_preset_name(&self) -> String {
        self.0.clone()
    }

    /// Returns true if this framework has a built-in preset.
    pub fn has_builtin_preset(&self) -> bool {
        preset_loader::PresetLoader::get_all_builtin_names().contains(&self.0.as_str())
    }
}
