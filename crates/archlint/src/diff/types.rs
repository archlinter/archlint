use crate::snapshot::SnapshotSmell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffResult {
    /// Whether any regressions were found
    pub has_regressions: bool,

    /// List of architectural regressions
    pub regressions: Vec<Regression>,

    /// List of improvements (for info)
    pub improvements: Vec<Improvement>,

    /// Summary statistics
    pub summary: DiffSummary,

    /// Baseline commit (if available)
    pub baseline_commit: Option<String>,

    /// Current commit (if available)
    pub current_commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regression {
    /// Smell ID
    pub id: String,

    /// Type of regression
    pub regression_type: RegressionType,

    /// The smell data
    pub smell: SnapshotSmell,

    /// Human-readable message
    pub message: String,

    /// Detailed explanation (filled by explain phase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain: Option<ExplainBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RegressionType {
    /// New smell introduced
    NewSmell,

    /// Existing smell got worse severity
    SeverityIncrease { from: String, to: String },

    /// Metric worsened beyond threshold
    MetricWorsening {
        metric: String,
        from: f64,
        to: f64,
        change_percent: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Improvement {
    /// Smell ID that was fixed
    pub id: String,

    /// Type of improvement
    pub improvement_type: ImprovementType,

    /// Human-readable message
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImprovementType {
    /// Smell was completely fixed
    Fixed,

    /// Severity decreased
    SeverityDecrease { from: String, to: String },

    /// Metric improved
    MetricImprovement { metric: String, from: f64, to: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiffSummary {
    pub new_smells: usize,
    pub fixed_smells: usize,
    pub worsened_smells: usize,
    pub improved_smells: usize,
    pub total_regressions: usize,
    pub total_improvements: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainBlock {
    /// Why this is architecturally bad
    pub why_bad: String,

    /// Long-term consequences
    pub consequences: String,

    /// How to fix it
    pub how_to_fix: String,
}
