use crate::snapshot::types::{SmellDetails, SnapshotSmell};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
        function: String,
        depth: usize,
        line: usize,
    },
    /// A function with an excessively long list of parameters.
    LongParameterList { count: usize, function: String },

    /// Excessive use of primitive types instead of domain-specific objects.
    PrimitiveObsession { primitives: usize, function: String },
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum ConfigurableSmellType {
    CyclicDependency,
    CyclicDependencyCluster,
    GodModule,
    DeadCode,
    DeadSymbol,
    HighComplexity,
    LargeFile,
    UnstableInterface,
    FeatureEnvy,
    ShotgunSurgery,
    HubDependency,

    TestLeakage,
    LayerViolation,
    SdpViolation,

    BarrelFileAbuse,
    VendorCoupling,
    SideEffectImport,
    HubModule,

    LowCohesion,
    ScatteredModule,
    HighCoupling,

    PackageCycle,
    SharedMutableState,

    DeepNesting,
    LongParameterList,

    PrimitiveObsession,
    OrphanType,
    CircularTypeDependency,
    AbstractnessViolation,
    ScatteredConfiguration,
    CodeClone,
    Unknown,
}

impl std::str::FromStr for ConfigurableSmellType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cyclicdependency" | "cyclic_dependency" => Ok(ConfigurableSmellType::CyclicDependency),
            "cyclicdependencycluster" | "cyclic_dependency_cluster" => {
                Ok(ConfigurableSmellType::CyclicDependencyCluster)
            }
            "godmodule" | "god_module" => Ok(ConfigurableSmellType::GodModule),
            "deadcode" | "dead_code" => Ok(ConfigurableSmellType::DeadCode),
            "deadsymbol" | "dead_symbol" => Ok(ConfigurableSmellType::DeadSymbol),
            "highcomplexity" | "high_complexity" => Ok(ConfigurableSmellType::HighComplexity),
            "largefile" | "large_file" => Ok(ConfigurableSmellType::LargeFile),
            "unstableinterface" | "unstable_interface" => {
                Ok(ConfigurableSmellType::UnstableInterface)
            }
            "featureenvy" | "feature_envy" => Ok(ConfigurableSmellType::FeatureEnvy),
            "shotgunsurgery" | "shotgun_surgery" => Ok(ConfigurableSmellType::ShotgunSurgery),
            "hubdependency" | "hub_dependency" => Ok(ConfigurableSmellType::HubDependency),

            "testleakage" | "test_leakage" => Ok(ConfigurableSmellType::TestLeakage),
            "layerviolation" | "layer_violation" => Ok(ConfigurableSmellType::LayerViolation),
            "sdpviolation" | "sdp_violation" => Ok(ConfigurableSmellType::SdpViolation),
            "barrelfileabuse" | "barrel_file_abuse" => Ok(ConfigurableSmellType::BarrelFileAbuse),
            "vendorcoupling" | "vendor_coupling" => Ok(ConfigurableSmellType::VendorCoupling),
            "sideeffectimport" | "side_effect_import" => {
                Ok(ConfigurableSmellType::SideEffectImport)
            }
            "hubmodule" | "hub_module" => Ok(ConfigurableSmellType::HubModule),
            "lowcohesion" | "low_cohesion" => Ok(ConfigurableSmellType::LowCohesion),
            "scatteredmodule" | "scattered_module" => Ok(ConfigurableSmellType::ScatteredModule),
            "highcoupling" | "high_coupling" => Ok(ConfigurableSmellType::HighCoupling),
            "packagecycle" | "package_cycle" => Ok(ConfigurableSmellType::PackageCycle),
            "sharedmutablestate" | "shared_mutable_state" => {
                Ok(ConfigurableSmellType::SharedMutableState)
            }
            "deepnesting" | "deep_nesting" => Ok(ConfigurableSmellType::DeepNesting),
            "longparameterlist" | "long_parameter_list" => {
                Ok(ConfigurableSmellType::LongParameterList)
            }
            "primitiveobsession" | "primitive_obsession" => {
                Ok(ConfigurableSmellType::PrimitiveObsession)
            }
            "orphantype" | "orphan_type" => Ok(ConfigurableSmellType::OrphanType),
            "circulartypedependency" | "circular_type_dependency" => {
                Ok(ConfigurableSmellType::CircularTypeDependency)
            }
            "abstractnessviolation" | "abstractness_violation" => {
                Ok(ConfigurableSmellType::AbstractnessViolation)
            }
            "scatteredconfiguration" | "scattered_configuration" => {
                Ok(ConfigurableSmellType::ScatteredConfiguration)
            }
            "codeclone" | "code_clone" | "duplicates" => Ok(ConfigurableSmellType::CodeClone),
            "unknown" => Ok(ConfigurableSmellType::Unknown),

            _ => Err(format!("Unknown smell type: {}", s)),
        }
    }
}

