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
            if let (Some(base), Some(curr)) = (
                baseline.metrics.get(*metric_name),
                current.metrics.get(*metric_name),
            ) {
                let (regression, improvement) =
                    self.compare_metric(metric_name, id, base.as_f64(), curr.as_f64(), current);

                if let Some(r) = regression {
                    regressions.push(r);
                }
                if let Some(i) = improvement {
                    improvements.push(i);
                }
            }
        }

        (regressions, improvements)
    }

    fn compare_metric(
        &self,
        name: &str,
        id: &str,
        base: f64,
        curr: f64,
        current: &SnapshotSmell,
    ) -> (Option<Regression>, Option<Improvement>) {
        if base == curr {
            return (None, None);
        }

        let change_percent = if base == 0.0 {
            100.0
        } else {
            ((curr - base) / base) * 100.0
        };

        let is_worsened = if name == "cloneInstances" {
            curr > base
        } else {
            change_percent >= self.threshold_percent
        };

        if is_worsened {
            let message = if name == "cloneInstances" {
                format!(
                    "{} worsened: {} {} → {} (new clones detected)",
                    current.smell_type, name, base as i64, curr as i64
                )
            } else {
                format!(
                    "{} worsened: {} {} → {} (+{:.0}%)",
                    current.smell_type,
                    name,
                    if base == 0.0 {
                        "0".to_string()
                    } else {
                        (base as i64).to_string()
                    },
                    curr as i64,
                    change_percent
                )
            };

            return (
                Some(Regression {
                    id: id.to_string(),
                    regression_type: RegressionType::MetricWorsening {
                        metric: name.to_string(),
                        from: base,
                        to: curr,
                        change_percent,
                    },
                    smell: current.clone(),
                    message,
                    explain: None,
                }),
                None,
            );
        }

        if change_percent <= -self.threshold_percent {
            return (
                None,
                Some(Improvement {
                    id: id.to_string(),
                    improvement_type: ImprovementType::MetricImprovement {
                        metric: name.to_string(),
                        from: base,
                        to: curr,
                    },
                    message: format!(
                        "{} improved: {} {} → {} ({:.0}%)",
                        current.smell_type, name, base as i64, curr as i64, change_percent
                    ),
                }),
            );
        }

        (None, None)
    }
}
