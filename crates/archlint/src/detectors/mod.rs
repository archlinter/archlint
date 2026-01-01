pub mod abstractness;
pub mod barrel_abuse;
pub mod circular_type_deps;
pub mod complexity;
pub mod cycles;
pub mod dead_code;
pub mod dead_symbols;
pub mod deep_nesting;
pub mod feature_envy;
pub mod god_module;
pub mod high_coupling;
pub mod hub_dependency;
pub mod hub_module;
pub mod large_file;
pub mod layer_violation;
pub mod lcom;
pub mod long_params;
pub mod orphan_types;
pub mod package_cycle;
pub mod primitive_obsession;
pub mod registry;
pub mod scattered_config;
pub mod scattered_module;
pub mod sdp_violation;
pub mod shared_mutable_state;
pub mod shotgun_surgery;
pub mod side_effect_import;
pub mod test_leakage;
pub mod unstable_interface;
pub mod vendor_coupling;

pub use registry::{DetectorFactory, DetectorInfo, DetectorRegistry};

/// Ensures all detectors are registered.
/// This is needed to force the linker to include all modules when using the `inventory` crate.
pub fn init() {
    complexity::init();
    cycles::init();
    dead_code::init();
    dead_symbols::init();
    god_module::init();
    large_file::init();
    unstable_interface::init();
    feature_envy::init();
    shotgun_surgery::init();
    hub_dependency::init();
    test_leakage::init();
    layer_violation::init();
    sdp_violation::init();
    barrel_abuse::init();
    vendor_coupling::init();
    side_effect_import::init();
    hub_module::init();
    lcom::init();
    scattered_module::init();
    high_coupling::init();
    package_cycle::init();
    shared_mutable_state::init();
    deep_nesting::init();
    long_params::init();
    primitive_obsession::init();
    orphan_types::init();
    circular_type_deps::init();
    abstractness::init();
    scattered_config::init();
}

use crate::config::SeverityConfig;
use crate::engine::AnalysisContext;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub trait Detector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell>;
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
        depth: usize,
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

            _ => Err(format!("Unknown smell type: {}", s)),
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationDetail {
    pub file: PathBuf,
    pub line: usize,
    pub column: Option<usize>,
    pub range: Option<CodeRange>,
    pub description: String,
}

impl LocationDetail {
    pub fn new(file: PathBuf, line: usize, description: String) -> Self {
        Self {
            file,
            line,
            column: None,
            range: None,
            description,
        }
    }

    pub fn with_range(mut self, range: CodeRange) -> Self {
        self.line = range.start_line;
        self.column = Some(range.start_column);
        self.range = Some(range);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub struct CodeRange {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CycleCluster {
    pub files: Vec<PathBuf>,
    pub hotspots: Vec<HotspotInfo>,
    pub critical_edges: Vec<CriticalEdge>,
    pub internal_edges: Vec<LocationDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HotspotInfo {
    pub file: PathBuf,
    pub in_degree: usize,
    pub out_degree: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CriticalEdge {
    pub from: PathBuf,
    pub to: PathBuf,
    pub line: usize,
    pub range: Option<CodeRange>,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchSmell {
    pub smell_type: SmellType,
    pub severity: Severity,
    pub files: Vec<PathBuf>,
    pub metrics: Vec<SmellMetric>,
    pub locations: Vec<LocationDetail>,
    pub cluster: Option<CycleCluster>,
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
}

impl ArchSmell {
    pub fn fan_in(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::FanIn(v) => Some(*v),
            _ => None,
        })
    }

    pub fn fan_out(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::FanOut(v) => Some(*v),
            _ => None,
        })
    }

    pub fn churn(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::Churn(v) => Some(*v),
            _ => None,
        })
    }

