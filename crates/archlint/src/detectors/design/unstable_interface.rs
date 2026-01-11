use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellWithExplanation,
};
use crate::engine::AnalysisContext;

pub fn init() {}

#[detector(
    id = "unstable_interface",
    name = "Unstable Interface Detector",
    description = "Detects modules with high churn and many dependants",
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
    fn name(&self) -> &'static str {
        "UnstableInterface"
    }

    fn explain(&self, smell: &ArchSmell) -> Explanation {
        let churn = smell.churn().unwrap_or(0);
        let dependants = smell.fan_in().unwrap_or(0);
        let score = smell.instability_score().unwrap_or(0);

        Explanation {
            problem: format!(
                "Unstable interface detected (churn: {}, dependants: {}, score: {})",
                churn, dependants, score
            ),
            reason: "This module changes frequently and is used by many other modules. This means changes here have a high probability of breaking other parts of the system.".to_string(),
            risks: vec![
                "Frequent regressions in dependant modules".to_string(),
                "High cost of maintenance due to cascading changes".to_string(),
                "Difficult to stabilize the overall architecture".to_string(),
            ],
            recommendations: vec![
                "Identify why the module changes so frequently and extract stable parts".to_string(),
                "Introduce a stable interface (API) and keep implementation details hidden".to_string(),
                "Reduce the number of dependants by using events or a message bus".to_string(),
            ],
        }
    }

    fn render_markdown(
        &self,
        unstable_interfaces: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::explain::ExplainEngine;

        crate::define_report_section!("Unstable Interfaces", unstable_interfaces, {
            crate::render_table!(
                vec!["File", "Churn", "Dependants", "Score", "pts"],
                unstable_interfaces,
                |&(smell, _): &&SmellWithExplanation| {
                    let file_path = smell.files.first().unwrap();
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let churn = smell.churn().unwrap_or(0);
                    let dependants = smell.fan_in().unwrap_or(0);
                    let score = smell.instability_score().unwrap_or(0);
                    let pts = smell.score(severity_config);

                    vec![
                        format!("`{}`", formatted_path),
                        churn.to_string(),
                        dependants.to_string(),
                        score.to_string(),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        // Check if git churn information is available
        let git_available = ctx.config.git.enabled && !ctx.churn_map.is_empty();

        ctx.graph
            .nodes()
            .filter_map(|node| {
                let path = ctx.graph.get_file_path(node)?;
                let rule = ctx.get_rule_for_file("unstable_interface", path)?;

                let min_churn: usize = rule.get_option("min_churn").unwrap_or(10);
                let min_dependants: usize = rule.get_option("min_dependants").unwrap_or(5);
                let score_threshold: usize = rule.get_option("score_threshold").unwrap_or(100);

                let churn = ctx.churn_map.get(path).copied().unwrap_or(0);
                let dependants = ctx.graph.fan_in(node);

                let score = churn * dependants;

                // If git is not available, we skip the churn and score threshold checks
                let churn_ok = !git_available || churn >= min_churn;
                let score_ok = !git_available || score >= score_threshold;

                if churn_ok && score_ok && dependants >= min_dependants {
                    let mut smell =
                        ArchSmell::new_unstable_interface(path.clone(), churn, dependants, score);
                    smell.severity = rule.severity;
                    Some(smell)
                } else {
                    None
                }
            })
            .collect()
    }
}
