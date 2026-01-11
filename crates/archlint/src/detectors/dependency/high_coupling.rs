use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;

pub fn init() {}

#[detector(
    id = "high_coupling",
    name = "High Coupling Detector (CBO)",
    description = "Detects modules with too many incoming and outgoing dependencies",
    category = DetectorCategory::GraphBased,
    default_enabled = false
)]
pub struct HighCouplingDetector;

impl HighCouplingDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for HighCouplingDetector {
    fn name(&self) -> &'static str {
        "HighCoupling"
    }

    fn explain(&self, _smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "High Coupling (CBO)".to_string(),
            reason: "Module has too many incoming and outgoing dependencies (Coupling Between Objects). High coupling makes code difficult to change and test in isolation.".to_string(),
            risks: vec!["Fragile system: changes ripple through many modules".to_string(), "Difficult to mock dependencies for testing".to_string()],
            recommendations: vec!["Refactor to reduce dependencies or move functionality to a more appropriate place".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::explain::ExplainEngine;
        crate::define_report_section!("High Coupling", smells, {
            crate::render_table!(
                vec!["File", "CBO Score", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let file_path = smell.files.first().unwrap();
                    let formatted_path = ExplainEngine::format_file_path(file_path);
                    let cbo = match &smell.smell_type {
                        SmellType::HighCoupling { cbo } => cbo.to_string(),
                        _ => "unknown".to_string(),
                    };
                    let pts = smell.score(severity_config);
                    vec![format!("`{}`", formatted_path), cbo, format!("{} pts", pts)]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                let rule = match ctx.get_rule_for_file("high_coupling", path) {
                    Some(r) => r,
                    None => continue,
                };

                let max_cbo: usize = rule.get_option("max_cbo").unwrap_or(20);

                let fan_in = ctx.graph.fan_in(node);
                let fan_out = ctx.graph.fan_out(node);
                let cbo = fan_in + fan_out;

                if cbo >= max_cbo {
                    let mut smell = ArchSmell::new_high_coupling(path.clone(), cbo);
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
