use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(
    id = "unstable_interface",
    name = "Unstable Interface Detector",
    description = "Detects modules with high churn and many dependents",
    category = DetectorCategory::Global,
    default_enabled = false
)]
pub struct UnstableInterfaceDetector;

impl UnstableInterfaceDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for UnstableInterfaceDetector {
    crate::impl_detector_report!(
        name: "UnstableInterface",
        explain: smell => {
            let churn = smell.churn().unwrap_or(0);
            let dependents = smell.fan_in().unwrap_or(0);
            let score = smell.instability_score().unwrap_or(0);

            crate::detectors::Explanation {
                problem: format!(
                    "Unstable interface detected (churn: {}, dependents: {}, score: {})",
                    churn, dependents, score
                ),
                reason: "This module changes frequently and is used by many other modules. This means changes here have a high probability of breaking other parts of the system.".into(),
                risks: crate::strings![
                    "Frequent regressions in dependent modules",
                    "High cost of maintenance due to cascading changes",
                    "Difficult to stabilize the overall architecture"
                ],
                recommendations: crate::strings![
                    "Identify why the module changes so frequently and extract stable parts",
                    "Introduce a stable interface (API) and keep implementation details hidden",
                    "Reduce the number of dependents by using events or a message bus"
                ]
            }
        },
        table: {
            title: "Unstable Interfaces",
            columns: ["File", "Churn", "Dependents", "Score", "pts"],
            row: UnstableInterface { } (smell, location, pts) => [
                location,
                smell.churn().unwrap_or(0),
                smell.fan_in().unwrap_or(0),
                smell.instability_score().unwrap_or(0),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        // Check if git churn information is available
        let git_available = ctx.config.git.enabled && !ctx.churn_map.is_empty();

        ctx.graph
            .nodes()
            .filter_map(|node| {
                let path = ctx.graph.get_file_path(node)?;
                let rule = ctx.get_rule_for_file("unstable_interface", path)?;

                let min_churn: usize = rule.get_option("min_churn").unwrap_or(10);
                let min_dependents: usize = rule
                    .get_option("min_dependents")
                    .or_else(|| rule.get_option("min_dependants"))
                    .unwrap_or(5);
                let score_threshold: usize = rule.get_option("score_threshold").unwrap_or(100);

                let churn = ctx.churn_map.get(path).copied().unwrap_or(0);
                let dependents = ctx.graph.fan_in(node);

                let score = churn.saturating_mul(dependents);

                // If git is not available, we skip the churn and score threshold checks
                let churn_ok = !git_available || churn >= min_churn;
                let score_ok = !git_available || score >= score_threshold;

                if churn_ok && score_ok && dependents >= min_dependents {
                    let mut smell =
                        ArchSmell::new_unstable_interface(path.clone(), churn, dependents, score);
                    smell.severity = rule.severity;
                    Some(smell)
                } else {
                    None
                }
            })
            .collect()
    }
}
