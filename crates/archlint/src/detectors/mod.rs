pub mod code_clone;
pub mod registry;
pub mod smell;
pub mod types;

pub mod dependency;
pub mod design;
pub mod hygiene;
pub mod metrics;

pub use registry::{DetectorFactory, DetectorInfo, DetectorRegistry};
pub use smell::*;
pub use types::*;

// Re-export detectors for convenience and backward compatibility
pub use dependency::*;
pub use design::*;
pub use hygiene::*;
pub use metrics::*;

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
}

