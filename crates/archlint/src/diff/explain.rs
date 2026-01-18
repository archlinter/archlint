use super::types::{ExplainBlock, Regression, RegressionType};
use crate::explain::{ExplainEngine, Explanation};
use crate::snapshot::SnapshotSmell;

#[must_use]
pub fn generate_explain(regression: &Regression, config: &crate::config::Config) -> ExplainBlock {
    match &regression.regression_type {
        RegressionType::NewSmell => explain_for_smell_type(&regression.smell, config),
        RegressionType::SeverityIncrease { from, to } => {
            explain_severity_increase(&regression.smell, from, to, config)
        }
        RegressionType::MetricWorsening {
            metric,
            from,
            to,
            change_percent,
        } => explain_metric_worsening(&regression.smell, metric, *from, *to, *change_percent),
    }
}

#[must_use]
pub fn explain_for_smell_type(
    smell: &SnapshotSmell,
    config: &crate::config::Config,
) -> ExplainBlock {
    let explanation = ExplainEngine::explain_snapshot_smell(smell, config);
    ExplanationConverter::to_explain_block(explanation)
}

#[must_use]
pub fn explain_severity_increase(
    smell: &SnapshotSmell,
    from: &str,
    to: &str,
    config: &crate::config::Config,
) -> ExplainBlock {
    let base = explain_for_smell_type(smell, config);

    ExplainBlock {
        why_bad: format!(
            "An existing {} got worse (severity: {} → {}). {}",
            smell.smell_type, from, to, base.why_bad
        ),
        consequences: format!(
            "The problem is growing, not shrinking. {}",
            base.consequences
        ),
        how_to_fix: format!(
            "Address this before it becomes Critical. {}",
            base.how_to_fix
        ),
    }
}

#[must_use]
pub fn explain_metric_worsening(
    smell: &SnapshotSmell,
    metric: &str,
    from: f64,
    to: f64,
    change_percent: f64,
) -> ExplainBlock {
    let metric_explanation = match metric {
        "fanIn" => "More modules now depend on this code. Changes here affect more consumers.",
        "fanOut" => "This module now depends on more external code. It's becoming harder to test.",
        "complexity" => "More code paths added. Testing and debugging become harder.",
        "cycleLength" => "The dependency cycle grew larger. Breaking it becomes more expensive.",
        "lcom" => "Class cohesion decreased. Methods share less state, suggesting split needed.",
        "cbo" => "Coupling increased. The module is now harder to change independently.",
        _ => "This metric indicates growing architectural debt.",
    };

    ExplainBlock {
        why_bad: format!(
            "{} increased from {} to {} (+{:.0}%). {}",
            metric, from as i64, to as i64, change_percent, metric_explanation
        ),
        consequences: format!(
            "If {} continues growing, the {} will become unmaintainable. \
            Consider this a warning sign.",
            metric, smell.smell_type
        ),
        how_to_fix: format!(
            "Review recent changes to understand why {metric} increased. \
            Consider refactoring to reduce coupling."
        ),
    }
}

struct ExplanationConverter;

impl ExplanationConverter {
    fn to_explain_block(explanation: Explanation) -> ExplainBlock {
        ExplainBlock {
            why_bad: format!("{}\n{}", explanation.problem, explanation.reason),
            consequences: explanation.risks.join("\n"),
            how_to_fix: explanation.recommendations.join("\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_test_smell(smell_type: &str) -> SnapshotSmell {
        SnapshotSmell {
            id: "test".to_string(),
            smell_type: smell_type.to_string(),
            severity: "High".to_string(),
            files: vec!["test.ts".to_string()],
            metrics: HashMap::new(),
            details: None,
            locations: vec![],
        }
    }

    #[test]
    fn test_explain_cycle() {
        let smell = make_test_smell("CyclicDependency");
        let config = crate::config::Config::default();
        let explain = explain_for_smell_type(&smell, &config);

        assert!(explain.why_bad.contains("Circular"));
        assert!(explain.how_to_fix.contains("Extract"));
    }

    #[test]
    fn test_explain_metric_worsening() {
        let smell = make_test_smell("GodModule");
        let explain = explain_metric_worsening(&smell, "fanIn", 10.0, 25.0, 150.0);

        assert!(explain.why_bad.contains("150%"));
        assert!(explain.why_bad.contains("fanIn"));
    }
}
