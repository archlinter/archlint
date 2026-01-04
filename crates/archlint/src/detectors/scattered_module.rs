use crate::config::Config;
use crate::detectors::DetectorCategory;
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use petgraph::graph::UnGraph;
use std::collections::HashSet;

pub fn init() {}

pub struct ScatteredModuleDetector;

pub struct ScatteredModuleDetectorFactory;

impl DetectorFactory for ScatteredModuleDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "scattered_module",
            name: "Scattered Module Detector",
            description: "Detects modules where exports are unrelated to each other",
            default_enabled: false,
            is_deep: false,
            category: DetectorCategory::Global,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(ScatteredModuleDetector)
    }
}

inventory::submit! {
    &ScatteredModuleDetectorFactory as &dyn DetectorFactory
}

impl Detector for ScatteredModuleDetector {
    fn name(&self) -> &'static str {
        "ScatteredModule"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let thresholds = &ctx.config.thresholds.module_cohesion;

        for (path, symbols) in ctx.file_symbols.as_ref() {
            if ctx.should_skip_detector(path, "scattered_module") {
                continue;
            }

            // Ignore small files and barrels (barrels are handled by BarrelFileAbuseDetector)
            if symbols.exports.len() < thresholds.min_exports || self.is_barrel_file(path, symbols)
            {
                continue;
            }

            let components = self.calculate_components(symbols);

            if components > thresholds.max_components {
                smells.push(ArchSmell::new_scattered_module(path.clone(), components));
            }
        }

        smells
    }
}

impl ScatteredModuleDetector {
    fn is_barrel_file(&self, path: &std::path::Path, symbols: &crate::parser::FileSymbols) -> bool {
        let is_index = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("index."))
            .unwrap_or(false);

        let only_reexports = symbols.exports.iter().all(|e| e.source.is_some());

        is_index || only_reexports
    }

    fn calculate_components(&self, symbols: &crate::parser::FileSymbols) -> usize {
        let mut graph = UnGraph::<(), ()>::new_undirected();
        let mut export_nodes = Vec::new();

        for _ in 0..symbols.exports.len() {
            export_nodes.push(graph.add_node(()));
        }

        for i in 0..symbols.exports.len() {
            for j in (i + 1)..symbols.exports.len() {
                let e1 = &symbols.exports[i];
                let e2 = &symbols.exports[j];

                // Check if they share any used symbols
                let shared: HashSet<_> = e1.used_symbols.intersection(&e2.used_symbols).collect();

                // If they share symbols or one uses the other
                let one_uses_other =
                    e1.used_symbols.contains(&e2.name) || e2.used_symbols.contains(&e1.name);

                if !shared.is_empty() || one_uses_other {
                    graph.add_edge(export_nodes[i], export_nodes[j], ());
                }
            }
        }

        petgraph::algo::connected_components(&graph)
    }
}
