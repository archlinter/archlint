pub mod detector;
pub mod preset_loader;
pub mod preset_types;
pub mod presets;

use serde::{Deserialize, Serialize};

/// Supported web and backend frameworks for specialized analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    Generic(String),
}

impl std::fmt::Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Framework::NestJS => write!(f, "NestJS"),
            Framework::NextJS => write!(f, "Next.js"),
            Framework::Express => write!(f, "Express"),
            Framework::React => write!(f, "React"),
            Framework::Angular => write!(f, "Angular"),
            Framework::Vue => write!(f, "Vue"),
            Framework::TypeORM => write!(f, "TypeORM"),
            Framework::Prisma => write!(f, "Prisma"),
            Framework::Oclif => write!(f, "Oclif"),
            Framework::Generic(name) => write!(f, "{}", name),
        }
    }
}
