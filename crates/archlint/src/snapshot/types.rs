use crate::detectors::{CodeRange, SmellType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Schema version for forward compatibility
pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    /// Schema version for compatibility checking
    pub schema_version: u32,

    /// Archlint version that generated this snapshot
    pub archlint_version: String,

    /// ISO 8601 timestamp
    pub generated_at: String,

    /// Git commit hash (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,

    /// All detected smells with stable IDs
    pub smells: Vec<SnapshotSmell>,

    /// Aggregated metrics
    pub summary: SnapshotSummary,

    /// Architecture grade (A/B/C/D/F)
    pub grade: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotSmell {
    /// Stable ID for diff comparison
    pub id: String,

    /// Smell type as string (e.g., "CyclicDependency", "GodModule")
    pub smell_type: String,

    /// Severity level
    pub severity: String,

    /// Affected files (relative paths, sorted)
    pub files: Vec<String>,

    /// Metrics for comparison
    pub metrics: HashMap<String, MetricValue>,

    /// Type-specific details (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<SmellType>,

    /// Specific locations with line/column information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: Option<usize>,
    pub range: Option<CodeRange>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MetricValue {
    Int(i64),
    Float(f64),
    String(String),
}

impl MetricValue {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            MetricValue::Int(v) => Some(*v),
            MetricValue::Float(v) => Some(*v as i64),
            MetricValue::String(_) => None,
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            MetricValue::Int(v) => *v as f64,
            MetricValue::Float(v) => *v,
            MetricValue::String(_) => 0.0,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            MetricValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotSummary {
    pub total_smells: usize,
    pub files_analyzed: usize,

    // By category
    pub cycles: usize,
    pub god_modules: usize,
    pub dead_code: usize,
    pub dead_symbols: usize,
    pub layer_violations: usize,
    #[serde(default, alias = "highComplexity")]
    pub high_cyclomatic_complexity: usize,
    #[serde(default)]
    pub high_cognitive_complexity: usize,
    pub hub_modules: usize,

    // Optional extended metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_fan_in: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_fan_out: Option<f64>,
}

impl Snapshot {
    pub fn validate(&self) -> Result<(), SnapshotError> {
        if self.schema_version > SCHEMA_VERSION {
            return Err(SnapshotError::UnsupportedVersion {
                found: self.schema_version,
                supported: SCHEMA_VERSION,
            });
        }

        // Check for duplicate IDs
        let mut ids = std::collections::HashSet::new();
        for smell in &self.smells {
            if !ids.insert(&smell.id) {
                return Err(SnapshotError::DuplicateId(smell.id.clone()));
            }
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SnapshotError {
    #[error("Unsupported schema version {found}, max supported: {supported}")]
    UnsupportedVersion { found: u32, supported: u32 },

    #[error("Duplicate smell ID: {0}")]
    DuplicateId(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let snapshot = Snapshot {
            schema_version: SCHEMA_VERSION,
            archlint_version: "0.5.0".into(),
            generated_at: "2026-01-05T12:00:00Z".into(),
            commit: Some("abc123".into()),
            smells: vec![],
            summary: SnapshotSummary::default(),
            grade: "A".into(),
        };

        let json = serde_json::to_string(&snapshot).unwrap();
        let parsed: Snapshot = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn test_metric_value_conversions() {
        let int_val = MetricValue::Int(42);
        assert_eq!(int_val.as_i64(), Some(42));
        assert_eq!(int_val.as_f64(), 42.0);

        let float_val = MetricValue::Float(123.45);
        assert_eq!(float_val.as_i64(), Some(123));
        assert_eq!(float_val.as_f64(), 123.45);
    }

    #[test]
    fn test_validate_duplicate_ids() {
        let snapshot = Snapshot {
            schema_version: SCHEMA_VERSION,
            archlint_version: "0.5.0".into(),
            generated_at: "2026-01-05T12:00:00Z".into(),
            commit: None,
            smells: vec![
                SnapshotSmell {
                    id: "dup".into(),
                    smell_type: "Type".into(),
                    severity: "High".into(),
                    files: vec![],
                    metrics: HashMap::new(),
                    details: None,
                    locations: vec![],
                },
                SnapshotSmell {
                    id: "dup".into(),
                    smell_type: "Type".into(),
                    severity: "High".into(),
                    files: vec![],
                    metrics: HashMap::new(),
                    details: None,
                    locations: vec![],
                },
            ],
            summary: SnapshotSummary::default(),
            grade: "A".into(),
        };

        assert!(matches!(
            snapshot.validate(),
            Err(SnapshotError::DuplicateId(_))
        ));
    }
}
