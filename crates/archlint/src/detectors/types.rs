use crate::snapshot::types::SnapshotSmell;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{Display, EnumMessage, EnumString, IntoStaticStr};

use crate::detectors::ArchSmell;

/// Category for incremental analysis optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectorCategory {
    /// Only analyzes file contents (complexity, deep_nesting, long_params, etc.)
    FileLocal,
    /// Analyzes file imports (layer_violation, vendor_coupling, etc.)
    ImportBased,
    /// Analyzes dependency subgraph (cycles, hub_module, etc.)
    GraphBased,
    /// Requires full graph analysis (dead_code, god_module, etc.)
    Global,
}

/// Defines the specific type of an architectural smell.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SmellType {
    /// Two or more files form a dependency cycle.
    CyclicDependency,
    /// A large group of interconnected cycles.
    CyclicDependencyCluster,
    /// A module with excessive incoming and outgoing dependencies.
    GodModule,
    /// Code that is never imported or executed.
    DeadCode,
    /// An exported symbol (function, class, etc.) that is never used.
    DeadSymbol { name: String, kind: String },
    /// A function with high cyclomatic complexity.
    HighComplexity {
        name: String,
        line: usize,
        complexity: usize,
    },
    /// A file with too many lines of code.
    LargeFile,
    /// An interface that changes frequently despite having many dependents.
    UnstableInterface,
    /// A module that accesses more data from another module than its own.
    FeatureEnvy { most_envied_module: PathBuf },
    /// A change in one module requires many small changes in other modules.
    ShotgunSurgery,
    /// A package that is a central dependency for many parts of the project.
    HubDependency { package: String },

    /// A test file that is imported by non-test code.
    TestLeakage { test_file: PathBuf },
    /// A dependency that violates defined architectural layers.
    LayerViolation {
        from_layer: String,
        to_layer: String,
    },
    /// A stable module depending on a less stable module (Stable Dependencies Principle).
    SdpViolation,

    /// A file that exports too many unrelated symbols.
    BarrelFileAbuse,
    /// Excessive reliance on a specific third-party package.
    VendorCoupling { package: String },
    /// An import that is only executed for its side effects.
    SideEffectImport,
    /// A module that acts as a central hub for many other modules.
    HubModule,

    /// A class where methods don't operate on common fields (Lack of Cohesion of Methods).
    LowCohesion { lcom: usize, class_name: String },
    /// A module that consists of multiple unconnected components.
    ScatteredModule { components: usize },
    /// A module with high coupling to other modules (Coupling Between Objects).
    HighCoupling { cbo: usize },

    /// A dependency cycle between different packages.
    PackageCycle { packages: Vec<String> },
    /// A shared global state that is modified from multiple locations.
    SharedMutableState { symbol: String },

    /// A function with too many levels of nested control structures.
    DeepNesting {
        #[serde(alias = "function")]
        name: String,
        depth: usize,
        line: usize,
    },
    /// A function with an excessively long list of parameters.
    LongParameterList {
        count: usize,
        #[serde(alias = "function")]
        name: String,
    },

    /// Excessive use of primitive types instead of domain-specific objects.
    PrimitiveObsession {
        primitives: usize,
        #[serde(alias = "function")]
        name: String,
    },
    /// A type that is defined but never used.
    OrphanType { name: String },
    /// Circular dependency involving only types (type-only imports).
    CircularTypeDependency,
    /// A module that is neither stable nor abstract enough (Abstractness violation).
    AbstractnessViolation,
    /// Environment variables accessed from many different files.
    ScatteredConfiguration { env_var: String, files_count: usize },
    /// Identical or near-identical code blocks in multiple locations.
    CodeClone {
        clone_hash: String,
        token_count: usize,
    },
    /// Unknown smell type encountered during deserialization.
    Unknown { raw_type: String },
}

