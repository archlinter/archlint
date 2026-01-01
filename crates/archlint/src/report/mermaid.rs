use crate::detectors::{ArchSmell, SmellType};
use crate::graph::DependencyGraph;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub struct MermaidGenerator;

impl MermaidGenerator {
    pub fn generate(
        smells: &[(ArchSmell, crate::explain::Explanation)],
        graph: &DependencyGraph,
    ) -> String {
        // Collect all problematic files
        let mut problem_files = HashSet::new();
        let mut file_smells: HashMap<std::path::PathBuf, Vec<&SmellType>> = HashMap::new();

        for (smell, _) in smells {
            for file in &smell.files {
                problem_files.insert(file.clone());
                file_smells
                    .entry(file.clone())
                    .or_default()
                    .push(&smell.smell_type);
            }
        }

        if problem_files.is_empty() {
            return String::new();
        }

        let mut output = String::from("```mermaid\nflowchart LR\n");

        // Create nodes for problem files
        let mut node_ids = HashMap::new();
        for (idx, file) in problem_files.iter().enumerate() {
            let node_id = format!("file{}", idx);
            let file_name = Self::get_short_name(file);

            // Determine CSS class based on smell types
            let classes = Self::get_css_classes(file_smells.get(file).map(|v| v.as_slice()).unwrap_or(&[]));

            if classes.is_empty() {
                output.push_str(&format!("    {}[{}]\n", node_id, file_name));
            } else {
                output.push_str(&format!("    {}[{}]:::{}\n", node_id, file_name, classes));
            }

            node_ids.insert(file.clone(), node_id);
        }

        output.push('\n');

        // Add edges between problem files
        let mut added_edges = HashSet::new();
        for file in &problem_files {
            if let Some(source_idx) = graph.get_node(file) {
                for target_idx in graph.dependencies(source_idx) {
                    if let Some(target_file) = graph.get_file_path(target_idx) {
                        if problem_files.contains(target_file) {
                            let source_id = node_ids.get(file).unwrap();
                            let target_id = node_ids.get(target_file).unwrap();
                            let edge = (source_id.clone(), target_id.clone());

                            if !added_edges.contains(&edge) {
                                output.push_str(&format!("    {} --> {}\n", source_id, target_id));
                                added_edges.insert(edge);
                            }
                        }
                    }
                }
            }
        }

        // Add legend/styles
        output.push_str("\n    classDef deadCode fill:#e0e0e0,stroke:#999,stroke-dasharray:5\n");
        output.push_str("    classDef deadSymbol fill:#f0f0f0,stroke:#bbb,stroke-dasharray:3\n");
        output.push_str("    classDef godModule fill:#ffcccc,stroke:#cc0000\n");
        output.push_str("    classDef cycle fill:#ffffcc,stroke:#cccc00\n");
        output.push_str("    classDef complexity fill:#f9f,stroke:#333\n");
        output.push_str("    classDef unstableInterface fill:#ff9966,stroke:#993300\n");
        output.push_str("    classDef featureEnvy fill:#99ccff,stroke:#003366\n");
        output.push_str("    classDef shotgunSurgery fill:#ff6666,stroke:#660000\n");
        output.push_str("    classDef hubDependency fill:#cc99ff,stroke:#330066\n");
        output.push_str("    classDef testLeakage fill:#ff6666,stroke:#cc0000,stroke-width:2px\n");
        output.push_str("    classDef layerViolation fill:#ff3333,stroke:#990000,stroke-width:3px\n");
        output.push_str("    classDef sdpViolation fill:#ffcc99,stroke:#ff6600\n");

        output.push_str("```\n");

        output
    }

    fn get_short_name(path: &Path) -> String {
        // Get just the filename without full path
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        // Sanitize for Mermaid (no special chars that break syntax)
        file_name
            .replace(['.', '-', ' '], "_")
    }

    fn get_css_classes(smell_types: &[&SmellType]) -> String {
        let mut classes = Vec::new();

        for smell_type in smell_types {
            match smell_type {
                SmellType::DeadCode => classes.push("deadCode"),
                SmellType::GodModule => classes.push("godModule"),
                SmellType::CyclicDependency => classes.push("cycle"),
                SmellType::CyclicDependencyCluster => classes.push("cycle"),
                SmellType::DeadSymbol { .. } => classes.push("deadSymbol"),
                SmellType::HighComplexity { .. } => classes.push("complexity"),
                SmellType::LargeFile => classes.push("largeFile"),
                SmellType::UnstableInterface => classes.push("unstableInterface"),
                SmellType::FeatureEnvy { .. } => classes.push("featureEnvy"),
                SmellType::ShotgunSurgery => classes.push("shotgunSurgery"),
                SmellType::HubDependency { .. } => classes.push("hubDependency"),
                SmellType::TestLeakage { .. } => classes.push("testLeakage"),
                SmellType::LayerViolation { .. } => classes.push("layerViolation"),
                SmellType::SdpViolation => classes.push("sdpViolation"),

                _ => {}
            }
        }

        // Return first class (file can have multiple smells, but mermaid supports one class)
        classes.first().copied().unwrap_or("").to_string()
    }
}
