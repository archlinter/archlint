use crate::detectors::{detector, ArchSmell, Detector, Explanation};
use crate::engine::AnalysisContext;
use crate::graph::EdgeData;
use crate::parser::{ClassSymbol, FileSymbols, SymbolKind};
use petgraph::graph::NodeIndex;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

#[detector(SmellType::AbstractnessViolation, default_enabled = false)]
pub struct AbstractnessViolationDetector;

impl AbstractnessViolationDetector {
    #[must_use]
    pub const fn new_default(_config: &crate::config::Config) -> Self {
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

        f64::from(abstract_usages) / incoming_edges.len() as f64
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

    fn should_skip_node(
        &self,
        ctx: &AnalysisContext,
        node: NodeIndex,
        rule: &crate::rule_resolver::ResolvedRuleConfig,
        a: f64,
        i: f64,
        d: f64,
    ) -> bool {
        let fan_in = ctx.graph.fan_in(node);
        let fan_in_threshold: usize = rule.get_option("fan_in_threshold").unwrap_or(10);
        let distance_threshold: f64 = rule.get_option("distance_threshold").unwrap_or(0.85);

        // 1. Check if we should ignore this file based on stability and abstractness
        // Zone of Pain: Needs high fan-in to be relevant.
        // Zone of Uselessness: Needs high abstractness but low fan-in is exactly the point.
        let is_potential_pain = i < 0.5 && a < 0.5;
        if is_potential_pain && fan_in < fan_in_threshold {
            return true;
        }

        d <= distance_threshold
    }

    fn has_active_components(&self, symbols: &FileSymbols) -> bool {
        symbols
            .classes
            .iter()
            .any(|c| self.is_active_class(symbols, c))
    }

    fn is_active_class(&self, symbols: &FileSymbols, c: &ClassSymbol) -> bool {
        // 1. Must be exported
        let is_exported = symbols
            .exports
            .iter()
            .any(|e| e.kind == SymbolKind::Class && e.name == c.name);

        if !is_exported {
            return false;
        }

        // 2. Exclude abstract classes (they are already abstractions by definition)
        if c.is_abstract {
            return false;
        }

        // 3. Exclude simple Errors
        if let Some(super_class) = &c.super_class {
            if super_class.to_lowercase().contains("error") {
                return false;
            }
        }

        // 4. Exclude Data Carriers (DTOs, Entities, simple structures)
        if c.methods.is_empty() {
            return false;
        }

        // 5. Exclude generic infrastructure patterns (like Migrations)
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
                    "Module distance from the 'Main Sequence' is {distance:.2}. It is in the {zone}. Current Abstractness: {abstractness:.2}, Instability: {instability:.2}."
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
        ctx.graph
            .nodes()
            .filter_map(|node| {
                let path = ctx.graph.get_file_path(node)?;
                let symbols = ctx.file_symbols.get(path)?;
                let rule = ctx.get_rule_for_file("abstractness", path)?;

                let fan_in = ctx.graph.fan_in(node);
                let a = self.calculate_abstractness(ctx, path, node);
                let i = self.calculate_instability(ctx, node);
                let d = (a + i - 1.0).abs();

                if self.should_skip_node(ctx, node, &rule, a, i, d) {
                    return None;
                }

                if !self.has_active_components(symbols) {
                    return None;
                }

                let mut smell =
                    ArchSmell::new_abstractness_violation(path.clone(), d, a, i, fan_in);
                smell.severity = rule.severity;
                Some(smell)
            })
            .collect()
    }
}