/// Represents a smell type that can be configured in the `.archlint.yaml`.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
    EnumString,
    IntoStaticStr,
    EnumMessage,
)]
#[serde(rename_all = "PascalCase")]
#[strum(ascii_case_insensitive)]
pub enum ConfigurableSmellType {
    #[strum(
        to_string = "cycles",
        message = "Cyclic Dependency",
        serialize = "cyclic_dependency",
        serialize = "cyclicdependency"
    )]
    CyclicDependency,
    #[strum(
        to_string = "cycle_clusters",
        message = "Cyclic Dependency Cluster",
        serialize = "cyclic_dependency_cluster",
        serialize = "cyclicdependencycluster"
    )]
    CyclicDependencyCluster,
    #[strum(
        to_string = "god_module",
        message = "God Module",
        serialize = "godmodule"
    )]
    GodModule,
    #[strum(to_string = "dead_code", message = "Dead Code", serialize = "deadcode")]
    DeadCode,
    #[strum(
        to_string = "dead_symbols",
        message = "Dead Symbol",
        serialize = "dead_symbol",
        serialize = "deadsymbol"
    )]
    DeadSymbol,
    #[strum(
        to_string = "complexity",
        message = "High Complexity",
        serialize = "high_complexity",
        serialize = "highcomplexity"
    )]
    HighComplexity,
    #[strum(
        to_string = "large_file",
        message = "Large File",
        serialize = "largefile"
    )]
    LargeFile,
    #[strum(
        to_string = "unstable_interface",
        message = "Unstable Interface",
        serialize = "unstableinterface"
    )]
    UnstableInterface,
    #[strum(
        to_string = "feature_envy",
        message = "Feature Envy",
        serialize = "featureenvy"
    )]
    FeatureEnvy,
    #[strum(
        to_string = "shotgun_surgery",
        message = "Shotgun Surgery",
        serialize = "shotgunsurgery"
    )]
    ShotgunSurgery,
    #[strum(
        to_string = "hub_dependency",
        message = "Hub Dependency",
        serialize = "hubdependency"
    )]
    HubDependency,

    #[strum(
        to_string = "test_leakage",
        message = "Test Leakage",
        serialize = "testleakage"
    )]
    TestLeakage,
    #[strum(
        to_string = "layer_violation",
        message = "Layer Violation",
        serialize = "layerviolation"
    )]
    LayerViolation,
    #[strum(
        to_string = "sdp_violation",
        message = "SDP Violation",
        serialize = "sdpviolation"
    )]
    SdpViolation,

    #[strum(
        to_string = "barrel_file",
        message = "Barrel File Abuse",
        serialize = "barrel_file_abuse",
        serialize = "barrelfileabuse"
    )]
    BarrelFileAbuse,
    #[strum(
        to_string = "vendor_coupling",
        message = "Vendor Coupling",
        serialize = "vendorcoupling"
    )]
    VendorCoupling,
    #[strum(
        to_string = "side_effect_import",
        message = "Side-effect Import",
        serialize = "sideeffectimport"
    )]
    SideEffectImport,
    #[strum(
        to_string = "hub_module",
        message = "Hub Module",
        serialize = "hubmodule"
    )]
    HubModule,

    #[strum(
        to_string = "lcom",
        message = "Low Cohesion (LCOM)",
        serialize = "low_cohesion",
        serialize = "lowcohesion"
    )]
    LowCohesion,
    #[strum(
        to_string = "module_cohesion",
        message = "Scattered Module",
        serialize = "scattered_module",
        serialize = "scatteredmodule"
    )]
    ScatteredModule,
    #[strum(
        to_string = "high_coupling",
        message = "High Coupling",
        serialize = "highcoupling"
    )]
    HighCoupling,

    #[strum(
        to_string = "package_cycles",
        message = "Package Cycle",
        serialize = "package_cycle",
        serialize = "packagecycle"
    )]
    PackageCycle,
    #[strum(
        to_string = "shared_mutable_state",
        message = "Shared Mutable State",
        serialize = "sharedmutablestate"
    )]
    SharedMutableState,

    #[strum(
        to_string = "deep_nesting",
        message = "Deep Nesting",
        serialize = "deepnesting"
    )]
    DeepNesting,
    #[strum(
        to_string = "long_params",
        message = "Long Parameter List",
        serialize = "long_parameter_list",
        serialize = "longparameterlist"
    )]
    LongParameterList,

    #[strum(
        to_string = "primitive_obsession",
        message = "Primitive Obsession",
        serialize = "primitiveobsession"
    )]
    PrimitiveObsession,
    #[strum(
        to_string = "orphan_types",
        message = "Orphan Type",
        serialize = "orphan_type",
        serialize = "orphantype"
    )]
    OrphanType,
    #[strum(
        to_string = "circular_type_deps",
        message = "Circular Type Dependency",
        serialize = "circular_type_dependency",
        serialize = "circulartypedependency"
    )]
    CircularTypeDependency,
    #[strum(
        to_string = "abstractness",
        message = "Abstractness Violation",
        serialize = "abstractness_violation",
        serialize = "abstractnessviolation"
    )]
    AbstractnessViolation,
    #[strum(
        to_string = "scattered_config",
        message = "Scattered Configuration",
        serialize = "scattered_configuration",
        serialize = "scatteredconfiguration"
    )]
    ScatteredConfiguration,
    #[strum(
        to_string = "code_clone",
        message = "Code Clone",
        serialize = "codeclone",
        serialize = "duplicates"
    )]
    CodeClone,
    #[strum(to_string = "unknown", message = "Unknown")]
    Unknown,
}

