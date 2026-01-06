use crate::api::result::{ScanResult, SmellWithExplanation};
use crate::detectors::{ArchSmell, SmellMetric, SmellType};
use crate::snapshot::id::generate_smell_id;
use crate::snapshot::types::{
    MetricValue, SmellDetails, Snapshot, SnapshotSmell, SnapshotSummary, SCHEMA_VERSION,
};
use chrono::Utc;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct SnapshotGenerator {
    project_root: PathBuf,
    include_commit: bool,
}

impl SnapshotGenerator {
    pub fn new(project_root: PathBuf) -> Self {
        let project_root = project_root.canonicalize().unwrap_or(project_root);
        Self {
            project_root,
            include_commit: true,
        }
    }

    pub fn with_commit(mut self, include: bool) -> Self {
        self.include_commit = include;
        self
    }

    pub fn generate(&self, scan_result: &ScanResult) -> Snapshot {
        let commit = if self.include_commit {
            get_git_commit(&self.project_root)
        } else {
            None
        };

        let smells: Vec<SnapshotSmell> = scan_result
            .smells
            .iter()
            .map(|s| self.convert_smell(&s.smell))
            .collect();

        Snapshot {
            schema_version: SCHEMA_VERSION,
            archlint_version: env!("CARGO_PKG_VERSION").to_string(),
            generated_at: Utc::now().to_rfc3339(),
            commit,
            smells,
            summary: self.build_summary(scan_result),
            grade: format!("{:?}", scan_result.grade),
        }
    }

    fn convert_smell(&self, smell: &ArchSmell) -> SnapshotSmell {
        let id = generate_smell_id(smell, &self.project_root);

        let files: Vec<String> = smell
            .files
            .iter()
            .filter_map(|f| f.strip_prefix(&self.project_root).ok())
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .collect();

        let metrics = self.extract_metrics(smell);
        let details = self.extract_details(smell);

        SnapshotSmell {
            id,
            smell_type: smell_type_to_string(&smell.smell_type),
            severity: format!("{:?}", smell.severity),
            files,
            metrics,
            details,
            locations: self.extract_locations(smell),
        }
    }

    fn extract_locations(&self, smell: &ArchSmell) -> Vec<crate::snapshot::types::Location> {
        smell
            .locations
            .iter()
            .map(|loc| {
                let file = loc
                    .file
                    .strip_prefix(&self.project_root)
                    .map(|p| p.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| loc.file.to_string_lossy().replace('\\', "/"));

                crate::snapshot::types::Location {
                    file,
                    line: loc.line,
                    column: loc.column,
                    description: if loc.description.is_empty() {
                        None
                    } else {
                        Some(loc.description.clone())
                    },
                }
            })
            .collect()
    }

    fn extract_metrics(&self, smell: &ArchSmell) -> HashMap<String, MetricValue> {
        let mut metrics = HashMap::new();

        for metric in &smell.metrics {
            match metric {
                SmellMetric::FanIn(v) => {
                    metrics.insert("fanIn".into(), MetricValue::Int(*v as i64));
                }
                SmellMetric::FanOut(v) => {
                    metrics.insert("fanOut".into(), MetricValue::Int(*v as i64));
                }
                SmellMetric::CycleLength(v) => {
                    metrics.insert("cycleLength".into(), MetricValue::Int(*v as i64));
                }
                SmellMetric::Complexity(v) => {
                    metrics.insert("complexity".into(), MetricValue::Int(*v as i64));
                }
                SmellMetric::Lcom(v) => {
                    metrics.insert("lcom".into(), MetricValue::Int(*v as i64));
                }
                SmellMetric::Cbo(v) => {
                    metrics.insert("cbo".into(), MetricValue::Int(*v as i64));
                }
                SmellMetric::Instability(v) => {
                    metrics.insert("instability".into(), MetricValue::Float(*v));
                }
                SmellMetric::Lines(v) => {
                    metrics.insert("lines".into(), MetricValue::Int(*v as i64));
                }
                SmellMetric::InstabilityScore(v) => {
                    metrics.insert("instabilityScore".into(), MetricValue::Int(*v as i64));
                }
                SmellMetric::EnvyRatio(v) => {
                    metrics.insert("envyRatio".into(), MetricValue::Float(*v));
                }
                SmellMetric::AvgCoChanges(v) => {
                    metrics.insert("avgCoChanges".into(), MetricValue::Float(*v));
                }
                SmellMetric::DependantCount(v) => {
                    metrics.insert("dependantCount".into(), MetricValue::Int(*v as i64));
                }
                SmellMetric::Components(v) => {
                    metrics.insert("components".into(), MetricValue::Int(*v as i64));
                }
                SmellMetric::Depth(v) => {
                    metrics.insert("depth".into(), MetricValue::Int(*v as i64));
                }
                _ => {}
            }
        }

        metrics
    }

