use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use petgraph::graph::UnGraph;

pub fn init() {}

pub struct LcomDetector;

pub struct LcomDetectorFactory;

impl DetectorFactory for LcomDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "lcom",
            name: "Lack of Cohesion of Methods (LCOM4)",
            description:
                "Detects classes with low cohesion where methods don't share common fields",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::FileLocal,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(LcomDetector)
    }
}

inventory::submit! {
    &LcomDetectorFactory as &dyn DetectorFactory
}

impl Detector for LcomDetector {
    fn name(&self) -> &'static str {
        "Lcom"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.lcom;

        for (path, symbols) in ctx.file_symbols.as_ref() {
            if ctx.is_excluded(path, &thresholds.exclude_patterns)
                || ctx.should_skip_detector(path, "lcom")
            {
                continue;
            }
            for class in &symbols.classes {
                if class.methods.len() < thresholds.min_methods {
                    continue;
                }

                let lcom4 = self.calculate_lcom4(class);

                if lcom4 > thresholds.max_lcom {
                    smells.push(ArchSmell::new_low_cohesion(
                        path.clone(),
                        class.name.to_string(),
                        lcom4,
                    ));
                }
            }
        }

        smells
    }
}

impl LcomDetector {
    fn calculate_lcom4(&self, class: &crate::parser::ClassSymbol) -> usize {
        let mut graph = UnGraph::<(), ()>::new_undirected();
        let mut method_nodes = Vec::new();

        for _ in 0..class.methods.len() {
            method_nodes.push(graph.add_node(()));
        }

        for i in 0..class.methods.len() {
            for j in (i + 1)..class.methods.len() {
                let m1 = &class.methods[i];
                let m2 = &class.methods[j];

                // methods are connected if they share a field
                let shares_field = m1.used_fields.iter().any(|f| m2.used_fields.contains(f));

                // or if one calls the other (simplified: if m1 uses m2's name)
                // we don't have perfect call graph yet, but we can check used_methods
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
