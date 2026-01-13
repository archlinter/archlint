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

impl Framework {
    pub fn as_preset_name(&self) -> &'static str {
        match self {
            Framework::NestJS => "nestjs",
            Framework::NextJS => "nextjs",
            Framework::Express => "express",
            Framework::React => "react",
            Framework::Angular => "angular",
            Framework::Vue => "vue",
            Framework::TypeORM => "typeorm",
            Framework::Prisma => "prisma",
            Framework::Oclif => "oclif",
        }
    }

    /// Returns true if this framework has a built-in preset.
    pub fn has_builtin_preset(&self) -> bool {
        // Currently all frameworks have built-in presets
        true
    }
}
