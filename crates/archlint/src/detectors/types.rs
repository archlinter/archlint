use crate::snapshot::types::SnapshotSmell;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{Display, EnumMessage, EnumProperty, EnumString, IntoStaticStr};

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
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    strum::EnumDiscriminants,
)]
#[strum_discriminants(derive(
    Serialize,
    Deserialize,
    EnumString,
    IntoStaticStr,
    EnumMessage,
    EnumProperty,
    Hash,
    PartialOrd,
    Ord,
))]
#[strum_discriminants(name(SmellKind))]
#[strum_discriminants(serde(rename_all = "PascalCase"))]
#[strum_discriminants(strum(ascii_case_insensitive))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SmellType {
    /// Two or more files form a dependency cycle.
    #[strum_discriminants(strum(
        to_string = "cyclic_dependency",
        message = "Cyclic Dependency",
        serialize = "cyclic_dependency",
        serialize = "cyclicdependency",
        serialize = "cycles",
        props(
            category = "GraphBased",
            description = "Detects circular dependencies between modules"
        )
    ))]
    CyclicDependency,

    /// A large group of interconnected cycles.
    #[strum_discriminants(strum(
        to_string = "cycle_clusters",
        message = "Cyclic Dependency Cluster",
        serialize = "cyclic_dependency_cluster",
        serialize = "cyclicdependencycluster",
        props(
            category = "GraphBased",
            description = "A large group of interconnected dependency cycles"
        )
    ))]
    CyclicDependencyCluster,

    /// A module with excessive incoming and outgoing dependencies.
    #[strum_discriminants(strum(
        to_string = "god_module",
        message = "God Module",
        serialize = "godmodule",
        props(
            category = "Global",
            description = "A module with excessive incoming and outgoing dependencies"
        )
    ))]
    GodModule,

    /// Code that is never imported or executed.
    #[strum_discriminants(strum(
        to_string = "dead_code",
        message = "Dead Code",
        serialize = "deadcode",
        props(
            category = "Global",
            description = "Code that is never imported or executed"
        )
    ))]
    DeadCode,

    /// An exported symbol (function, class, etc.) that is never used.
    #[strum_discriminants(strum(
        to_string = "dead_symbols",
        message = "Dead Symbol",
        serialize = "dead_symbol",
        serialize = "deadsymbol",
        props(
            category = "Global",
            description = "An exported symbol that is never used"
        )
    ))]
    DeadSymbol { name: String, kind: String },

    /// A function with high cyclomatic complexity.
    #[strum_discriminants(strum(
        to_string = "cyclomatic_complexity",
        message = "High Cyclomatic Complexity",
        serialize = "cyclomatic_complexity",
        serialize = "high_cyclomatic_complexity",
        serialize = "complexity",
        props(
            category = "FileLocal",
            description = "A function with high cyclomatic complexity"
        )
    ))]
    HighCyclomaticComplexity {
        name: String,
        line: usize,
        complexity: usize,
    },

    /// A function with high cognitive complexity.
    #[strum_discriminants(strum(
        to_string = "cognitive_complexity",
        message = "High Cognitive Complexity",
        serialize = "cognitive_complexity",
        serialize = "high_cognitive_complexity",
        props(
            category = "FileLocal",
            description = "A function with high cognitive complexity (how hard it is to understand)"
        )
    ))]
    HighCognitiveComplexity {
        name: String,
        line: usize,
        complexity: usize,
    },

    /// A file with too many lines of code.
    #[strum_discriminants(strum(
        to_string = "large_file",
        message = "Large File",
        serialize = "largefile",
        props(
            category = "FileLocal",
            description = "A file with too many lines of code"
        )
    ))]
    LargeFile,

    /// An interface that changes frequently despite having many dependents.
    #[strum_discriminants(strum(
        to_string = "unstable_interface",
        message = "Unstable Interface",
        serialize = "unstableinterface",
        props(
            category = "Global",
            description = "An interface that changes frequently despite having many dependents"
        )
    ))]
    UnstableInterface,

    /// A module that accesses more data from another module than its own.
    #[strum_discriminants(strum(
        to_string = "feature_envy",
        message = "Feature Envy",
        serialize = "featureenvy",
        props(
            category = "GraphBased",
            description = "A module that accesses more data from another module than its own"
        )
    ))]
    FeatureEnvy { most_envied_module: PathBuf },

    /// A change in one module requires many small changes in other modules.
    #[strum_discriminants(strum(
        to_string = "shotgun_surgery",
        message = "Shotgun Surgery",
        serialize = "shotgunsurgery",
        props(
            category = "Global",
            description = "A change in one module requires many small changes in other modules"
        )
    ))]
    ShotgunSurgery,

    /// A package that is a central dependency for many parts of the project.
    #[strum_discriminants(strum(
        to_string = "hub_dependency",
        message = "Hub Dependency",
        serialize = "hubdependency",
        props(
            category = "GraphBased",
            description = "A package that is a central dependency for many parts of the project"
        )
    ))]
    HubDependency { package: String },

    /// A test file that is imported by non-test code.
    #[strum_discriminants(strum(
        to_string = "test_leakage",
        message = "Test Leakage",
        serialize = "testleakage",
        props(
            category = "ImportBased",
            description = "A test file that is imported by non-test code"
        )
    ))]
    TestLeakage { test_file: PathBuf },

    /// A dependency that violates defined architectural layers.
    #[strum_discriminants(strum(
        to_string = "layer_violation",
        message = "Layer Violation",
        serialize = "layerviolation",
        props(
            category = "ImportBased",
            description = "A dependency that violates defined architectural layers"
        )
    ))]
    LayerViolation {
        from_layer: String,
        to_layer: String,
    },

    /// A stable module depending on a less stable module (Stable Dependencies Principle).
    #[strum_discriminants(strum(
        to_string = "sdp_violation",
        message = "SDP Violation",
        serialize = "sdpviolation",
        props(
            category = "Global",
            description = "A stable module depending on a less stable module (Stable Dependencies Principle)"
        )
    ))]
    SdpViolation,

    /// A file that exports too many unrelated symbols.
    #[strum_discriminants(strum(
        to_string = "barrel_file",
        message = "Barrel File Abuse",
        serialize = "barrel_file_abuse",
        serialize = "barrelfileabuse",
        props(
            category = "ImportBased",
            description = "Detects excessive use of barrel files (index.ts) that inflate the dependency graph"
        )
    ))]
    BarrelFileAbuse,

    /// Excessive reliance on a specific third-party package.
    #[strum_discriminants(strum(
        to_string = "vendor_coupling",
        message = "Vendor Coupling",
        serialize = "vendorcoupling",
        props(
            category = "ImportBased",
            description = "Excessive reliance on a specific third-party package"
        )
    ))]
    VendorCoupling { package: String },

    /// An import that is only executed for its side effects.
    #[strum_discriminants(strum(
        to_string = "side_effect_import",
        message = "Side-effect Import",
        serialize = "sideeffectimport",
        props(
            category = "ImportBased",
            description = "An import that is only executed for its side effects"
        )
    ))]
    SideEffectImport,

    /// A module that acts as a central hub for many other modules.
    #[strum_discriminants(strum(
        to_string = "hub_module",
        message = "Hub Module",
        serialize = "hubmodule",
        props(
            category = "GraphBased",
            description = "A module that acts as a central hub for many other modules"
        )
    ))]
    HubModule,

    /// A class where methods don't operate on common fields (Lack of Cohesion of Methods).
    #[strum_discriminants(strum(
        to_string = "lcom",
        message = "Low Cohesion (LCOM)",
        serialize = "low_cohesion",
        serialize = "lowcohesion",
        props(
            category = "FileLocal",
            description = "Detects classes with low cohesion where methods don't share common fields"
        )
    ))]
    LowCohesion { lcom: usize, class_name: String },

    /// A module that consists of multiple unconnected components.
    #[strum_discriminants(strum(
        to_string = "module_cohesion",
        message = "Scattered Module",
        serialize = "scattered_module",
        serialize = "scatteredmodule",
        props(
            category = "Global",
            description = "A module that consists of multiple unconnected components"
        )
    ))]
    ScatteredModule { components: usize },

    /// A module with high coupling to other modules (Coupling Between Objects).
    #[strum_discriminants(strum(
        to_string = "high_coupling",
        message = "High Coupling",
        serialize = "highcoupling",
        props(
            category = "FileLocal",
            description = "A module with high coupling to other modules (Coupling Between Objects)"
        )
    ))]
    HighCoupling { cbo: usize },

    /// A dependency cycle between different packages.
    #[strum_discriminants(strum(
        to_string = "package_cycles",
        message = "Package Cycle",
        serialize = "package_cycle",
        serialize = "packagecycle",
        props(
            category = "GraphBased",
            description = "A dependency cycle between different packages"
        )
    ))]
    PackageCycle { packages: Vec<String> },

    /// A shared global state that is modified from multiple locations.
    #[strum_discriminants(strum(
        to_string = "shared_mutable_state",
        message = "Shared Mutable State",
        serialize = "sharedmutablestate",
        props(
            category = "Global",
            description = "A shared global state that is modified from multiple locations"
        )
    ))]
    SharedMutableState { symbol: String },

    /// A function with too many levels of nested control structures.
    #[strum_discriminants(strum(
        to_string = "deep_nesting",
        message = "Deep Nesting",
        serialize = "deepnesting",
        props(
            category = "FileLocal",
            description = "A function with too many levels of nested control structures"
        )
    ))]
    DeepNesting {
        #[serde(alias = "function")]
        name: String,
        depth: usize,
        line: usize,
    },

    /// A function with an excessively long list of parameters.
    #[strum_discriminants(strum(
        to_string = "long_params",
        message = "Long Parameter List",
        serialize = "long_parameter_list",
        serialize = "longparameterlist",
        props(
            category = "FileLocal",
            description = "A function with an excessively long list of parameters"
        )
    ))]
    LongParameterList {
        count: usize,
        #[serde(alias = "function")]
        name: String,
    },

    /// Excessive use of primitive types instead of domain-specific objects.
    #[strum_discriminants(strum(
        to_string = "primitive_obsession",
        message = "Primitive Obsession",
        serialize = "primitiveobsession",
        props(
            category = "FileLocal",
            description = "Excessive use of primitive types instead of domain-specific objects"
        )
    ))]
    PrimitiveObsession {
        primitives: usize,
        #[serde(alias = "function")]
        name: String,
    },

    /// A type that is defined but never used.
    #[strum_discriminants(strum(
        to_string = "orphan_types",
        message = "Orphan Type",
        serialize = "orphan_type",
        serialize = "orphantype",
        props(
            category = "Global",
            description = "A type that is defined but never used"
        )
    ))]
    OrphanType { name: String },

    /// Circular dependency involving only types (type-only imports).
    #[strum_discriminants(strum(
        to_string = "circular_type_deps",
        message = "Circular Type Dependency",
        serialize = "circular_type_dependency",
        serialize = "circulartypedependency",
        props(
            category = "GraphBased",
            description = "Circular dependency involving only types (type-only imports)"
        )
    ))]
    CircularTypeDependency,

    /// A module that is neither stable nor abstract enough (Abstractness violation).
    #[strum_discriminants(strum(
        to_string = "abstractness",
        message = "Abstractness Violation",
        serialize = "abstractness_violation",
        serialize = "abstractnessviolation",
        props(
            category = "Global",
            description = "A module that is neither stable nor abstract enough (Abstractness violation)"
        )
    ))]
    AbstractnessViolation,

    /// Environment variables accessed from many different files.
    #[strum_discriminants(strum(
        to_string = "scattered_config",
        message = "Scattered Configuration",
        serialize = "scattered_configuration",
        serialize = "scatteredconfiguration",
        props(
            category = "Global",
            description = "Environment variables accessed from many different files"
        )
    ))]
    ScatteredConfiguration { env_var: String, files_count: usize },

    /// Identical or near-identical code blocks in multiple locations.
    #[strum_discriminants(strum(
        to_string = "code_clone",
        message = "Code Clone",
        serialize = "codeclone",
        serialize = "duplicates",
        props(
            category = "Global",
            description = "Identical or near-identical code blocks in multiple locations"
        )
    ))]
    CodeClone {
        clone_hash: String,
        token_count: usize,
    },

    /// Unknown smell type encountered during deserialization.
    #[strum_discriminants(strum(to_string = "unknown", message = "Unknown"))]
    Unknown { raw_type: String },
}