impl ConfigurableSmellType {
    pub fn to_id(&self) -> &'static str {
        self.into()
    }

    pub fn display_name(&self) -> &'static str {
        self.get_message().unwrap_or_default()
    }
}

impl std::fmt::Display for ConfigurableSmellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl SmellType {
    pub fn category(&self) -> ConfigurableSmellType {
        match self {
            SmellType::CyclicDependency => ConfigurableSmellType::CyclicDependency,
            SmellType::CyclicDependencyCluster => ConfigurableSmellType::CyclicDependencyCluster,
            SmellType::GodModule => ConfigurableSmellType::GodModule,
            SmellType::DeadCode => ConfigurableSmellType::DeadCode,
            SmellType::DeadSymbol { .. } => ConfigurableSmellType::DeadSymbol,
            SmellType::HighComplexity { .. } => ConfigurableSmellType::HighComplexity,
            SmellType::LargeFile => ConfigurableSmellType::LargeFile,
            SmellType::UnstableInterface => ConfigurableSmellType::UnstableInterface,
            SmellType::FeatureEnvy { .. } => ConfigurableSmellType::FeatureEnvy,
            SmellType::ShotgunSurgery => ConfigurableSmellType::ShotgunSurgery,
            SmellType::HubDependency { .. } => ConfigurableSmellType::HubDependency,

            SmellType::TestLeakage { .. } => ConfigurableSmellType::TestLeakage,
            SmellType::LayerViolation { .. } => ConfigurableSmellType::LayerViolation,
            SmellType::SdpViolation => ConfigurableSmellType::SdpViolation,
            SmellType::BarrelFileAbuse => ConfigurableSmellType::BarrelFileAbuse,
            SmellType::VendorCoupling { .. } => ConfigurableSmellType::VendorCoupling,
            SmellType::SideEffectImport => ConfigurableSmellType::SideEffectImport,
            SmellType::HubModule => ConfigurableSmellType::HubModule,
            SmellType::LowCohesion { .. } => ConfigurableSmellType::LowCohesion,
            SmellType::ScatteredModule { .. } => ConfigurableSmellType::ScatteredModule,
            SmellType::HighCoupling { .. } => ConfigurableSmellType::HighCoupling,
            SmellType::PackageCycle { .. } => ConfigurableSmellType::PackageCycle,
            SmellType::SharedMutableState { .. } => ConfigurableSmellType::SharedMutableState,
            SmellType::DeepNesting { .. } => ConfigurableSmellType::DeepNesting,
            SmellType::LongParameterList { .. } => ConfigurableSmellType::LongParameterList,
            SmellType::PrimitiveObsession { .. } => ConfigurableSmellType::PrimitiveObsession,
            SmellType::OrphanType { .. } => ConfigurableSmellType::OrphanType,
            SmellType::CircularTypeDependency => ConfigurableSmellType::CircularTypeDependency,
            SmellType::AbstractnessViolation => ConfigurableSmellType::AbstractnessViolation,
            SmellType::ScatteredConfiguration { .. } => {
                ConfigurableSmellType::ScatteredConfiguration
            }
            SmellType::CodeClone { .. } => ConfigurableSmellType::CodeClone,
            SmellType::Unknown { .. } => ConfigurableSmellType::Unknown,
        }
    }
}

