pub mod code_clone;
#[macro_use]
pub mod macros;
pub use archlint_macros::detector;
pub mod registry;
pub mod smell;
pub mod types;

pub mod dependency;
pub mod design;
pub mod hygiene;
pub mod metrics;

pub use registry::{DetectorFactory, DetectorInfo, DetectorRegistry};
pub use smell::{ArchSmell, CodeRange, CriticalEdge, CycleCluster, HotspotInfo, LocationDetail};
pub use types::{
    ConfigurableSmellType, DetectorCategory, Explanation, Severity, SmellMetric, SmellType,
    SmellWithExplanation,
};

// Re-export detectors for convenience and backward compatibility
pub use dependency::{
    circular_type_deps, cycles, high_coupling, hub_dependency, hub_module, layer_violation,
    package_cycle, vendor_coupling,
};
pub use design::{
    abstractness, barrel_abuse, feature_envy, god_module, orphan_types, primitive_obsession,
    scattered_config, scattered_module, sdp_violation, shared_mutable_state, shotgun_surgery,
    unstable_interface,
};
pub use hygiene::{dead_code, dead_symbols, side_effect_import, test_leakage};
pub use metrics::{complexity, deep_nesting, large_file, lcom, long_params};

/// Ensures all detectors are registered.
/// This is needed to force the linker to include all modules when using the `inventory` crate.
pub fn init() {
    metrics::init();
    dependency::init();
    design::init();
    hygiene::init();
    code_clone::init();
}

use crate::engine::AnalysisContext;

pub trait Detector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell>;

    fn explain(&self, _smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Unknown Problem".to_string(),
            reason: "No explanation provided for this detector".to_string(),
            risks: vec![],
            recommendations: vec![],
        }
    }

    fn render_markdown(
        &self,
        _smells: &[&SmellWithExplanation],
        _severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        String::new()
    }
}
