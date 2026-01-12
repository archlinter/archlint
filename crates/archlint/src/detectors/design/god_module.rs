use crate::detectors::DetectorCategory;
use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(
    id = "god_module",
    name = "God Module Detector",
    description = "Detects large modules with many incoming and outgoing dependencies",
    category = DetectorCategory::Global
)]
pub struct GodModuleDetector;

impl GodModuleDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn generate_metrics_report(output: &mut String, smell: &crate::detectors::ArchSmell) {
        output.push_str("**Metrics:**\n");
        if let Some(fan_in) = smell.fan_in() {
            output.push_str(&format!("- Fan-in: {}\n", fan_in));
        }
        if let Some(fan_out) = smell.fan_out() {
            output.push_str(&format!("- Fan-out: {}\n", fan_out));
        }
        if let Some(churn) = smell.churn() {
            output.push_str(&format!("- Churn: {} commits\n", churn));
        }
        output.push('\n');
    }

    fn generate_dependencies_report(
        output: &mut String,
        file_path: &std::path::Path,
        graph: Option<&crate::graph::DependencyGraph>,
    ) {
        let Some(graph) = graph else { return };
        let Some(node_idx) = graph.get_node(file_path) else {
            return;
        };

        use crate::explain::ExplainEngine;

        let mut incoming: Vec<_> = graph
            .graph()
            .neighbors_directed(node_idx, petgraph::Direction::Incoming)
            .filter_map(|idx| graph.get_file_path(idx))
            .map(|p| ExplainEngine::format_file_path(p))
            .collect();

        if !incoming.is_empty() {
            incoming.sort();
            output.push_str("**Used by:**\n");
            for p in incoming.iter().take(15) {
                output.push_str(&format!("- `{}`\n", p));
            }
            if incoming.len() > 15 {
                output.push_str(&format!("- ... and {} more\n", incoming.len() - 15));
            }
            output.push('\n');
        }

        let mut outgoing: Vec<_> = graph
            .dependencies(node_idx)
            .filter_map(|idx| graph.get_file_path(idx))
            .map(|p| ExplainEngine::format_file_path(p))
            .collect();

        if !outgoing.is_empty() {
            outgoing.sort();
            output.push_str("**Depends on:**\n");
            for p in outgoing.iter().take(15) {
                output.push_str(&format!("- `{}`\n", p));
            }
            if outgoing.len() > 15 {
                output.push_str(&format!("- ... and {} more\n", outgoing.len() - 15));
            }
            output.push('\n');
        }
    }
}

impl Detector for GodModuleDetector {
    crate::impl_detector_report!(
        name: "GodModule",
        explain: smell => {
            let fan_in = smell.fan_in().unwrap_or(0);
            let fan_out = smell.fan_out().unwrap_or(0);
            let churn = smell.churn().unwrap_or(0);

            crate::detectors::Explanation {
                problem: format!(
                    "Module has excessive responsibilities (fan-in: {}, fan-out: {}, churn: {})",
                    fan_in, fan_out, churn
                ),
                reason: "This module is imported by many files (high fan-in), imports many files (high fan-out), and changes frequently (high churn). This indicates it's doing too much and violates the Single Responsibility Principle.".to_string(),
                risks: crate::strings![
                    "Single point of failure in the system",
                    "Difficult to understand and maintain",
                    "High risk of merge conflicts",
                    "Changes affect many parts of the system",
                    "Hard to test in isolation",
                    "Performance bottleneck potential"
                ],
                recommendations: crate::strings![
                    "Split the module by domain or functionality",
                    "Apply Single Responsibility Principle (SRP)",
                    "Extract utility functions into focused, single-purpose modules",
                    "Use facade pattern if the module serves as an integration point",
                    "Identify cohesive groups of functions and separate them",
                    "Consider creating a layered architecture to reduce coupling"
                ]
            }
        }
    );

    fn render_markdown(
        &self,
        gods: &[&crate::detectors::SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::explain::ExplainEngine;
        use std::path::PathBuf;

        crate::define_report_section!("God Modules", gods, {
            let mut body = String::new();
            for (smell, explanation) in gods {
                let file_path = smell
                    .files
                    .first()
                    .cloned()
                    .unwrap_or_else(|| PathBuf::from("Unknown"));

                let formatted_path = ExplainEngine::format_file_path(&file_path);
                let score = smell.score(severity_config);

                body.push_str(&format!("### {} ({} pts)\n\n", formatted_path, score));

                Self::generate_metrics_report(&mut body, smell);
                Self::generate_dependencies_report(&mut body, &file_path, graph);
                crate::report::markdown::common::append_explanation(&mut body, explanation);
            }
            body
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        // Check if git churn information is available
        let git_available = ctx.config.git.enabled && !ctx.churn_map.is_empty();

        ctx.graph
            .nodes()
            .filter_map(|node| {
                let fan_in = ctx.graph.fan_in(node);
                let fan_out = ctx.graph.fan_out(node);
                let path = ctx.graph.get_file_path(node)?;

                let rule = ctx.get_rule_for_file("god_module", path)?;

                let fan_in_threshold: usize = rule.get_option("fan_in").unwrap_or(10);
                let fan_out_threshold: usize = rule.get_option("fan_out").unwrap_or(10);
                let churn_threshold: usize = rule.get_option("churn").unwrap_or(20);

                let file_churn = ctx.churn_map.get(path).copied().unwrap_or(0);

                // If git is not available, we skip the churn threshold check
                let churn_ok = !git_available || file_churn >= churn_threshold;

                if fan_in >= fan_in_threshold && fan_out >= fan_out_threshold && churn_ok {
                    let mut smell =
                        ArchSmell::new_god_module(path.clone(), fan_in, fan_out, file_churn);
                    smell.severity = rule.severity;
                    Some(smell)
                } else {
                    None
                }
            })
            .collect()
    }
}
