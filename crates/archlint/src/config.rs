use crate::detectors::{ConfigurableSmellType, Severity};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub ignore: Vec<String>,

    #[serde(default)]
    pub aliases: HashMap<String, String>,

    #[serde(default)]
    pub thresholds: Thresholds,

    #[serde(default)]
    pub entry_points: Vec<String>,

    #[serde(default)]
    pub detectors: DetectorConfig,

    #[serde(default)]
    pub severity: SeverityConfig,

    #[serde(default)]
    pub watch: WatchConfig,

    #[serde(default)]
    pub framework: Option<String>,

    #[serde(default = "default_true")]
    pub auto_detect_framework: bool,

    #[serde(default = "default_true")]
    pub enable_git: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchConfig {
    #[serde(default = "default_debounce_ms")]
    pub debounce_ms: u64,

    #[serde(default = "default_clear_screen")]
    pub clear_screen: bool,

    #[serde(default)]
    pub ignore: Vec<String>,
}

fn default_debounce_ms() -> u64 {
    300
}

fn default_clear_screen() -> bool {
    false
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            debounce_ms: default_debounce_ms(),
            clear_screen: default_clear_screen(),
            ignore: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeverityConfig {
    #[serde(default)]
    pub overrides: HashMap<ConfigurableSmellType, Severity>,

    #[serde(default = "default_weights")]
    pub weights: SeverityWeights,

    #[serde(default)]
    pub minimum: Option<Severity>,

    #[serde(default)]
    pub minimum_score: Option<u32>,

    #[serde(default)]
    pub grade_thresholds: GradeThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityWeights {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeThresholds {
    pub excellent: f32,
    pub good: f32,
    pub fair: f32,
    pub moderate: f32,
    pub poor: f32,
}

impl Default for GradeThresholds {
    fn default() -> Self {
        Self {
            excellent: 1.0,
            good: 3.0,
            fair: 7.0,
            moderate: 15.0,
            poor: 30.0,
        }
    }
}

fn default_weights() -> SeverityWeights {
    SeverityWeights {
        critical: 100,
        high: 50,
        medium: 20,
        low: 5,
    }
}

impl SeverityWeights {
    pub fn score(&self, severity: &Severity) -> u32 {
        match severity {
            Severity::Critical => self.critical,
            Severity::High => self.high,
            Severity::Medium => self.medium,
            Severity::Low => self.low,
        }
    }
}

impl Default for SeverityWeights {
    fn default() -> Self {
        default_weights()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DetectorConfig {
    #[serde(default)]
    pub enabled: Option<Vec<String>>,
    #[serde(default)]
    pub disabled: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Thresholds {
    #[serde(default = "default_god_module")]
    pub god_module: GodModuleThresholds,

    #[serde(default = "default_complexity")]
    pub complexity: ComplexityThresholds,

    #[serde(default = "default_large_file")]
    pub large_file: LargeFileThresholds,

    #[serde(default = "default_unstable_interface")]
    pub unstable_interface: UnstableInterfaceThresholds,

    #[serde(default = "default_feature_envy")]
    pub feature_envy: FeatureEnvyThresholds,

    #[serde(default = "default_shotgun_surgery")]
    pub shotgun_surgery: ShotgunSurgeryThresholds,

    #[serde(default = "default_hub_dependency")]
    pub hub_dependency: HubDependencyThresholds,

    #[serde(default = "default_test_leakage")]
    pub test_leakage: TestLeakageThresholds,

    #[serde(default = "default_layer_violation")]
    pub layer_violation: LayerViolationThresholds,

    #[serde(default = "default_sdp_violation")]
    pub sdp_violation: SdpThresholds,

    #[serde(default = "default_barrel_file_abuse")]
    pub barrel_file: BarrelFileThresholds,

    #[serde(default = "default_vendor_coupling")]
    pub vendor_coupling: VendorCouplingThresholds,

    #[serde(default = "default_hub_module")]
    pub hub_module: HubModuleThresholds,

    #[serde(default = "default_lcom")]
    pub lcom: LcomThresholds,

    #[serde(default = "default_module_cohesion")]
    pub module_cohesion: ModuleCohesionThresholds,

    #[serde(default = "default_high_coupling")]
    pub high_coupling: HighCouplingThresholds,

    #[serde(default = "default_package_cycles")]
    pub package_cycles: PackageCyclesThresholds,

    #[serde(default = "default_shared_state")]
    pub shared_state: SharedStateThresholds,

    #[serde(default = "default_deep_nesting")]
    pub deep_nesting: DeepNestingThresholds,

    #[serde(default = "default_long_params")]
    pub long_params: LongParamsThresholds,

    #[serde(default = "default_primitive_obsession")]
    pub primitive_obsession: PrimitiveObsessionThresholds,

    #[serde(default = "default_orphan_types")]
    pub orphan_types: OrphanTypesThresholds,

    #[serde(default = "default_abstractness")]
    pub abstractness: AbstractnessThresholds,

    #[serde(default = "default_scattered_config")]
    pub scattered_config: ScatteredConfigThresholds,

    #[serde(default)]
    pub cycles: CyclesThresholds,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GodModuleThresholds {
    #[serde(default = "default_fan_in")]
    pub fan_in: usize,

    #[serde(default = "default_fan_out")]
    pub fan_out: usize,

    #[serde(default = "default_churn")]
    pub churn: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComplexityThresholds {
    #[serde(default = "default_function_complexity")]
    pub function_threshold: usize,

    #[serde(default = "default_file_complexity")]
    pub file_threshold: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LargeFileThresholds {
    #[serde(default = "default_large_file_lines")]
    pub lines: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnstableInterfaceThresholds {
    #[serde(default = "default_min_churn")]
    pub min_churn: usize,
    #[serde(default = "default_min_dependants")]
    pub min_dependants: usize,
    #[serde(default = "default_unstable_score")]
    pub score_threshold: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeatureEnvyThresholds {
    #[serde(default = "default_envy_ratio")]
    pub ratio: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShotgunSurgeryThresholds {
    #[serde(default = "default_min_co_changes")]
    pub min_co_changes: usize,
    #[serde(default = "default_min_frequency")]
    pub min_frequency: usize,
    #[serde(default = "default_lookback_commits")]
    pub lookback_commits: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HubDependencyThresholds {
    #[serde(default = "default_hub_min_dependants")]
    pub min_dependants: usize,
    #[serde(default = "default_hub_ignore_packages")]
    pub ignore_packages: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TestLeakageThresholds {
    #[serde(default = "default_test_patterns")]
    pub test_patterns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LayerViolationThresholds {
    #[serde(default)]
    pub layers: Vec<LayerConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LayerConfig {
    pub name: String,
    pub path: String,
    pub allowed_imports: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SdpThresholds {
    #[serde(default = "default_instability_diff")]
    pub instability_diff: f64,
    #[serde(default = "default_min_fan_total")]
    pub min_fan_total: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BarrelFileThresholds {
    #[serde(default = "default_max_reexports")]
    pub max_reexports: usize,
    #[serde(default = "default_max_transitive_deps")]
    pub max_transitive_deps: usize,
    #[serde(default = "default_flag_if_in_cycle")]
    pub flag_if_in_cycle: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VendorCouplingThresholds {
    #[serde(default = "default_vendor_max_files")]
    pub max_files_per_package: usize,
    #[serde(default = "default_vendor_ignore")]
    pub ignore_packages: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HubModuleThresholds {
    #[serde(default = "default_hub_min_fan_in")]
    pub min_fan_in: usize,
    #[serde(default = "default_hub_min_fan_out")]
    pub min_fan_out: usize,
    #[serde(default = "default_hub_max_complexity")]
    pub max_complexity: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LcomThresholds {
    #[serde(default = "default_max_lcom")]
    pub max_lcom: usize,
    #[serde(default = "default_lcom_min_methods")]
    pub min_methods: usize,
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleCohesionThresholds {
    #[serde(default = "default_cohesion_min_exports")]
    pub min_exports: usize,
    #[serde(default = "default_cohesion_max_components")]
    pub max_components: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HighCouplingThresholds {
    #[serde(default = "default_max_cbo")]
    pub max_cbo: usize,
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackageCyclesThresholds {
    #[serde(default = "default_package_depth")]
    pub package_depth: usize,
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CyclesThresholds {
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SharedStateThresholds {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeepNestingThresholds {
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LongParamsThresholds {
    #[serde(default = "default_max_params")]
    pub max_params: usize,
    #[serde(default = "default_ignore_constructors")]
    pub ignore_constructors: bool,
}

fn default_ignore_constructors() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrimitiveObsessionThresholds {
    #[serde(default = "default_max_primitives")]
    pub max_primitives: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct OrphanTypesThresholds {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbstractnessThresholds {
    #[serde(default = "default_distance_threshold")]
    pub distance_threshold: f64,
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScatteredConfigThresholds {
    #[serde(default = "default_max_config_files")]
    pub max_files: usize,
}

fn default_fan_in() -> usize { 10 }
fn default_fan_out() -> usize { 10 }
fn default_churn() -> usize { 20 }
fn default_function_complexity() -> usize { 10 }
fn default_file_complexity() -> usize { 50 }
fn default_large_file_lines() -> usize { 1000 }
fn default_min_churn() -> usize { 10 }
fn default_min_dependants() -> usize { 5 }
fn default_unstable_score() -> usize { 100 }
fn default_envy_ratio() -> f64 { 3.0 }
fn default_min_co_changes() -> usize { 3 }
fn default_min_frequency() -> usize { 5 }
fn default_lookback_commits() -> usize { 500 }
fn default_hub_min_dependants() -> usize { 20 }
fn default_hub_ignore_packages() -> Vec<String> {
    vec!["react".to_string(), "lodash".to_string(), "typescript".to_string()]
}
fn default_test_patterns() -> Vec<String> {
    vec![
        "**/*.test.ts".to_string(),
        "**/*.test.js".to_string(),
        "**/*.spec.ts".to_string(),
        "**/*.spec.js".to_string(),
        "**/__tests__/**".to_string(),
        "**/__mocks__/**".to_string(),
    ]
}
fn default_instability_diff() -> f64 { 0.3 }
fn default_min_fan_total() -> usize { 5 }
fn default_max_reexports() -> usize { 10 }
fn default_max_transitive_deps() -> usize { 50 }
fn default_flag_if_in_cycle() -> bool { true }
fn default_vendor_max_files() -> usize { 10 }
fn default_hub_min_fan_in() -> usize { 5 }
fn default_hub_min_fan_out() -> usize { 5 }
fn default_hub_max_complexity() -> usize { 5 }
fn default_max_lcom() -> usize { 4 }
fn default_lcom_min_methods() -> usize { 3 }
fn default_cohesion_min_exports() -> usize { 5 }
fn default_cohesion_max_components() -> usize { 2 }
fn default_max_cbo() -> usize { 20 }
fn default_package_depth() -> usize { 2 }
fn default_max_depth() -> usize { 4 }
fn default_max_params() -> usize { 5 }
fn default_max_primitives() -> usize { 3 }
fn default_distance_threshold() -> f64 { 0.85 }
fn default_max_config_files() -> usize { 3 }

fn default_god_module() -> GodModuleThresholds {
    GodModuleThresholds { fan_in: default_fan_in(), fan_out: default_fan_out(), churn: default_churn() }
}
fn default_complexity() -> ComplexityThresholds {
    ComplexityThresholds { function_threshold: default_function_complexity(), file_threshold: default_file_complexity() }
}
fn default_large_file() -> LargeFileThresholds {
    LargeFileThresholds { lines: default_large_file_lines() }
}
fn default_unstable_interface() -> UnstableInterfaceThresholds {
    UnstableInterfaceThresholds { min_churn: default_min_churn(), min_dependants: default_min_dependants(), score_threshold: default_unstable_score() }
}
fn default_feature_envy() -> FeatureEnvyThresholds {
    FeatureEnvyThresholds { ratio: default_envy_ratio() }
}
fn default_shotgun_surgery() -> ShotgunSurgeryThresholds {
    ShotgunSurgeryThresholds { min_co_changes: default_min_co_changes(), min_frequency: default_min_frequency(), lookback_commits: default_lookback_commits() }
}
fn default_hub_dependency() -> HubDependencyThresholds {
    HubDependencyThresholds { min_dependants: default_hub_min_dependants(), ignore_packages: default_hub_ignore_packages() }
}
fn default_test_leakage() -> TestLeakageThresholds {
    TestLeakageThresholds { test_patterns: default_test_patterns() }
}
fn default_layer_violation() -> LayerViolationThresholds { LayerViolationThresholds::default() }
fn default_sdp_violation() -> SdpThresholds {
    SdpThresholds { instability_diff: default_instability_diff(), min_fan_total: default_min_fan_total() }
}
fn default_barrel_file_abuse() -> BarrelFileThresholds {
    BarrelFileThresholds { max_reexports: default_max_reexports(), max_transitive_deps: default_max_transitive_deps(), flag_if_in_cycle: default_flag_if_in_cycle() }
}
fn default_vendor_coupling() -> VendorCouplingThresholds {
    VendorCouplingThresholds { max_files_per_package: default_vendor_max_files(), ignore_packages: default_vendor_ignore() }
}
fn default_vendor_ignore() -> Vec<String> { vec!["react".to_string(), "lodash".to_string()] }
fn default_hub_module() -> HubModuleThresholds {
    HubModuleThresholds { min_fan_in: default_hub_min_fan_in(), min_fan_out: default_hub_min_fan_out(), max_complexity: default_hub_max_complexity() }
}
fn default_lcom() -> LcomThresholds {
    LcomThresholds {
        max_lcom: default_max_lcom(),
        min_methods: default_lcom_min_methods(),
        exclude_patterns: vec!["**/*.controller.ts".to_string()],
    }
}

fn default_module_cohesion() -> ModuleCohesionThresholds {
    ModuleCohesionThresholds {
        min_exports: default_cohesion_min_exports(),
        max_components: default_cohesion_max_components(),
    }
}

fn default_high_coupling() -> HighCouplingThresholds {
    HighCouplingThresholds {
        max_cbo: default_max_cbo(),
        exclude_patterns: Vec::new(),
    }
}

fn default_package_cycles() -> PackageCyclesThresholds {
    PackageCyclesThresholds {
        package_depth: default_package_depth(),
        exclude_patterns: Vec::new(),
    }
}
fn default_shared_state() -> SharedStateThresholds { SharedStateThresholds::default() }
fn default_deep_nesting() -> DeepNestingThresholds {
    DeepNestingThresholds { max_depth: default_max_depth() }
}
fn default_long_params() -> LongParamsThresholds {
    LongParamsThresholds {
        max_params: default_max_params(),
        ignore_constructors: default_ignore_constructors(),
    }
}
fn default_primitive_obsession() -> PrimitiveObsessionThresholds {
    PrimitiveObsessionThresholds { max_primitives: default_max_primitives() }
}
fn default_orphan_types() -> OrphanTypesThresholds { OrphanTypesThresholds::default() }
fn default_abstractness() -> AbstractnessThresholds {
    AbstractnessThresholds {
        distance_threshold: default_distance_threshold(),
        exclude_patterns: vec![
            "**/*.dto.ts".to_string(),
            "**/*.interface.ts".to_string(),
            "**/*.types.ts".to_string(),
        ],
    }
}

fn default_cycles_thresholds() -> CyclesThresholds {
    CyclesThresholds {
        exclude_patterns: vec!["**/*.entity.ts".to_string()],
    }
}

fn default_scattered_config() -> ScatteredConfigThresholds {
    ScatteredConfigThresholds { max_files: default_max_config_files() }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            god_module: default_god_module(),
            complexity: default_complexity(),
            large_file: default_large_file(),
            unstable_interface: default_unstable_interface(),
            feature_envy: default_feature_envy(),
            shotgun_surgery: default_shotgun_surgery(),
            hub_dependency: default_hub_dependency(),
            test_leakage: default_test_leakage(),
            layer_violation: default_layer_violation(),
            sdp_violation: default_sdp_violation(),
            barrel_file: default_barrel_file_abuse(),
            vendor_coupling: default_vendor_coupling(),
            hub_module: default_hub_module(),
            lcom: default_lcom(),
            module_cohesion: default_module_cohesion(),
            high_coupling: default_high_coupling(),
            package_cycles: default_package_cycles(),
            shared_state: default_shared_state(),
            deep_nesting: default_deep_nesting(),
            long_params: default_long_params(),
            primitive_obsession: default_primitive_obsession(),
            orphan_types: default_orphan_types(),
            abstractness: default_abstractness(),
            scattered_config: default_scattered_config(),
            cycles: default_cycles_thresholds(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ignore: vec![
                "**/*.test.ts".to_string(),
                "**/*.test.js".to_string(),
                "**/*.spec.ts".to_string(),
                "**/*.spec.js".to_string(),
                "**/__tests__/**".to_string(),
                "**/__mocks__/**".to_string(),
                "**/test/**".to_string(),
                "**/tests/**".to_string(),
                "**/__fixtures__/**".to_string(),
                "**/*.mock.ts".to_string(),
                "**/*.mock.js".to_string(),
            ],
            aliases: HashMap::new(),
            thresholds: Thresholds::default(),
            entry_points: Vec::new(),
            detectors: DetectorConfig::default(),
            severity: SeverityConfig::default(),
            watch: WatchConfig::default(),
            framework: None,
            auto_detect_framework: true,
            enable_git: true,
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn load_or_default(path: Option<&Path>, project_root: Option<&Path>) -> Result<Self> {
        if let Some(p) = path {
            return Self::load(p);
        }

        let filenames = [
            ".archlint.yaml",
            ".archlint.yml",
            "archlint.yaml",
            "archlint.yml",
        ];

        for filename in filenames {
            let p = project_root
                .map(|root| root.join(filename))
                .unwrap_or_else(|| PathBuf::from(filename));

            if p.exists() {
                return Self::load(p);
            }
        }

        Ok(Self::default())
    }
}
