use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;
use crate::parser::SymbolKind;
use petgraph::graph::NodeIndex;

pub fn init() {}

#[detector(
    id = "abstractness",
    name = "Abstractness vs Instability Violation Detector",
    description = "Detects modules that are far from the Main Sequence (Zone of Pain or Uselessness)",
    category = DetectorCategory::Global,
    default_enabled = false
)]
pub struct AbstractnessViolationDetector;

impl AbstractnessViolationDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn calculate_abstractness(&self, symbols: &crate::parser::FileSymbols) -> f64 {
        let total = symbols.exports.len();
        if total == 0 {
            return 0.0;
        }

        let abstract_count = symbols
            .exports
            .iter()
            .filter(|e| e.kind == SymbolKind::Interface || e.kind == SymbolKind::Type)
            .count();

        abstract_count as f64 / total as f64
    }

    fn calculate_instability(&self, ctx: &AnalysisContext, node: NodeIndex) -> f64 {
        let fan_in = ctx.graph.fan_in(node);
        let fan_out = ctx.graph.fan_out(node);
        if fan_in + fan_out == 0 {
            return 0.0;
        }
        fan_out as f64 / (fan_in + fan_out) as f64
    }
}

impl Detector for AbstractnessViolationDetector {
    crate::impl_detector_report!(
        name: "AbstractnessViolation",
        explain: _smell => {
            crate::detectors::Explanation {
                problem: "Abstractness Violation".into(),
                reason: "Module distance from the 'Main Sequence' is too high. This means it's either too stable and concrete (Zone of Pain) or too unstable and abstract (Zone of Uselessness).".into(),
                risks: crate::strings![
                    "Rigid code that is hard to change",
                    "Unused abstractions that add complexity"
                ],
                recommendations: crate::strings![
                    "Balance stability with abstractness by introducing interfaces or refactoring implementation details"
                ]
            }
        },
        table: {
            title: "Abstractness Violations",
            columns: ["File", "Distance", "pts"],
            row: AbstractnessViolation { } (smell, location, pts) => [
                location,
                format!("{:.2}", smell.metrics.iter().find_map(|m| if let crate::detectors::SmellMetric::Distance(d) = m { Some(*d) } else { None }).unwrap_or(0.0)),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                let rule = match ctx.get_rule_for_file("abstractness", path) {
                    Some(r) => r,
                    None => continue,
                };

                let distance_threshold: f64 = rule.get_option("distance_threshold").unwrap_or(0.85);

                if let Some(symbols) = ctx.file_symbols.get(path) {
                    let a = self.calculate_abstractness(symbols);
                    let i = self.calculate_instability(ctx, node);

                    let d = (a + i - 1.0).abs();

                    if d > distance_threshold {
                        let mut smell = ArchSmell::new_abstractness_violation(path.clone(), d);
                        smell.severity = rule.severity;
                        smells.push(smell);
                    }
                }
            }
        }

        smells
    }
}
