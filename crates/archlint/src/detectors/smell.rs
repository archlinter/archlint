use crate::config::SeverityConfig;
use crate::detectors::types::{Severity, SmellMetric, SmellType};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Default)]
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

    pub fn token_count(&self) -> Option<usize> {
        self.metrics.iter().find_map(|m| match m {
            SmellMetric::TokenCount(v) => Some(*v),
            _ => None,
        })
    }

    /// Get effective severity considering config overrides
    pub fn effective_severity(&self, _config: &SeverityConfig) -> Severity {
        self.severity
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
        let location =
            LocationDetail::new(file.clone(), 0, format!("{} '{}' definition", kind, name));
        Self {
            smell_type: SmellType::DeadSymbol { name, kind },
            severity: Severity::Low,
            files: vec![file],
            metrics: Vec::new(),
            locations: vec![location],
            cluster: None,
        }
    }

    pub fn new_dead_symbol_with_line(
        file: PathBuf,
        name: String,
        kind: String,
        line: usize,
    ) -> Self {
        let location = LocationDetail::new(
            file.clone(),
            line,
            format!("{} '{}' definition", kind, name),
        );

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
                LocationDetail::new(
                    file.clone(),
                    line,
                    format!("Function '{}' (complexity: {})", name, complexity),
                )
                .with_range(r),
            );
        }

        Self {
            smell_type: SmellType::HighComplexity {
                name,
                line,
                complexity,
            },
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

    pub fn new_test_leakage(
        from: PathBuf,
        to: PathBuf,
        import_line: usize,
        import_range: Option<CodeRange>,
    ) -> Self {
        let mut location = LocationDetail::new(
            from,
            import_line,
            format!("Imports test file: {}", to.display()),
        );

        if let Some(range) = import_range {
            location = location.with_range(range);
        }

        Self {
            smell_type: SmellType::TestLeakage {
                test_file: to.clone(),
            },
            severity: Severity::High,
            files: vec![location.file.clone()],
            metrics: Vec::new(),
            locations: vec![location],
            cluster: None,
        }
    }

    pub fn new_layer_violation(
        from: PathBuf,
        to: PathBuf,
        from_layer: String,
        to_layer: String,
        import_line: usize,
        import_range: Option<CodeRange>,
    ) -> Self {
        let mut location = LocationDetail::new(
            from,
            import_line,
            format!(
                "Illegal import of layer '{}' from '{}'",
                to_layer,
                to.display()
            ),
        );

        if let Some(range) = import_range {
            location = location.with_range(range);
        }

        Self {
            smell_type: SmellType::LayerViolation {
                from_layer,
                to_layer,
            },
            severity: Severity::High,
            files: vec![location.file.clone()],
            metrics: Vec::new(),
            locations: vec![location],
            cluster: None,
        }
    }

    pub fn new_sdp_violation(
        from: PathBuf,
        to: PathBuf,
        from_i: f64,
        to_i: f64,
        import_line: usize,
        import_range: Option<CodeRange>,
    ) -> Self {
        let mut location = LocationDetail::new(
            from,
            import_line,
            format!(
                "Stable module (I={:.2}) depends on unstable module (I={:.2}) from {}",
                from_i,
                to_i,
                to.display()
            ),
        );

        if let Some(range) = import_range {
            location = location.with_range(range);
        }

        Self {
            smell_type: SmellType::SdpViolation,
            severity: Severity::Medium,
            files: vec![location.file.clone()],
            metrics: vec![
                SmellMetric::Instability(from_i),
                SmellMetric::InstabilityDiff(to_i - from_i),
            ],
            locations: vec![location],
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

    pub fn new_deep_nesting(
        path: PathBuf,
        function: String,
        depth: usize,
        line: usize,
        range: CodeRange,
    ) -> Self {
        Self {
            smell_type: SmellType::DeepNesting { depth },
            severity: Severity::Low,
            files: vec![path.clone()],
            metrics: vec![SmellMetric::Depth(depth)],
            locations: vec![LocationDetail::new(
                path,
                line,
                format!("Function '{}' is too deeply nested", function),
            )
            .with_range(range)],
            cluster: None,
        }
    }

    pub fn new_long_params(
        path: PathBuf,
        function: String,
        count: usize,
        line: usize,
        range: CodeRange,
    ) -> Self {
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
                line,
                format!("Function '{}' has {} parameters", function, count),
            )
            .with_range(range)],
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

    pub fn new_code_clone(
        locations: Vec<LocationDetail>,
        token_count: usize,
        clone_hash: String,
    ) -> Self {
        let files = locations
            .iter()
            .map(|l| l.file.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        Self {
            smell_type: SmellType::CodeClone {
                clone_hash,
                token_count,
            },
            severity: if token_count >= 100 {
                Severity::High
            } else if token_count >= 50 {
                Severity::Medium
            } else {
                Severity::Low
            },
            files,
            metrics: vec![
                SmellMetric::TokenCount(token_count),
                SmellMetric::CloneInstances(locations.len()),
            ],
            locations,
            cluster: None,
        }
    }
}
