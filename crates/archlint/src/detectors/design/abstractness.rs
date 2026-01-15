use crate::detectors::{detector, ArchSmell, Detector, Explanation};
use crate::engine::AnalysisContext;
use crate::graph::EdgeData;
use crate::parser::{FileSymbols, SymbolKind};
use petgraph::graph::NodeIndex;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(SmellType::AbstractnessViolation, default_enabled = false)]
pub struct AbstractnessViolationDetector;

impl AbstractnessViolationDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn calculate_abstractness(
        &self,
        ctx: &AnalysisContext,
        path: &std::path::PathBuf,
        node: NodeIndex,
    ) -> f64 {
        let incoming_edges: Vec<_> = ctx
            .graph
            .graph()
            .edges_directed(node, petgraph::Direction::Incoming)
            .collect();

        if incoming_edges.is_empty() {
            return self.calculate_fallback_abstractness(ctx, path);
        }

        let mut abstract_usages = 0;
        let our_symbols = ctx.file_symbols.get(path);

        for edge in &incoming_edges {
            if self.is_abstract_usage(edge.weight(), our_symbols) {
                abstract_usages += 1;
            }
        }

        abstract_usages as f64 / incoming_edges.len() as f64
    }

    fn calculate_fallback_abstractness(
        &self,
        ctx: &AnalysisContext,
        path: &std::path::PathBuf,
    ) -> f64 {
        let symbols = match ctx.file_symbols.get(path) {
            Some(s) => s,
            None => return 0.0,
        };

        let abstract_count = symbols
            .exports
            .iter()
            .filter(|e| matches!(e.kind, SymbolKind::Interface | SymbolKind::Type))
            .count();

        let total_count = symbols
            .exports
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    SymbolKind::Class
                        | SymbolKind::Function
                        | SymbolKind::Interface
                        | SymbolKind::Type
                )
            })
            .count();

        if total_count == 0 {
            return 0.0;
        }
        abstract_count as f64 / total_count as f64
    }

    fn is_abstract_usage(&self, edge_data: &EdgeData, symbols: Option<&FileSymbols>) -> bool {
        if edge_data.imported_symbols.is_empty() {
            return false;
        }

        !edge_data.imported_symbols.iter().any(|symbol_name| {
            symbols.is_some_and(|s| {
                s.exports.iter().any(|e| {
                    e.name.as_str() == symbol_name
                        && matches!(e.kind, SymbolKind::Class | SymbolKind::Function)
                })
            })
        })
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
        explain: smell => {
            let distance = smell.distance().unwrap_or(0.0);
            let abstractness = smell.abstractness().unwrap_or(0.0);
            let instability = smell.instability().unwrap_or(0.0);

            let zone = if abstractness + instability < 1.0 {
                "Zone of Pain (Too stable and concrete - changes are hard because many depend on this concrete implementation)"
            } else {
                "Zone of Uselessness (Too abstract and unstable - abstractions are defined but not used enough to justify them)"
            };

            Explanation {
                problem: "Abstractness Violation".to_string(),
                reason: format!(
                    "Module distance from the 'Main Sequence' is {:.2}. It is in the {}. Current Abstractness: {:.2}, Instability: {:.2}.",
                    distance, zone, abstractness, instability
                ),
                risks: vec![
                    "Rigid code that is hard to change (if in Zone of Pain)".to_string(),
                    "Unused abstractions that add complexity (if in Zone of Uselessness)".to_string(),
                ],
                recommendations: vec![
                    "In Zone of Pain: Introduce interfaces/abstract classes to decouple dependencies.".to_string(),
                    "In Zone of Uselessness: Make the module more concrete or ensure abstractions are actually needed.".to_string(),
                    "For utility/constant/enum files: This rule might be too strict; consider ignoring it for these file types.".to_string(),
                ],
            }
        },
        table: {
            title: "Abstractness Violations",
            columns: ["File", "Distance", "Abstractness", "Instability", "In", "pts"],
            row: AbstractnessViolation (smell, location, pts) => [
                location,
                format!("{:.2}", smell.distance().unwrap_or(0.0)),
                format!("{:.2}", smell.abstractness().unwrap_or(0.0)),
                format!("{:.2}", smell.instability().unwrap_or(0.0)),
                smell.fan_in().unwrap_or(0).to_string(),
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for node in ctx.graph.nodes() {
            if let Some(path) = ctx.graph.get_file_path(node) {
                if let Some(symbols) = ctx.file_symbols.get(path) {
                    let rule = match ctx.get_rule_for_file("abstractness", path) {
                        Some(r) => r,
                        None => continue,
                    };

                    let fan_in = ctx.graph.fan_in(node);
                    let fan_in_threshold: usize = rule.get_option("fan_in_threshold").unwrap_or(10);
                    let distance_threshold: f64 =
                        rule.get_option("distance_threshold").unwrap_or(0.85);

                    let a = self.calculate_abstractness(ctx, path, node);
                    let i = self.calculate_instability(ctx, node);
                    let d = (a + i - 1.0).abs();

                    // 1. Check if we should ignore this file based on stability and abstractness
                    // Zone of Pain: Needs high fan-in to be relevant.
                    // Zone of Uselessness: Needs high abstractness but low fan-in is exactly the point.
                    let is_potential_pain = i < 0.5 && a < 0.5;
                    if is_potential_pain && fan_in < fan_in_threshold {
                        continue;
                    }

                    if d <= distance_threshold {
                        continue;
                    }

                    // 2. Identify "Active" components by checking exported classes
                    let active_exported_classes: Vec<_> = symbols
                        .classes
                        .iter()
                        .filter(|c| {
                            // Must be exported
                            let is_exported = symbols
                                .exports
                                .iter()
                                .any(|e| e.kind == SymbolKind::Class && e.name == c.name);

                            if !is_exported {
                                return false;
                            }

                            // A. Exclude simple Errors (generic check for any class extending something with "Error")
                            if let Some(super_class) = &c.super_class {
                                let sc = super_class.to_lowercase();
                                if sc.contains("error") {
                                    return false;
                                }
                            }

                            // B. Exclude Data Carriers (DTOs, Entities, simple structures)
                            // A class with no methods is just a data container.
                            if c.methods.is_empty() {
                                return false;
                            }

                            // C. Exclude generic infrastructure patterns (like Migrations)
                            // Migrations usually have up/down methods and no other logic.
                            let method_names: Vec<String> = c
                                .methods
                                .iter()
                                .map(|m| m.name.to_lowercase().to_string())
                                .collect();
                            if method_names.len() <= 2
                                && (method_names.contains(&"up".to_string())
                                    || method_names.contains(&"down".to_string()))
                            {
                                return false;
                            }

                            true
                        })
                        .collect();

                    if active_exported_classes.is_empty() {
                        continue;
                    }

                    let distance_threshold: f64 =
                        rule.get_option("distance_threshold").unwrap_or(0.85);

                    let a = self.calculate_abstractness(ctx, path, node);
                    let i = self.calculate_instability(ctx, node);

                    let d = (a + i - 1.0).abs();

                    if d > distance_threshold {
                        let mut smell =
                            ArchSmell::new_abstractness_violation(path.clone(), d, a, i, fan_in);
                        smell.severity = rule.severity;
                        smells.push(smell);
                    }
                }
            }
        }

        smells
    }
}