    fn extract_details(&self, smell: &ArchSmell) -> Option<SmellDetails> {
        match &smell.smell_type {
            SmellType::CyclicDependency | SmellType::CyclicDependencyCluster => {
                let path: Vec<String> = smell
                    .files
                    .iter()
                    .filter_map(|f| f.strip_prefix(&self.project_root).ok())
                    .map(|p| p.to_string_lossy().replace('\\', "/"))
                    .collect();

                Some(SmellDetails::Cycle { path })
            }

            SmellType::LayerViolation {
                from_layer,
                to_layer,
            } => {
                let import_file = smell
                    .files
                    .first()
                    .and_then(|f| f.strip_prefix(&self.project_root).ok())
                    .map(|p| p.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_default();

                Some(SmellDetails::LayerViolation {
                    from_layer: from_layer.clone(),
                    to_layer: to_layer.clone(),
                    import_file,
                })
            }

            SmellType::DeadSymbol { name, kind } => Some(SmellDetails::DeadSymbol {
                name: name.clone(),
                kind: kind.clone(),
            }),

            SmellType::HighComplexity { name, line, .. } => Some(SmellDetails::Complexity {
                function_name: name.clone(),
                line: *line,
            }),

            SmellType::FeatureEnvy { most_envied_module } => Some(SmellDetails::FeatureEnvy {
                most_envied_module: most_envied_module.to_string_lossy().to_string(),
            }),

            SmellType::TestLeakage { test_file } => Some(SmellDetails::TestLeakage {
                test_file: test_file.to_string_lossy().to_string(),
            }),

            SmellType::VendorCoupling { package } => Some(SmellDetails::VendorCoupling {
                package: package.clone(),
            }),

            SmellType::PackageCycle { packages } => Some(SmellDetails::PackageCycle {
                packages: packages.clone(),
            }),

            SmellType::SharedMutableState { symbol } => Some(SmellDetails::SharedMutableState {
                symbol: symbol.clone(),
            }),

            SmellType::LongParameterList { function, .. } => {
                Some(SmellDetails::LongParameterList {
                    function: function.clone(),
                })
            }

            SmellType::PrimitiveObsession { function, .. } => {
                Some(SmellDetails::PrimitiveObsession {
                    function: function.clone(),
                })
            }

            SmellType::OrphanType { name } => Some(SmellDetails::OrphanType { name: name.clone() }),

            SmellType::ScatteredConfiguration { env_var, .. } => {
                Some(SmellDetails::ScatteredConfiguration {
                    env_var: env_var.clone(),
                })
            }

            _ => None,
        }
    }

    fn build_summary(&self, scan_result: &ScanResult) -> SnapshotSummary {
        let summary = &scan_result.summary;

        SnapshotSummary {
            total_smells: summary.total_smells,
            files_analyzed: summary.files_analyzed,
            cycles: summary.cyclic_dependencies,
            god_modules: summary.god_modules,
            dead_code: summary.dead_code,
            dead_symbols: summary.dead_symbols,
            layer_violations: count_by_type(&scan_result.smells, "LayerViolation"),
            high_complexity: summary.high_complexity_functions,
            hub_modules: summary.hub_dependencies, // Based on result.rs summary, hub_dependencies is what we have
            avg_fan_in: None,
            avg_fan_out: None,
        }
    }
}

fn count_by_type(smells: &[SmellWithExplanation], type_prefix: &str) -> usize {
    smells
        .iter()
        .filter(|s| smell_type_to_string(&s.smell.smell_type).starts_with(type_prefix))
        .count()
}

fn smell_type_to_string(smell_type: &SmellType) -> String {
    match smell_type {
        SmellType::CyclicDependency => "CyclicDependency".to_string(),
        SmellType::CyclicDependencyCluster => "CyclicDependencyCluster".to_string(),
        SmellType::GodModule => "GodModule".to_string(),
        SmellType::DeadCode => "DeadCode".to_string(),
        SmellType::DeadSymbol { .. } => "DeadSymbol".to_string(),
        SmellType::HighComplexity { .. } => "HighComplexity".to_string(),
        SmellType::LayerViolation { .. } => "LayerViolation".to_string(),
        SmellType::HubModule => "HubModule".to_string(),
        SmellType::LowCohesion { .. } => "LowCohesion".to_string(),
        _ => format!("{:?}", smell_type)
            .split('{')
            .next()
            .unwrap_or("Unknown")
            .trim()
            .to_string(),
    }
}

fn get_git_commit(project_root: &Path) -> Option<String> {
    let repo = git2::Repository::discover(project_root).ok()?;
    let head = repo.head().ok()?;
    let target = head.target()?;
    Some(target.to_string()[..7].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::result::Summary;
    use crate::report::ArchitectureGrade;

    fn make_test_scan_result() -> ScanResult {
        ScanResult {
            smells: vec![],
            summary: Summary::default(),
            files: vec![],
            grade: ArchitectureGrade::default(),
            project_path: PathBuf::from("/test"),
        }
    }

    #[test]
    fn test_generator_sets_metadata() {
        let scan = make_test_scan_result();
        let gen = SnapshotGenerator::new(PathBuf::from("/test")).with_commit(false);

        let snapshot = gen.generate(&scan);

        assert_eq!(snapshot.schema_version, SCHEMA_VERSION);
        assert!(!snapshot.archlint_version.is_empty());
        assert!(!snapshot.generated_at.is_empty());
        assert!(snapshot.commit.is_none());
    }

    #[test]
    fn test_smell_type_to_string() {
        assert_eq!(
            smell_type_to_string(&SmellType::CyclicDependency),
            "CyclicDependency"
        );
        assert_eq!(
            smell_type_to_string(&SmellType::LayerViolation {
                from_layer: "ui".into(),
                to_layer: "domain".into(),
            }),
            "LayerViolation"
        );
    }
}
