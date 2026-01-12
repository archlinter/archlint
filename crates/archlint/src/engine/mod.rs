//! Analysis engine and orchestration logic.
//!
//! This module contains the core logic for running the analysis pipeline,
//! including graph building, symbol resolution, and detector execution.

pub mod builder;
pub mod context;
pub mod detector_runner;
pub mod progress;
pub mod runner;

pub use builder::EngineBuilder;
pub use context::AnalysisContext;
pub use detector_runner::DetectorRunner;
pub use runner::AnalysisEngine;
