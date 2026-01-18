use crate::api::result::{ScanResult, SmellWithExplanation};
use crate::detectors::{ArchSmell, SmellType};
use crate::snapshot::id::generate_smell_id;
use crate::snapshot::types::{
    MetricValue, Snapshot, SnapshotSmell, SnapshotSummary, SCHEMA_VERSION,
};
use chrono::Utc;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct SnapshotGenerator {
    project_root: PathBuf,
    include_commit: bool,
}

impl SnapshotGenerator {
    #[must_use]
    pub fn new(project_root: PathBuf) -> Self {
        let project_root = project_root.canonicalize().unwrap_or(project_root);
        Self {
            project_root,
            include_commit: true,
        }
    }

    #[must_use]
    pub const fn with_commit(mut self, include: bool) -> Self {
        self.include_commit = include;
        self
    }

    #[must_use]
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

        SnapshotSmell {
            id,
            smell_type: smell_type_to_string(&smell.smell_type),
            severity: format!("{:?}", smell.severity),
            files,
            metrics,
            details: Some(smell.smell_type.clone()),
            locations: self.extract_locations(smell),
        }
    }

    fn extract_locations(&self, smell: &ArchSmell) -> Vec<crate::snapshot::types::Location> {
        smell
            .locations
            .iter()
            .map(|loc| {
                let file = loc.file.strip_prefix(&self.project_root).map_or_else(
                    |_| loc.file.to_string_lossy().replace('\\', "/"),
                    |p| p.to_string_lossy().replace('\\', "/"),
                );

                crate::snapshot::types::Location {
                    file,
                    line: loc.line,
                    column: loc.column,
                    range: loc.range,
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
            if let Ok(serde_json::Value::Object(map)) = serde_json::to_value(metric) {
                for (k, v) in map {
                    match serde_json::from_value(v) {
                        Ok(val) => {
                            metrics.insert(k, val);
                        }
                        Err(e) => {
                            log::debug!("Failed to convert metric value for {k}: {e}");
                        }
                    }
                }
            } else {
                log::debug!("Failed to serialize metric to JSON object: {metric:?}");
            }
        }

        // Add special fields from SmellType
        match &smell.smell_type {
            SmellType::CodeClone {
                clone_hash,
                token_count,
            } => {
                metrics.insert("cloneHash".into(), MetricValue::String(clone_hash.clone()));
                metrics.insert(
                    "tokenCount".into(),
                    MetricValue::Int(i64::try_from(*token_count).unwrap_or(i64::MAX)),
                );
            }
            SmellType::ScatteredConfiguration { files_count, .. } => {
                metrics.insert(
                    "filesCount".into(),
                    MetricValue::Int(i64::try_from(*files_count).unwrap_or(i64::MAX)),
                );
            }
            _ => {}
        }

        metrics
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
            high_cyclomatic_complexity: summary.high_cyclomatic_complexity_functions,
            high_cognitive_complexity: summary.high_cognitive_complexity_functions,
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
        SmellType::HighCyclomaticComplexity { .. } => "HighCyclomaticComplexity".to_string(),
        SmellType::HighCognitiveComplexity { .. } => "HighCognitiveComplexity".to_string(),
        SmellType::LayerViolation { .. } => "LayerViolation".to_string(),
        SmellType::HubModule => "HubModule".to_string(),
        SmellType::LowCohesion { .. } => "LowCohesion".to_string(),
        _ => format!("{smell_type:?}")
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
    use crate::detectors::SmellMetric;
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
    fn test_extract_metrics_comprehensive() {
        let gen = SnapshotGenerator::new(PathBuf::from("/test"));
        let smell = ArchSmell {
            smell_type: SmellType::PrimitiveObsession {
                primitives: 10,
                name: "test".to_string(),
            },
            severity: crate::detectors::Severity::High,
            files: vec![],
            metrics: vec![
                SmellMetric::PrimitiveCount(10),
                SmellMetric::Churn(42),
                SmellMetric::InstabilityDiff(0.5),
            ],
            locations: vec![],
            cluster: None,
        };

        let metrics = gen.extract_metrics(&smell);
        assert_eq!(metrics.get("primitiveCount").unwrap().as_i64(), Some(10));
        assert_eq!(metrics.get("churn").unwrap().as_i64(), Some(42));
        assert_eq!(metrics.get("instabilityDiff").unwrap().as_f64(), 0.5);
    }
}
