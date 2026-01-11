use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SmellType {
    CyclicDependency,
    CyclicDependencyCluster,
    GodModule,
    DeadCode,
    DeadSymbol {
        name: String,
        kind: String,
    },
    HighComplexity {
        name: String,
        line: usize,
        complexity: usize,
    },
    LargeFile,
    UnstableInterface,
    FeatureEnvy {
        most_envied_module: PathBuf,
    },
    ShotgunSurgery,
    HubDependency {
        package: String,
    },

    TestLeakage {
        test_file: PathBuf,
    },
    LayerViolation {
        from_layer: String,
        to_layer: String,
    },
    SdpViolation,

    BarrelFileAbuse,
    VendorCoupling {
        package: String,
    },
    SideEffectImport,
    HubModule,

    LowCohesion {
        lcom: usize,
    },
    ScatteredModule {
        components: usize,
    },
    HighCoupling {
        cbo: usize,
    },

    PackageCycle {
        packages: Vec<String>,
    },
    SharedMutableState {
        symbol: String,
    },

    DeepNesting {
        function: String,
        depth: usize,
        line: usize,
    },
    LongParameterList {
        count: usize,
        function: String,
    },

    PrimitiveObsession {
        primitives: usize,
        function: String,
    },
    OrphanType {
        name: String,
    },
    CircularTypeDependency,
    AbstractnessViolation,
    ScatteredConfiguration {
        env_var: String,
        files_count: usize,
    },
    CodeClone {
        clone_hash: String,
        token_count: usize,
    },
}

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

    // Phase 1
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

            _ => Err(format!("Unknown smell type: {}", s)),
        }
    }
}

impl ConfigurableSmellType {
    pub fn to_id(&self) -> &'static str {
        match self {
            ConfigurableSmellType::CyclicDependency => "cycles",
            ConfigurableSmellType::CyclicDependencyCluster => "cycles_cluster",
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
            ConfigurableSmellType::SharedMutableState => "shared_state",
            ConfigurableSmellType::DeepNesting => "deep_nesting",
            ConfigurableSmellType::LongParameterList => "long_params",
            ConfigurableSmellType::PrimitiveObsession => "primitive_obsession",
            ConfigurableSmellType::OrphanType => "orphan_types",
            ConfigurableSmellType::CircularTypeDependency => "circular_type_deps",
            ConfigurableSmellType::AbstractnessViolation => "abstractness",
            ConfigurableSmellType::ScatteredConfiguration => "scattered_config",
            ConfigurableSmellType::CodeClone => "code_clone",
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
    DependantCount(usize),

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
}