impl SmellKind {
    pub fn to_id(&self) -> &'static str {
        self.into()
    }

    pub fn display_name(&self) -> &'static str {
        self.get_message().unwrap_or_default()
    }

    pub fn default_category(&self) -> DetectorCategory {
        use EnumProperty;
        match self.get_str("category") {
            Some("FileLocal") => DetectorCategory::FileLocal,
            Some("ImportBased") => DetectorCategory::ImportBased,
            Some("GraphBased") => DetectorCategory::GraphBased,
            Some("Global") => DetectorCategory::Global,
            _ => DetectorCategory::Global,
        }
    }
}

impl std::fmt::Display for SmellKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl SmellType {
    pub fn category(&self) -> SmellKind {
        SmellKind::from(self)
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
            "HighComplexity"
            | "Complexity"
            | "HighCyclomaticComplexity"
            | "CyclomaticComplexity" => {
                SmellType::HighCyclomaticComplexity {
                    name: String::new(), // Will be filled by details if present
                    line: 0,
                    complexity: 0,
                }
            }
            "HighCognitiveComplexity" | "CognitiveComplexity" => {
                SmellType::HighCognitiveComplexity {
                    name: String::new(),
                    line: 0,
                    complexity: 0,
                }
            }
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
        metrics.insert("cyclomaticComplexity".to_string(), MetricValue::Int(20));

        let snapshot = SnapshotSmell {
            id: "test".to_string(),
            smell_type: "HighCyclomaticComplexity".to_string(),
            severity: "High".to_string(),
            files: vec!["file.ts".to_string()],
            metrics,
            details: Some(SmellType::HighCyclomaticComplexity {
                name: "myFunc".to_string(),
                line: 10,
                complexity: 20,
            }),
            locations: vec![],
        };

        let smell_type = SmellType::from(&snapshot);
        match smell_type {
            SmellType::HighCyclomaticComplexity {
                name,
                line,
                complexity,
            } => {
                assert_eq!(name, "myFunc");
                assert_eq!(line, 10);
                assert_eq!(complexity, 20);
            }
            _ => panic!("Expected HighCyclomaticComplexity, got {:?}", smell_type),
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
    CyclomaticComplexity(usize),
    CognitiveComplexity(usize),
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
    Abstractness(f64),
    TokenCount(usize),
    CloneInstances(usize),
    ParameterCount(usize),
    PrimitiveCount(usize),
    FilesCount(usize),
    InternalRefs(usize),
    ExternalRefs(usize),
}
