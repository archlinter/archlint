use crate::config::{Config, LayerConfig};
use crate::detectors::{ArchSmell, Detector, DetectorFactory, DetectorInfo};
use crate::engine::AnalysisContext;
use inventory;
use std::path::Path;

pub fn init() {}

pub struct LayerViolationDetector;

pub struct LayerViolationDetectorFactory;

impl DetectorFactory for LayerViolationDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "layer_violation",
            name: "Layer Architecture Violation Detector",
            description: "Detects violations of layered architecture rules",
            default_enabled: false,
            is_deep: false,
        }
    }

    fn create(&self, _config: &Config) -> Box<dyn Detector> {
        Box::new(LayerViolationDetector)
    }
}

inventory::submit! {
    &LayerViolationDetectorFactory as &dyn DetectorFactory
}

impl Detector for LayerViolationDetector {
    fn name(&self) -> &'static str {
        "LayerViolation"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let layers = &ctx.config.thresholds.layer_violation.layers;

        if layers.is_empty() {
            return smells;
        }

        for node in ctx.graph.nodes() {
            if let Some(from_path) = ctx.graph.get_file_path(node) {
                if let Some(from_layer) = self.find_layer(from_path, layers) {
                    for to_node in ctx.graph.dependencies(node) {
                        if let Some(to_path) = ctx.graph.get_file_path(to_node) {
                            if let Some(to_layer) = self.find_layer(to_path, layers) {
                                // Check if this import is allowed
                                if from_layer.name != to_layer.name
                                    && !from_layer.allowed_imports.contains(&to_layer.name)
                                {
                                    smells.push(ArchSmell::new_layer_violation(
                                        from_path.clone(),
                                        to_path.clone(),
                                        from_layer.name.clone(),
                                        to_layer.name.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        smells
    }
}

impl LayerViolationDetector {
    fn find_layer<'a>(&self, path: &Path, layers: &'a [LayerConfig]) -> Option<&'a LayerConfig> {
        // Find the most specific layer (longest matching path)
        let mut matching_layers: Vec<&LayerConfig> = layers
            .iter()
            .filter(|l| self.matches_path(path, &l.path))
            .collect();

        matching_layers.sort_by_key(|l| std::cmp::Reverse(l.path.len()));

        matching_layers.first().copied()
    }

    fn matches_path(&self, path: &Path, pattern: &str) -> bool {
        let path_str = path.to_string_lossy();

        if pattern.contains("**") {
            let cleaned = pattern.replace("**", "");
            let parts: Vec<&str> = cleaned.split('*').filter(|p| !p.is_empty()).collect();
            if parts.iter().all(|part| path_str.contains(part)) {
                return true;
            }
        } else if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').filter(|p| !p.is_empty()).collect();
            if parts.iter().all(|part| path_str.contains(part)) {
                return true;
            }
        } else if path_str.contains(pattern) {
            return true;
        }

        false
    }
}