impl From<&SnapshotSmell> for SmellType {
    fn from(smell: &SnapshotSmell) -> Self {
        if let Some(details) = &smell.details {
            return details.clone();
        }

        match smell.smell_type.as_str() {
            "CyclicDependency" | "Cycles" => SmellType::CyclicDependency,
            "CyclicDependencyCluster" => SmellType::CyclicDependencyCluster,
            "GodModule" => SmellType::GodModule,
            "DeadCode" => SmellType::DeadCode,
            "SdpViolation" => SmellType::SdpViolation,
            "LargeFile" => SmellType::LargeFile,
            "UnstableInterface" => SmellType::UnstableInterface,
            "BarrelFileAbuse" => SmellType::BarrelFileAbuse,
            "SideEffectImport" => SmellType::SideEffectImport,
            "HubModule" => SmellType::HubModule,
            "CircularTypeDependency" => SmellType::CircularTypeDependency,
            "AbstractnessViolation" => SmellType::AbstractnessViolation,
            "ShotgunSurgery" => SmellType::ShotgunSurgery,
            unknown => {
                log::warn!("Missing details for complex smell type: {}", unknown);
                SmellType::Unknown {
                    raw_type: unknown.to_string(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::types::{MetricValue, SnapshotSmell};
    use std::collections::HashMap;

    #[test]
    fn test_smell_type_from_snapshot_complex() {
        let mut metrics = HashMap::new();
        metrics.insert("complexity".to_string(), MetricValue::Int(20));

        let snapshot = SnapshotSmell {
            id: "test".to_string(),
            smell_type: "HighComplexity".to_string(),
            severity: "High".to_string(),
            files: vec!["file.ts".to_string()],
            metrics,
            details: Some(SmellType::HighComplexity {
                name: "myFunc".to_string(),
                line: 10,
                complexity: 20,
            }),
            locations: vec![],
        };

        let smell_type = SmellType::from(&snapshot);
        match smell_type {
            SmellType::HighComplexity {
                name,
                line,
                complexity,
            } => {
                assert_eq!(name, "myFunc");
                assert_eq!(line, 10);
                assert_eq!(complexity, 20);
            }
            _ => panic!("Expected HighComplexity, got {:?}", smell_type),
        }
    }

    #[test]
    fn test_smell_type_from_snapshot_hub_dep() {
        let snapshot = SnapshotSmell {
            id: "test".to_string(),
            smell_type: "HubDependency".to_string(),
            severity: "High".to_string(),
            files: vec![],
            metrics: HashMap::new(),
            details: Some(SmellType::HubDependency {
                package: "axios".to_string(),
            }),
            locations: vec![],
        };

        let smell_type = SmellType::from(&snapshot);
        match smell_type {
            SmellType::HubDependency { package } => {
                assert_eq!(package, "axios");
            }
            _ => panic!("Expected HubDependency, got {:?}", smell_type),
        }
    }

    #[test]
    fn test_smell_type_from_snapshot_unknown() {
        let snapshot = SnapshotSmell {
            id: "test".to_string(),
            smell_type: "NewMagicSmell".to_string(),
            severity: "Low".to_string(),
            files: vec![],
            metrics: HashMap::new(),
            details: None,
            locations: vec![],
        };

        let smell_type = SmellType::from(&snapshot);
        match smell_type {
            SmellType::Unknown { raw_type } => {
                assert_eq!(raw_type, "NewMagicSmell");
            }
            _ => panic!("Expected Unknown, got {:?}", smell_type),
        }
    }

    #[test]
    fn test_smell_type_from_snapshot_string_fallback() {
        let snapshot = SnapshotSmell {
            id: "test".to_string(),
            smell_type: "CyclicDependency".to_string(),
            severity: "High".to_string(),
            files: vec!["file.ts".to_string()],
            metrics: HashMap::new(),
            details: None,
            locations: vec![],
        };

        let smell_type = SmellType::from(&snapshot);
        assert_eq!(smell_type, SmellType::CyclicDependency);

        let snapshot_cycles = SnapshotSmell {
            id: "test".to_string(),
            smell_type: "Cycles".to_string(),
            severity: "High".to_string(),
            files: vec!["file.ts".to_string()],
            metrics: HashMap::new(),
            details: None,
            locations: vec![],
        };
        assert_eq!(
            SmellType::from(&snapshot_cycles),
            SmellType::CyclicDependency
        );

        let snapshot_shotgun = SnapshotSmell {
            id: "test".to_string(),
            smell_type: "ShotgunSurgery".to_string(),
            severity: "High".to_string(),
            files: vec!["file.ts".to_string()],
            metrics: HashMap::new(),
            details: None,
            locations: vec![],
        };
        assert_eq!(
            SmellType::from(&snapshot_shotgun),
            SmellType::ShotgunSurgery
        );
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Copy,
    schemars::JsonSchema,
    Display,
    EnumString,
    IntoStaticStr,
)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    pub problem: String,
    pub reason: String,
    pub risks: Vec<String>,
    pub recommendations: Vec<String>,
}

pub type SmellWithExplanation = (ArchSmell, Explanation);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SmellMetric {
    FanIn(usize),
    FanOut(usize),
    Churn(usize),
    CycleLength(usize),
    Complexity(usize),
    Lines(usize),
    InstabilityScore(usize),
    EnvyRatio(f64),
    AvgCoChanges(f64),
    DependentCount(usize),

    Instability(f64),
    InstabilityDiff(f64),
    Lcom(usize),
    MethodCount(usize),
    FieldCount(usize),
    Components(usize),
    Cbo(usize),
    Depth(usize),
    Distance(f64),
    TokenCount(usize),
    CloneInstances(usize),
    ParameterCount(usize),
    PrimitiveCount(usize),
    FilesCount(usize),
    InternalRefs(usize),
    ExternalRefs(usize),
}