impl ConfigurableSmellType {
    pub fn to_id(&self) -> &'static str {
        match self {
            ConfigurableSmellType::CyclicDependency => "cycles",
            ConfigurableSmellType::CyclicDependencyCluster => "cycles",
            ConfigurableSmellType::GodModule => "god_module",
            ConfigurableSmellType::DeadCode => "dead_code",
            ConfigurableSmellType::DeadSymbol => "dead_symbols",
            ConfigurableSmellType::HighComplexity => "complexity",
            ConfigurableSmellType::LargeFile => "large_file",
            ConfigurableSmellType::UnstableInterface => "unstable_interface",
            ConfigurableSmellType::FeatureEnvy => "feature_envy",
            ConfigurableSmellType::ShotgunSurgery => "shotgun_surgery",
            ConfigurableSmellType::HubDependency => "hub_dependency",
            ConfigurableSmellType::TestLeakage => "test_leakage",
            ConfigurableSmellType::LayerViolation => "layer_violation",
            ConfigurableSmellType::SdpViolation => "sdp_violation",
            ConfigurableSmellType::BarrelFileAbuse => "barrel_file",
            ConfigurableSmellType::VendorCoupling => "vendor_coupling",
            ConfigurableSmellType::SideEffectImport => "side_effect_import",
            ConfigurableSmellType::HubModule => "hub_module",
            ConfigurableSmellType::LowCohesion => "lcom",
            ConfigurableSmellType::ScatteredModule => "module_cohesion",
            ConfigurableSmellType::HighCoupling => "high_coupling",
            ConfigurableSmellType::PackageCycle => "package_cycles",
            ConfigurableSmellType::SharedMutableState => "shared_mutable_state",
            ConfigurableSmellType::DeepNesting => "deep_nesting",
            ConfigurableSmellType::LongParameterList => "long_params",
            ConfigurableSmellType::PrimitiveObsession => "primitive_obsession",
            ConfigurableSmellType::OrphanType => "orphan_types",
            ConfigurableSmellType::CircularTypeDependency => "circular_type_deps",
            ConfigurableSmellType::AbstractnessViolation => "abstractness",
            ConfigurableSmellType::ScatteredConfiguration => "scattered_config",
            ConfigurableSmellType::CodeClone => "code_clone",
            ConfigurableSmellType::Unknown => "unknown",
        }
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
        let details = smell.details.as_ref();
        let metric = |name: &str| {
            smell
                .metrics
                .get(name)
                .and_then(|v| v.as_i64())
                .map(|v| v as usize)
                .unwrap_or(0)
        };
        let metric_str = |name: &str| {
            smell
                .metrics
                .get(name)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string()
        };

        macro_rules! d {
            ($variant:ident, $field:ident, $default:expr) => {
                match details {
                    Some(SmellDetails::$variant { $field, .. }) => $field.clone(),
                    _ => $default,
                }
            };
            ($variant:ident, $field:ident) => {
                d!($variant, $field, Default::default())
            };
        }

