use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;
use petgraph::graph::UnGraph;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(SmellType::LowCohesion, default_enabled = false)]
pub struct LcomDetector;

impl LcomDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn calculate_lcom4(&self, class: &crate::parser::ClassSymbol) -> usize {
        let methods: Vec<_> = class
            .methods
            .iter()
            .filter(|m| m.name != "constructor" && !m.is_accessor)
            .collect();

        if methods.is_empty() {
            return 1;
        }

        let mut graph = UnGraph::<(), ()>::new_undirected();
        let mut method_nodes = Vec::new();

        for _ in 0..methods.len() {
            method_nodes.push(graph.add_node(()));
        }

        for i in 0..methods.len() {
            for j in (i + 1)..methods.len() {
                let m1 = methods[i];
                let m2 = methods[j];

                // methods are connected if they share a field
                let shares_field = m1.used_fields.iter().any(|f| m2.used_fields.contains(f));

                // or if one calls the other
                let calls_each_other =
                    m1.used_methods.contains(&m2.name) || m2.used_methods.contains(&m1.name);

                if shares_field || calls_each_other {
                    graph.add_edge(method_nodes[i], method_nodes[j], ());
                }
            }
        }

        petgraph::algo::connected_components(&graph)
    }
}

impl Detector for LcomDetector {
    crate::impl_detector_report!(
        explain: _smell => (
            problem: "Low Cohesion of Methods (LCOM)",
            reason: "The methods in this class don't share common fields, suggesting the class might be doing too many unrelated things.",
            risks: [
                "Violation of SRP",
                "Difficult to maintain and test"
            ],
            recommendations: [
                "Split the class into smaller, more focused classes"
            ]
        ),
        table: {
            title: "Low Cohesion (LCOM)",
            columns: ["Class", "LCOM4 Score", "pts"],
            row: LowCohesion { lcom, class_name } (smell, location, pts) => [
                class_name,
                lcom,
                pts
            ]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let rule = match ctx.get_rule_for_file("lcom", path) {
                Some(r) => r,
                None => continue,
            };

            let min_methods: usize = rule.get_option("min_methods").unwrap_or(3);
            let max_lcom: usize = rule.get_option("max_lcom").unwrap_or(4);

            for class in &symbols.classes {
                if class.methods.len() < min_methods {
                    continue;
                }

                let lcom4 = self.calculate_lcom4(class);

                if lcom4 > max_lcom {
                    let mut smell =
                        ArchSmell::new_low_cohesion(path.clone(), class.name.to_string(), lcom4);
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
