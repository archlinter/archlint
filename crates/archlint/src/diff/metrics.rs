use super::types::{Improvement, ImprovementType, Regression, RegressionType};
use crate::snapshot::SnapshotSmell;

/// Metrics that should be compared for worsening
const TRACKABLE_METRICS: &[&str] = &[
    "fanIn",
    "fanOut",
    "cycleLength",
    "complexity",
    "lcom",
    "cbo",
    "depth",
    "cloneInstances",
];

pub struct MetricComparator {
    threshold_percent: f64,
}

impl MetricComparator {
    pub fn new(threshold_percent: f64) -> Self {
        Self { threshold_percent }
    }

    pub fn compare(
        &self,
        id: &str,
        baseline: &SnapshotSmell,
        current: &SnapshotSmell,
    ) -> (Vec<Regression>, Vec<Improvement>) {
        let mut regressions = Vec::new();
        let mut improvements = Vec::new();

        for metric_name in TRACKABLE_METRICS {
            let baseline_val = baseline.metrics.get(*metric_name);
            let current_val = current.metrics.get(*metric_name);

            if let (Some(base), Some(curr)) = (baseline_val, current_val) {
                let base_f = base.as_f64();
                let curr_f = curr.as_f64();

                if base_f == 0.0 {
                    if curr_f > 0.0 {
                        // If it was 0 and now it's not, it's an infinite % increase, so we count it as regression
                        regressions.push(Regression {
                            id: id.to_string(),
                            regression_type: RegressionType::MetricWorsening {
                                metric: metric_name.to_string(),
                                from: base_f,
                                to: curr_f,
                                change_percent: 100.0, // Arbitrary high value
                            },
                            smell: current.clone(),
                            message: format!(
                                "{} worsened: {} 0 → {} (+100%)",
                                current.smell_type, metric_name, curr_f as i64
                            ),
                            explain: None,
                        });
                    }
                    continue;
                }

                let change_percent = ((curr_f - base_f) / base_f) * 100.0;
                let is_worsened = if *metric_name == "cloneInstances" {
                    curr_f > base_f // Strictly greater for clones
                } else {
                    change_percent >= self.threshold_percent
                };

                if is_worsened {
                    // Worsened
                    regressions.push(Regression {
                        id: id.to_string(),
                        regression_type: RegressionType::MetricWorsening {
                            metric: metric_name.to_string(),
                            from: base_f,
                            to: curr_f,
                            change_percent,
                        },
                        smell: current.clone(),
                        message: format!(
                            "{} worsened: {} {} → {} (+{:.0}%)",
                            current.smell_type,
                            metric_name,
                            base_f as i64,
                            curr_f as i64,
                            change_percent
                        ),
                        explain: None,
                    });
                } else if change_percent <= -self.threshold_percent {
                    // Improved
                    improvements.push(Improvement {
                        id: id.to_string(),
                        improvement_type: ImprovementType::MetricImprovement {
                            metric: metric_name.to_string(),
                            from: base_f,
                            to: curr_f,
                        },
                        message: format!(
                            "{} improved: {} {} → {} ({:.0}%)",
                            current.smell_type,
                            metric_name,
                            base_f as i64,
                            curr_f as i64,
                            change_percent
                        ),
                    });
                }
            }
        }

        (regressions, improvements)
    }
}