        match smell.smell_type.as_str() {
            "CyclicDependency" | "Cycles" => SmellType::CyclicDependency,
            "CyclicDependencyCluster" => SmellType::CyclicDependencyCluster,
            "GodModule" => SmellType::GodModule,
            "DeadCode" => SmellType::DeadCode,
            "DeadSymbol" | "DeadSymbols" => SmellType::DeadSymbol {
                name: d!(DeadSymbol, name),
                kind: d!(DeadSymbol, kind, "Symbol".to_string()),
            },
            "HighComplexity" | "Complexity" => SmellType::HighComplexity {
                name: d!(Complexity, function_name),
                line: d!(Complexity, line),
                complexity: metric("complexity"),
            },
            "LayerViolation" => SmellType::LayerViolation {
                from_layer: d!(LayerViolation, from_layer),
                to_layer: d!(LayerViolation, to_layer),
            },
            "HubModule" => SmellType::HubModule,
            "HubDependency" => SmellType::HubDependency {
                package: d!(HubDependency, package),
            },
            "LowCohesion" | "Lcom" => SmellType::LowCohesion {
                lcom: metric("lcom"),
                class_name: d!(LowCohesion, class_name, "unknown".to_string()),
            },
            "SdpViolation" => SmellType::SdpViolation,
            "LargeFile" => SmellType::LargeFile,
            "UnstableInterface" => SmellType::UnstableInterface,
            "FeatureEnvy" => SmellType::FeatureEnvy {
                most_envied_module: PathBuf::from(d!(FeatureEnvy, most_envied_module)),
            },
            "ShotgunSurgery" => SmellType::ShotgunSurgery,
            "TestLeakage" => SmellType::TestLeakage {
                test_file: PathBuf::from(d!(TestLeakage, test_file)),
            },
            "BarrelFileAbuse" => SmellType::BarrelFileAbuse,
            "VendorCoupling" => SmellType::VendorCoupling {
                package: d!(VendorCoupling, package),
            },
            "SideEffectImport" => SmellType::SideEffectImport,
            "ScatteredModule" => SmellType::ScatteredModule {
                components: metric("components"),
            },
            "HighCoupling" => SmellType::HighCoupling { cbo: metric("cbo") },
            "PackageCycle" => SmellType::PackageCycle {
                packages: d!(PackageCycle, packages),
            },
            "SharedMutableState" => SmellType::SharedMutableState {
                symbol: d!(SharedMutableState, symbol),
            },
            "DeepNesting" => SmellType::DeepNesting {
                function: d!(Complexity, function_name),
                depth: metric("depth"),
                line: d!(Complexity, line),
            },
            "LongParameterList" => SmellType::LongParameterList {
                count: metric("parameterCount"),
                function: d!(LongParameterList, function),
            },
            "PrimitiveObsession" => SmellType::PrimitiveObsession {
                primitives: metric("primitiveCount"),
                function: d!(PrimitiveObsession, function),
            },
            "OrphanType" | "OrphanTypes" => SmellType::OrphanType {
                name: d!(OrphanType, name),
            },
            "CircularTypeDependency" | "CircularTypeDependencies" => {
                SmellType::CircularTypeDependency
            }
            "AbstractnessViolation" => SmellType::AbstractnessViolation,
            "ScatteredConfiguration" => SmellType::ScatteredConfiguration {
                env_var: d!(ScatteredConfiguration, env_var),
                files_count: metric("filesCount"),
            },
            "CodeClone" => SmellType::CodeClone {
                clone_hash: metric_str("cloneHash"),
                token_count: metric("tokenCount"),
            },
            unknown => {
                log::warn!("Unknown smell type encountered: {}", unknown);
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
    use crate::snapshot::types::{MetricValue, SmellDetails, SnapshotSmell};
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
            details: Some(SmellDetails::Complexity {
                function_name: "myFunc".to_string(),
                line: 10,
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
            details: Some(SmellDetails::HubDependency {
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::str::FromStr for Severity {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Severity::Low),
            "medium" => Ok(Severity::Medium),
            "high" => Ok(Severity::High),
            "critical" => Ok(Severity::Critical),
            _ => Err(format!("Unknown severity: {}", s)),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Explanation {
    pub problem: String,
    pub reason: String,
    pub risks: Vec<String>,
    pub recommendations: Vec<String>,
}

pub type SmellWithExplanation = (ArchSmell, Explanation);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
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
    InternalRefs(usize),
    ExternalRefs(usize),
}
