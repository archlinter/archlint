use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use crate::parser::SymbolKind;
use inventory;
use petgraph::graph::NodeIndex;

pub fn init() {}

pub struct AbstractnessViolationDetector;

pub struct AbstractnessViolationDetectorFactory;

impl DetectorFactory for AbstractnessViolationDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "abstractness",
            name: "Abstractness vs Instability Violation Detector",
            description:
                "Detects modules that are far from the Main Sequence (Zone of Pain or Uselessness)",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::Global,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(AbstractnessViolationDetector)
    }
}

inventory::submit! {
    &AbstractnessViolationDetectorFactory as &dyn DetectorFactory
}

impl Detector for AbstractnessViolationDetector {
    fn name(&self) -> &'static str {
        "AbstractnessViolation"
    }

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

impl AbstractnessViolationDetector {
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
