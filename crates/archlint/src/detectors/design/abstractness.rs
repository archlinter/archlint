use crate::detectors::{detector, ArchSmell, Detector, Explanation};
use crate::engine::AnalysisContext;
use crate::parser::SymbolKind;
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
            // Fallback to classic abstractness calculation if no one is using the module yet.
            // A = (Number of exported interfaces/types) / (Total exported classes/functions/interfaces/types)
            if let Some(symbols) = ctx.file_symbols.get(path) {
                let abstract_count = symbols
                    .exports
                    .iter()
                    .filter(|e| e.kind == SymbolKind::Interface || e.kind == SymbolKind::Type)
                    .count();
                let total_count = symbols
                    .exports
                    .iter()
                    .filter(|e| {
                        e.kind == SymbolKind::Class
                            || e.kind == SymbolKind::Function
                            || e.kind == SymbolKind::Interface
                            || e.kind == SymbolKind::Type
                    })
                    .count();

                if total_count == 0 {
                    return 0.0;
                }
                return abstract_count as f64 / total_count as f64;
            }
            return 0.0;
        }

        let mut abstract_usages = 0;
        let total_usages = incoming_edges.len();

        for edge in incoming_edges {
            let edge_data = edge.weight();

            // If there are no imported symbols information, we can't be sure,
            // so we treat it as concrete to be safe (or neutral).
            if edge_data.imported_symbols.is_empty() {
                continue;
            }

            // Check if any of the imported symbols are concrete (Classes or Functions)
            let has_concrete_import = edge_data.imported_symbols.iter().any(|symbol_name| {
                if let Some(our_symbols) = ctx.file_symbols.get(path) {
                    our_symbols.exports.iter().any(|e| {
                        e.name.as_str() == symbol_name
                            && (e.kind == SymbolKind::Class || e.kind == SymbolKind::Function)
                    })
                } else {
                    false
                }
            });

            if !has_concrete_import {
                abstract_usages += 1;
            }
        }

        abstract_usages as f64 / total_usages as f64
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
            columns: ["File", "Zone", "Distance", "Abstractness", "Instability", "In", "pts"],
            row: AbstractnessViolation (smell, location, pts) => [
                location,
                {
                    let a = smell.abstractness().unwrap_or(0.0);
                    let i = smell.instability().unwrap_or(0.0);
                    if a + i < 1.0 { "Pain" } else { "Uselessness" }
                },
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

                            // A. Exclude abstract classes (they are already abstractions by definition)
                            if c.is_abstract {
                                return false;
                            }

                            // B. Exclude simple Errors (generic check for any class extending something with "Error")
                            if let Some(super_class) = &c.super_class {
                                let sc = super_class.to_lowercase();
                                if sc.contains("error") {
                                    return false;
                                }
                            }

                            // C. Exclude Data Carriers (DTOs, Entities, simple structures)
                            // A class with no methods is just a data container.
                            if c.methods.is_empty() {
                                return false;
                            }

                            // D. Exclude generic infrastructure patterns (like Migrations)
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
