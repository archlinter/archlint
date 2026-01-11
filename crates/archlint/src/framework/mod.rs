pub mod detector;
pub mod preset_loader;
pub mod preset_types;
pub mod presets;

use serde::{Deserialize, Serialize};

/// Supported web and backend frameworks for specialized analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Framework {
    NestJS,
    NextJS,
    Express,
    React,
    Angular,
    Vue,
    TypeORM,
    Prisma,
    Oclif,
}
