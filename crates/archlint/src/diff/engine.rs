use super::fuzzy::FuzzyMatcher;
use super::metrics::MetricComparator;
use super::types::{
    DiffResult, DiffSummary, Improvement, ImprovementType, Regression, RegressionType,
};
use crate::snapshot::{Snapshot, SnapshotSmell};
use log::debug;
use std::collections::{HashMap, HashSet};

pub struct DiffEngine {
    metric_threshold_percent: f64,
    line_tolerance: usize,
}

impl Default for DiffEngine {
    fn default() -> Self {
        Self {
            metric_threshold_percent: 20.0, // 20% increase = regression
            line_tolerance: 50,             // fuzzy match within 50 lines
        }
    }
}

type SmellMap<'a> = HashMap<&'a str, &'a SnapshotSmell>;
type SmellIdSet<'a> = HashSet<&'a str>;

impl DiffEngine {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn with_threshold(mut self, percent: f64) -> Self {
        self.metric_threshold_percent = percent;
        self
    }

    #[must_use]
    pub const fn with_line_tolerance(mut self, lines: usize) -> Self {
        self.line_tolerance = lines;
        self
    }

    /// Add explanations to all regressions
    #[must_use]
    pub fn diff_with_explain(
        &self,
        baseline: &Snapshot,
        current: &Snapshot,
        config: &crate::config::Config,
    ) -> DiffResult {
        let mut result = self.diff(baseline, current);

        for regression in &mut result.regressions {
            regression.explain = Some(super::explain::generate_explain(regression, config));
        }

        result
    }

    #[must_use]
    pub fn diff(&self, baseline: &Snapshot, current: &Snapshot) -> DiffResult {
        let (baseline_map, current_map, baseline_ids, current_ids) =
            Self::build_smell_maps(baseline, current);

        debug!(
            "Diffing baseline ({} smells) with current ({} smells)",
            baseline_ids.len(),
            current_ids.len()
        );

        let (orphaned_baseline, orphaned_current) =
            Self::find_orphaned_smells(&baseline_map, &current_map, &baseline_ids, &current_ids);

        let (matched_baseline_ids, matched_current_ids) =
            self.apply_fuzzy_matching(&orphaned_baseline, &orphaned_current);

        let mut regressions = Self::collect_new_smells(&orphaned_current, &matched_current_ids);
        let mut improvements =
            Self::collect_fixed_smells(&orphaned_baseline, &matched_baseline_ids);

        self.check_existing_smells(
            &baseline_ids,
            &current_ids,
            &baseline_map,
            &current_map,
            &mut regressions,
            &mut improvements,
        );

        Self::sort_results(&mut regressions, &mut improvements);

        let summary = Self::build_summary(&regressions, &improvements);

        DiffResult {
            has_regressions: !regressions.is_empty(),
            regressions,
            improvements,
            summary,
            baseline_commit: baseline.commit.clone(),
            current_commit: current.commit.clone(),
        }
    }