    pub fn cycle_length(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::CycleLength(v) => Some(*v),
            _ => None,
        })
    }

    pub fn complexity(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::Complexity(v) => Some(*v),
            _ => None,
        })
    }

    pub fn lines(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::Lines(v) => Some(*v),
            _ => None,
        })
    }

    pub fn instability_score(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::InstabilityScore(v) => Some(*v),
            _ => None,
        })
    }

    pub fn envy_ratio(&self) -> Option<f64> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::EnvyRatio(v) => Some(*v),
            _ => None,
        })
    }

    pub fn avg_co_changes(&self) -> Option<f64> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::AvgCoChanges(v) => Some(*v),
            _ => None,
        })
    }

    pub fn dependant_count(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::DependantCount(v) => Some(*v),
            _ => None,
        })
    }

    pub fn instability(&self) -> Option<f64> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::Instability(v) => Some(*v),
            _ => None,
        })
    }

    pub fn lcom(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::Lcom(v) => Some(*v),
            _ => None,
        })
    }

    pub fn components(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::Components(v) => Some(*v),
            _ => None,
        })
    }

    pub fn cbo(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::Cbo(v) => Some(*v),
            _ => None,
        })
    }

    pub fn depth(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::Depth(v) => Some(*v),
            _ => None,
        })
    }

    /// Get effective severity considering config overrides
    pub fn effective_severity(&self, config: &SeverityConfig) -> Severity {
        config
            .overrides
            .get(&self.smell_type.category())
            .copied()
            .unwrap_or(self.severity)
    }

    /// Calculate weighted score
    pub fn score(&self, config: &SeverityConfig) -> u32 {
        let severity = self.effective_severity(config);
        config.weights.score(&severity)
    }

    pub fn new_cycle(files: Vec<PathBuf>) -> Self {
        let cycle_length = files.len();
        let severity = match cycle_length {
            0..=2 => Severity::Low,
            3..=5 => Severity::Medium,
            6..=10 => Severity::High,
            _ => Severity::Critical,
        };

        Self {
            smell_type: SmellType::CyclicDependency,
            severity,
            files,
            metrics: vec![SmellMetric::CycleLength(cycle_length)],
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_cycle_with_locations(files: Vec<PathBuf>, locations: Vec<LocationDetail>) -> Self {
        let cycle_length = files.len();
        let severity = match cycle_length {
            0..=2 => Severity::Low,
            3..=5 => Severity::Medium,
            _ => Severity::High,
        };

        Self {
            smell_type: SmellType::CyclicDependency,
            severity,
            files,
            metrics: vec![SmellMetric::CycleLength(cycle_length)],
            locations,
            cluster: None,
        }
    }

    pub fn new_cycle_cluster(cluster: CycleCluster) -> Self {
        let cycle_length = cluster.files.len();
        let severity = match cycle_length {
            0..=5 => Severity::Low,
            6..=15 => Severity::Medium,
            16..=30 => Severity::High,
            _ => Severity::Critical,
        };

        Self {
            smell_type: SmellType::CyclicDependencyCluster,
            severity,
            files: cluster.files.clone(),
            metrics: vec![SmellMetric::CycleLength(cycle_length)],
            locations: cluster.internal_edges.clone(),
            cluster: Some(cluster),
        }
    }

    pub fn new_god_module(file: PathBuf, fan_in: usize, fan_out: usize, churn: usize) -> Self {
        let score = fan_in + fan_out + churn / 2;
        let severity = match score {
            0..=30 => Severity::Low,
            31..=60 => Severity::Medium,
            61..=100 => Severity::High,
            _ => Severity::Critical,
        };

        Self {
            smell_type: SmellType::GodModule,
            severity,
            files: vec![file],
            metrics: vec![
                SmellMetric::FanIn(fan_in),
                SmellMetric::FanOut(fan_out),
                SmellMetric::Churn(churn),
            ],
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_dead_code(file: PathBuf) -> Self {
        Self {
            smell_type: SmellType::DeadCode,
            severity: Severity::Low,
            files: vec![file],
            metrics: Vec::new(),
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_dead_symbol(file: PathBuf, name: String, kind: String) -> Self {
        Self {
            smell_type: SmellType::DeadSymbol { name, kind },
            severity: Severity::Low,
            files: vec![file],
            metrics: Vec::new(),
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_dead_symbol_with_line(
        file: PathBuf,
        name: String,
        kind: String,
        line: usize,
    ) -> Self {
        let location = LocationDetail::new(file.clone(), line, format!("{} definition", kind));

        Self {
            smell_type: SmellType::DeadSymbol { name, kind },
            severity: Severity::Low,
            files: vec![file],
            metrics: Vec::new(),
            locations: vec![location],
            cluster: None,
        }
    }

    pub fn new_high_complexity(
        file: PathBuf,
        name: String,
        line: usize,
        complexity: usize,
        threshold: usize,
        range: Option<CodeRange>,
    ) -> Self {
        let severity = if complexity >= threshold * 2 {
            Severity::High
        } else if complexity >= (threshold as f32 * 1.5) as usize {
            Severity::Medium
        } else {
            Severity::Low
        };

        let mut locations = Vec::new();
        if let Some(r) = range {
            locations.push(
                LocationDetail::new(file.clone(), line, format!("Function '{}'", name))
                    .with_range(r),
            );
        }

        Self {
            smell_type: SmellType::HighComplexity { name, line },
            severity,
            files: vec![file],
            metrics: vec![SmellMetric::Complexity(complexity)],
            locations,
            cluster: None,
        }
    }

    pub fn new_large_file(file: PathBuf, lines: usize) -> Self {
        let severity = match lines {
            0..=1500 => Severity::Low,
            1501..=3000 => Severity::Medium,
            _ => Severity::High,
        };

        Self {
            smell_type: SmellType::LargeFile,
            severity,
            files: vec![file],
            metrics: vec![SmellMetric::Lines(lines)],
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_unstable_interface(
        file: PathBuf,
        churn: usize,
        dependants: usize,
        score: usize,
    ) -> Self {
        let severity = match score {
            0..=50 => Severity::Low,
            51..=200 => Severity::Medium,
            201..=500 => Severity::High,
            _ => Severity::Critical,
        };

        Self {
            smell_type: SmellType::UnstableInterface,
            severity,
            files: vec![file],
            metrics: vec![
                SmellMetric::FanIn(dependants),
                SmellMetric::Churn(churn),
                SmellMetric::InstabilityScore(score),
            ],
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_feature_envy(
        file: PathBuf,
        most_envied_module: PathBuf,
        ratio: f64,
        internal_refs: usize,
        external_refs: usize,
    ) -> Self {
        let severity = if ratio >= 5.0 {
            Severity::High
        } else if ratio >= 3.0 {
            Severity::Medium
        } else {
            Severity::Low
        };

        Self {
            smell_type: SmellType::FeatureEnvy { most_envied_module },
            severity,
            files: vec![file],
            metrics: vec![
                SmellMetric::EnvyRatio(ratio),
                SmellMetric::FanIn(external_refs),
                SmellMetric::FanOut(internal_refs),
            ],
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_shotgun_surgery(
        file: PathBuf,
        avg_co_changes: f64,
        co_changed_files: Vec<(PathBuf, usize)>,
    ) -> Self {
        let severity = if avg_co_changes >= 10.0 {
            Severity::Critical
        } else if avg_co_changes >= 5.0 {
            Severity::High
        } else if avg_co_changes >= 3.0 {
            Severity::Medium
        } else {
            Severity::Low
        };

        let mut locations = vec![LocationDetail::new(
            file.clone(),
            0,
            "Primary file (trigger)".to_string(),
        )];

        locations.extend(co_changed_files.iter().map(|(f, count)| {
            LocationDetail::new(f.clone(), 0, format!("Changed together {} times", count))
        }));

        Self {
            smell_type: SmellType::ShotgunSurgery,
            severity,
            files: vec![file],
            metrics: vec![
                SmellMetric::AvgCoChanges(avg_co_changes),
                SmellMetric::DependantCount(co_changed_files.len()),
            ],
            locations,
            cluster: None,
        }
    }

    pub fn new_hub_dependency(package: String, dependant_files: Vec<PathBuf>) -> Self {
        let count = dependant_files.len();
        let severity = if count >= 50 {
            Severity::Critical
        } else if count >= 30 {
            Severity::High
        } else if count >= 15 {
            Severity::Medium
        } else {
            Severity::Low
        };

        let locations = dependant_files
            .iter()
            .map(|f| LocationDetail::new(f.clone(), 0, String::new()))
            .collect();

        Self {
            smell_type: SmellType::HubDependency { package },
            severity,
            files: vec![], // Package is external, not a project file
            metrics: vec![
                SmellMetric::FanIn(count),
                SmellMetric::DependantCount(count),
            ],
            locations,
            cluster: None,
        }
    }

    pub fn new_test_leakage(from: PathBuf, to: PathBuf) -> Self {
        Self {
            smell_type: SmellType::TestLeakage {
                test_file: to.clone(),
            },
            severity: Severity::High,
            files: vec![from.clone()],
            metrics: Vec::new(),
            locations: vec![LocationDetail::new(
                from,
                0,
                format!("Imports test file: {}", to.display()),
            )],
            cluster: None,
        }
    }

    pub fn new_layer_violation(
        from: PathBuf,
        to: PathBuf,
        from_layer: String,
        to_layer: String,
    ) -> Self {
        Self {
            smell_type: SmellType::LayerViolation {
                from_layer,
                to_layer: to_layer.clone(),
            },
            severity: Severity::High,
            files: vec![from.clone()],
            metrics: Vec::new(),
            locations: vec![LocationDetail::new(
                from,
                0,
                format!(
                    "Illegal import of layer '{}' from '{}'",
                    to_layer,
                    to.display()
                ),
            )],
            cluster: None,
        }
    }

    pub fn new_sdp_violation(from: PathBuf, to: PathBuf, from_i: f64, to_i: f64) -> Self {
        Self {
            smell_type: SmellType::SdpViolation,
            severity: Severity::Medium,
            files: vec![from.clone()],
            metrics: vec![
                SmellMetric::Instability(from_i),
                SmellMetric::InstabilityDiff(to_i - from_i),
            ],
            locations: vec![LocationDetail::new(
                from,
                0,
                format!(
                    "Stable module (I={:.2}) depends on unstable module (I={:.2}) from {}",
                    from_i,
                    to_i,
                    to.display()
                ),
            )],
            cluster: None,
        }
    }

    pub fn new_barrel_abuse(path: PathBuf, reexport_count: usize, is_in_cycle: bool) -> Self {
        Self {
            smell_type: SmellType::BarrelFileAbuse,
            severity: if is_in_cycle {
                Severity::High
            } else {
                Severity::Medium
            },
            files: vec![path],
            metrics: vec![SmellMetric::DependantCount(reexport_count)],
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_vendor_coupling(package: String, files: Vec<PathBuf>) -> Self {
        let count = files.len();
        Self {
            smell_type: SmellType::VendorCoupling { package },
            severity: Severity::Medium,
            files: files.clone(),
            metrics: vec![SmellMetric::DependantCount(count)],
            locations: files
                .into_iter()
                .map(|f| LocationDetail::new(f, 0, String::new()))
                .collect(),
            cluster: None,
        }
    }

    pub fn new_side_effect_import(path: PathBuf, source: String) -> Self {
        Self {
            smell_type: SmellType::SideEffectImport,
            severity: Severity::Low,
            files: vec![path.clone()],
            metrics: Vec::new(),
            locations: vec![LocationDetail::new(
                path,
                0,
                format!("Side-effect import of '{}'", source),
            )],
            cluster: None,
        }
    }

    pub fn new_hub_module(path: PathBuf, fan_in: usize, fan_out: usize, complexity: usize) -> Self {
        Self {
            smell_type: SmellType::HubModule,
            severity: Severity::Medium,
            files: vec![path],
            metrics: vec![
                SmellMetric::FanIn(fan_in),
                SmellMetric::FanOut(fan_out),
                SmellMetric::Complexity(complexity),
            ],
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_low_cohesion(path: PathBuf, name: String, lcom: usize) -> Self {
        Self {
            smell_type: SmellType::LowCohesion { lcom },
            severity: Severity::Medium,
            files: vec![path.clone()],
            metrics: vec![SmellMetric::Lcom(lcom)],
            locations: vec![LocationDetail::new(
                path,
                0,
                format!("Class '{}' has low cohesion", name),
            )],
            cluster: None,
        }
    }

    pub fn new_scattered_module(path: PathBuf, components: usize) -> Self {
        Self {
            smell_type: SmellType::ScatteredModule { components },
            severity: Severity::Medium,
            files: vec![path.clone()],
            metrics: vec![SmellMetric::Components(components)],
            locations: vec![LocationDetail::new(
                path,
                0,
                format!("Module has {} unconnected components", components),
            )],
            cluster: None,
        }
    }

    pub fn new_high_coupling(path: PathBuf, cbo: usize) -> Self {
        Self {
            smell_type: SmellType::HighCoupling { cbo },
            severity: Severity::Medium,
            files: vec![path],
            metrics: vec![SmellMetric::Cbo(cbo)],
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_package_cycle(packages: Vec<String>) -> Self {
        Self {
            smell_type: SmellType::PackageCycle { packages },
            severity: Severity::High,
            files: Vec::new(),
            metrics: Vec::new(),
            locations: Vec::new(),
            cluster: None,
        }
    }

    pub fn new_shared_mutable_state(path: PathBuf, symbol: String) -> Self {
        Self {
            smell_type: SmellType::SharedMutableState {
                symbol: symbol.clone(),
            },
            severity: Severity::Medium,
            files: vec![path.clone()],
            metrics: Vec::new(),
            locations: vec![LocationDetail::new(
                path,
                0,
                format!("Exported mutable state '{}'", symbol),
            )],
            cluster: None,
        }
    }

    pub fn new_deep_nesting(path: PathBuf, function: String, depth: usize) -> Self {
        Self {
            smell_type: SmellType::DeepNesting { depth },
            severity: Severity::Low,
            files: vec![path.clone()],
            metrics: vec![SmellMetric::Depth(depth)],
            locations: vec![LocationDetail::new(
                path,
                0,
                format!("Function '{}' is too deeply nested", function),
            )],
            cluster: None,
        }
    }

    pub fn new_long_params(path: PathBuf, function: String, count: usize) -> Self {
        Self {
            smell_type: SmellType::LongParameterList {
                count,
                function: function.clone(),
            },
            severity: Severity::Low,
            files: vec![path.clone()],
            metrics: vec![SmellMetric::DependantCount(count)],
            locations: vec![LocationDetail::new(
                path,
                0,
                format!("Function '{}' has {} parameters", function, count),
            )],
            cluster: None,
        }
    }

    pub fn new_primitive_obsession(path: PathBuf, function: String, primitives: usize) -> Self {
        Self {
            smell_type: SmellType::PrimitiveObsession {
                primitives,
                function: function.clone(),
            },
            severity: Severity::Low,
            files: vec![path.clone()],
            metrics: vec![SmellMetric::DependantCount(primitives)],
            locations: vec![LocationDetail::new(
                path,
                0,
                format!(
                    "Function '{}' has {} primitive parameters",
                    function, primitives
                ),
            )],
            cluster: None,
        }
    }

    pub fn new_orphan_type(path: PathBuf, name: String) -> Self {
        Self {
            smell_type: SmellType::OrphanType { name: name.clone() },
            severity: Severity::Low,
            files: vec![path.clone()],
            metrics: Vec::new(),
            locations: vec![LocationDetail::new(
                path,
                0,
                format!("Type '{}' is never used", name),
            )],
            cluster: None,
        }
    }

    pub fn new_abstractness_violation(path: PathBuf, distance: f64) -> Self {
        Self {
            smell_type: SmellType::AbstractnessViolation,
            severity: Severity::Low,
            files: vec![path.clone()],
            metrics: vec![SmellMetric::Distance(distance)],
            locations: vec![LocationDetail::new(
                path,
                0,
                format!("Distance from main sequence: {:.2}", distance),
            )],
            cluster: None,
        }
    }

    pub fn new_scattered_configuration(env_var: String, files: Vec<PathBuf>) -> Self {
        let count = files.len();
        Self {
            smell_type: SmellType::ScatteredConfiguration {
                env_var: env_var.clone(),
                files_count: count,
            },
            severity: Severity::Low,
            files: files.clone(),
            metrics: vec![SmellMetric::DependantCount(count)],
            locations: files
                .into_iter()
                .map(|f| LocationDetail::new(f, 0, format!("Accesses '{}'", env_var)))
                .collect(),
            cluster: None,
        }
    }
}
