use super::metrics::MetricComparator;
use super::types::{
    DiffResult, DiffSummary, Improvement, ImprovementType, Regression, RegressionType,
};
use crate::snapshot::{Snapshot, SnapshotSmell};
use log::debug;
use std::collections::{HashMap, HashSet};

pub struct DiffEngine {
    metric_threshold_percent: f64,
}

impl Default for DiffEngine {
    fn default() -> Self {
        Self {
            metric_threshold_percent: 20.0, // 20% increase = regression
        }
    }
}

impl DiffEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_threshold(mut self, percent: f64) -> Self {
        self.metric_threshold_percent = percent;
        self
    }

    /// Add explanations to all regressions
    pub fn diff_with_explain(&self, baseline: &Snapshot, current: &Snapshot) -> DiffResult {
        let mut result = self.diff(baseline, current);

        for regression in &mut result.regressions {
            regression.explain = Some(super::explain::generate_explain(regression));
        }

        result
    }

    pub fn diff(&self, baseline: &Snapshot, current: &Snapshot) -> DiffResult {
        // Build ID -> smell maps
        let baseline_map: HashMap<&str, &SnapshotSmell> =
            baseline.smells.iter().map(|s| (s.id.as_str(), s)).collect();

        let current_map: HashMap<&str, &SnapshotSmell> =
            current.smells.iter().map(|s| (s.id.as_str(), s)).collect();

        let baseline_ids: HashSet<&str> = baseline_map.keys().copied().collect();
        let current_ids: HashSet<&str> = current_map.keys().copied().collect();

        let mut regressions = Vec::new();
        let mut improvements = Vec::new();

        debug!(
            "Diffing baseline ({} smells) with current ({} smells)",
            baseline_ids.len(),
            current_ids.len()
        );

        // 1. New smells = regressions
        for id in current_ids.difference(&baseline_ids) {
            let smell = current_map[id];
            debug!("New smell detected: {} ({})", id, smell.smell_type);
            regressions.push(Regression {
                id: id.to_string(),
                regression_type: RegressionType::NewSmell,
                smell: smell.clone(),
                message: format!(
                    "New {}: {}",
                    smell.smell_type,
                    smell.files.first().cloned().unwrap_or_default()
                ),
                explain: None,
            });
        }

        // 2. Fixed smells = improvements
        for id in baseline_ids.difference(&current_ids) {
            let smell = baseline_map[id];
            debug!("Fixed smell: {} ({})", id, smell.smell_type);
            improvements.push(Improvement {
                id: id.to_string(),
                improvement_type: ImprovementType::Fixed,
                message: format!(
                    "Fixed {}: {}",
                    smell.smell_type,
                    smell.files.first().cloned().unwrap_or_default()
                ),
            });
        }

        // 3. Check existing smells for worsening/improvement
        for id in baseline_ids.intersection(&current_ids) {
            let baseline_smell = baseline_map[id];
            let current_smell = current_map[id];

            // Check severity change
            if let Some(reg) = self.check_severity_change(id, baseline_smell, current_smell) {
                regressions.push(reg);
            }

            // Check metric worsening
            let comparator = MetricComparator::new(self.metric_threshold_percent);
            let (metric_regressions, metric_improvements) =
                comparator.compare(id, baseline_smell, current_smell);

            regressions.extend(metric_regressions);
            improvements.extend(metric_improvements);
        }

        // Build summary
        let summary = DiffSummary {
            new_smells: regressions
                .iter()
                .filter(|r| matches!(r.regression_type, RegressionType::NewSmell))
                .count(),
            fixed_smells: improvements
                .iter()
                .filter(|i| matches!(i.improvement_type, ImprovementType::Fixed))
                .count(),
            worsened_smells: regressions
                .iter()
                .filter(|r| !matches!(r.regression_type, RegressionType::NewSmell))
                .count(),
            improved_smells: improvements
                .iter()
                .filter(|i| !matches!(i.improvement_type, ImprovementType::Fixed))
                .count(),
            total_regressions: regressions.len(),
            total_improvements: improvements.len(),
        };

        DiffResult {
            has_regressions: !regressions.is_empty(),
            regressions,
            improvements,
            summary,
            baseline_commit: baseline.commit.clone(),
            current_commit: current.commit.clone(),
        }
    }

    fn check_severity_change(
        &self,
        id: &str,
        baseline: &SnapshotSmell,
        current: &SnapshotSmell,
    ) -> Option<Regression> {
        let severity_order = |s: &str| match s {
            "Low" => 0,
            "Medium" => 1,
            "High" => 2,
            "Critical" => 3,
            _ => 0,
        };

        let base_sev = severity_order(&baseline.severity);
        let curr_sev = severity_order(&current.severity);

        if curr_sev > base_sev {
            Some(Regression {
                id: id.to_string(),
                regression_type: RegressionType::SeverityIncrease {
                    from: baseline.severity.clone(),
                    to: current.severity.clone(),
                },
                smell: current.clone(),
                message: format!(
                    "{} severity increased: {} → {}",
                    current.smell_type, baseline.severity, current.severity
                ),
                explain: None,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::MetricValue;
    use crate::snapshot::SnapshotSummary;

    fn make_smell(id: &str, smell_type: &str, severity: &str) -> SnapshotSmell {
        SnapshotSmell {
            id: id.to_string(),
            smell_type: smell_type.to_string(),
            severity: severity.to_string(),
            files: vec!["test.ts".to_string()],
            metrics: HashMap::new(),
            details: None,
            locations: vec![],
        }
    }

    fn make_snapshot(smells: Vec<SnapshotSmell>) -> Snapshot {
        Snapshot {
            schema_version: 1,
            archlint_version: "0.5.0".to_string(),
            generated_at: "2026-01-05T12:00:00Z".to_string(),
            commit: None,
            smells,
            summary: SnapshotSummary::default(),
            grade: "B".to_string(),
        }
    }

    #[test]
    fn test_new_smell_is_regression() {
        let baseline = make_snapshot(vec![]);
        let current = make_snapshot(vec![make_smell("cycle:abc", "CyclicDependency", "High")]);

        let result = DiffEngine::default().diff(&baseline, &current);

        assert!(result.has_regressions);
        assert_eq!(result.regressions.len(), 1);
        assert!(matches!(
            result.regressions[0].regression_type,
            RegressionType::NewSmell
        ));
    }

    #[test]
    fn test_fixed_smell_is_improvement() {
        let baseline = make_snapshot(vec![make_smell("cycle:abc", "CyclicDependency", "High")]);
        let current = make_snapshot(vec![]);

        let result = DiffEngine::default().diff(&baseline, &current);

        assert!(!result.has_regressions);
        assert_eq!(result.improvements.len(), 1);
        assert!(matches!(
            result.improvements[0].improvement_type,
            ImprovementType::Fixed
        ));
    }

    #[test]
    fn test_severity_increase_is_regression() {
        let baseline = make_snapshot(vec![make_smell("god:service.ts", "GodModule", "Medium")]);
        let current = make_snapshot(vec![make_smell("god:service.ts", "GodModule", "High")]);

        let result = DiffEngine::default().diff(&baseline, &current);

        assert!(result.has_regressions);
        assert!(matches!(
            &result.regressions[0].regression_type,
            RegressionType::SeverityIncrease { from, to }
                if from == "Medium" && to == "High"
        ));
    }

    #[test]
    fn test_metric_worsening() {
        let mut base_smell = make_smell("god:service.ts", "GodModule", "High");
        base_smell
            .metrics
            .insert("fanIn".to_string(), MetricValue::Int(10));

        let mut curr_smell = make_smell("god:service.ts", "GodModule", "High");
        curr_smell
            .metrics
            .insert("fanIn".to_string(), MetricValue::Int(25));

        let baseline = make_snapshot(vec![base_smell]);
        let current = make_snapshot(vec![curr_smell]);

        let result = DiffEngine::default().diff(&baseline, &current);

        assert!(result.has_regressions);
        assert!(matches!(
            &result.regressions[0].regression_type,
            RegressionType::MetricWorsening { metric, .. } if metric == "fanIn"
        ));
    }

    #[test]
    fn test_identical_snapshots_no_diff() {
        let smell = make_smell("god:service.ts", "GodModule", "High");
        let baseline = make_snapshot(vec![smell.clone()]);
        let current = make_snapshot(vec![smell]);

        let result = DiffEngine::default().diff(&baseline, &current);

        assert!(!result.has_regressions);
        assert!(result.regressions.is_empty());
        assert!(result.improvements.is_empty());
    }
}