    fn build_smell_maps<'a>(
        baseline: &'a Snapshot,
        current: &'a Snapshot,
    ) -> (SmellMap<'a>, SmellMap<'a>, SmellIdSet<'a>, SmellIdSet<'a>) {
        let baseline_map: HashMap<&str, &SnapshotSmell> =
            baseline.smells.iter().map(|s| (s.id.as_str(), s)).collect();
        let current_map: HashMap<&str, &SnapshotSmell> =
            current.smells.iter().map(|s| (s.id.as_str(), s)).collect();
        let baseline_ids: HashSet<&str> = baseline_map.keys().copied().collect();
        let current_ids: HashSet<&str> = current_map.keys().copied().collect();
        (baseline_map, current_map, baseline_ids, current_ids)
    }

    fn find_orphaned_smells<'a>(
        baseline_map: &'a HashMap<&str, &SnapshotSmell>,
        current_map: &'a HashMap<&str, &SnapshotSmell>,
        baseline_ids: &'a HashSet<&str>,
        current_ids: &'a HashSet<&str>,
    ) -> (Vec<&'a SnapshotSmell>, Vec<&'a SnapshotSmell>) {
        let orphaned_baseline: Vec<&SnapshotSmell> = baseline_ids
            .difference(current_ids)
            .map(|id| baseline_map[id])
            .collect();
        let orphaned_current: Vec<&SnapshotSmell> = current_ids
            .difference(baseline_ids)
            .map(|id| current_map[id])
            .collect();
        (orphaned_baseline, orphaned_current)
    }

    fn apply_fuzzy_matching(
        &self,
        orphaned_baseline: &[&SnapshotSmell],
        orphaned_current: &[&SnapshotSmell],
    ) -> (HashSet<String>, HashSet<String>) {
        let fuzzy = FuzzyMatcher::new(self.line_tolerance);
        let matched_pairs = fuzzy.match_orphans(orphaned_baseline, orphaned_current);

        debug!(
            "Fuzzy matching: {} pairs matched out of {} orphaned baseline, {} orphaned current",
            matched_pairs.len(),
            orphaned_baseline.len(),
            orphaned_current.len()
        );

        let matched_baseline_ids: HashSet<String> = matched_pairs
            .iter()
            .map(|p| p.baseline.id.clone())
            .collect();
        let matched_current_ids: HashSet<String> =
            matched_pairs.iter().map(|p| p.current.id.clone()).collect();
        (matched_baseline_ids, matched_current_ids)
    }

    fn collect_new_smells(
        orphaned_current: &[&SnapshotSmell],
        matched_current_ids: &HashSet<String>,
    ) -> Vec<Regression> {
        let mut regressions = Vec::new();
        for smell in orphaned_current {
            if matched_current_ids.contains(&smell.id) {
                debug!(
                    "Smell shifted (not new): {} ({})",
                    smell.id, smell.smell_type
                );
                continue;
            }
            debug!("New smell detected: {} ({})", smell.id, smell.smell_type);
            regressions.push(Regression {
                id: smell.id.clone(),
                regression_type: RegressionType::NewSmell,
                smell: (*smell).clone(),
                message: format!(
                    "New {}: {}",
                    smell.smell_type,
                    smell.files.first().cloned().unwrap_or_default()
                ),
                explain: None,
            });
        }
        regressions
    }

    fn collect_fixed_smells(
        orphaned_baseline: &[&SnapshotSmell],
        matched_baseline_ids: &HashSet<String>,
    ) -> Vec<Improvement> {
        let mut improvements = Vec::new();
        for smell in orphaned_baseline {
            if matched_baseline_ids.contains(&smell.id) {
                debug!(
                    "Smell shifted (not fixed): {} ({})",
                    smell.id, smell.smell_type
                );
                continue;
            }
            debug!("Fixed smell: {} ({})", smell.id, smell.smell_type);
            improvements.push(Improvement {
                id: smell.id.clone(),
                improvement_type: ImprovementType::Fixed,
                message: format!(
                    "Fixed {}: {}",
                    smell.smell_type,
                    smell.files.first().cloned().unwrap_or_default()
                ),
            });
        }
        improvements
    }

    fn check_existing_smells(
        &self,
        baseline_ids: &HashSet<&str>,
        current_ids: &HashSet<&str>,
        baseline_map: &HashMap<&str, &SnapshotSmell>,
        current_map: &HashMap<&str, &SnapshotSmell>,
        regressions: &mut Vec<Regression>,
        improvements: &mut Vec<Improvement>,
    ) {
        for id in baseline_ids.intersection(current_ids) {
            let baseline_smell = baseline_map[id];
            let current_smell = current_map[id];

            if let Some(reg) = self.check_severity_change(id, baseline_smell, current_smell) {
                regressions.push(reg);
            }

            let comparator = MetricComparator::new(self.metric_threshold_percent);
            let (metric_regressions, metric_improvements) =
                comparator.compare(id, baseline_smell, current_smell);

            regressions.extend(metric_regressions);
            improvements.extend(metric_improvements);
        }
    }

    fn sort_results(regressions: &mut [Regression], improvements: &mut [Improvement]) {
        regressions.sort_by(|a, b| {
            let score_a = Self::severity_score(&a.smell.severity);
            let score_b = Self::severity_score(&b.smell.severity);
            score_b.cmp(&score_a)
        });

        improvements.sort_by(|a, b| {
            let priority_a = Self::improvement_priority(&a.improvement_type);
            let priority_b = Self::improvement_priority(&b.improvement_type);
            priority_a.cmp(&priority_b)
        });
    }

    fn build_summary(regressions: &[Regression], improvements: &[Improvement]) -> DiffSummary {
        DiffSummary {
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

    /// Get numeric score for severity (higher = more severe)
    fn severity_score(severity: &str) -> u8 {
        match severity.to_lowercase().as_str() {
            "critical" => 4,
            "high" => 3,
            "medium" => 2,
            "low" => 1,
            _ => 0,
        }
    }

    /// Get priority for improvement type (higher = more important)
    const fn improvement_priority(improvement_type: &ImprovementType) -> u8 {
        match improvement_type {
            ImprovementType::Fixed => 3,
            ImprovementType::SeverityDecrease { .. } => 2,
            ImprovementType::MetricImprovement { .. } => 1,
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
