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
        let (problem_files, file_smells) = Self::collect_problem_data(smells);

        if problem_files.is_empty() {
            return String::new();
        }

        let mut output = String::from("```mermaid\nflowchart LR\n");

        let node_ids = Self::generate_nodes(&mut output, &problem_files, &file_smells);
        output.push('\n');

        Self::generate_edges(&mut output, &problem_files, &node_ids, graph);

        Self::append_class_definitions(&mut output);

        output.push_str("```\n");

        output
    }

    fn collect_problem_data(
        smells: &[(ArchSmell, crate::explain::Explanation)],
    ) -> (
        HashSet<std::path::PathBuf>,
        HashMap<std::path::PathBuf, Vec<&SmellType>>,
    ) {
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

        (problem_files, file_smells)
    }

    fn generate_nodes(
        output: &mut String,
        problem_files: &HashSet<std::path::PathBuf>,
        file_smells: &HashMap<std::path::PathBuf, Vec<&SmellType>>,
    ) -> HashMap<std::path::PathBuf, String> {
        let mut node_ids = HashMap::new();
        for (idx, file) in problem_files.iter().enumerate() {
            let node_id = format!("file{}", idx);
            let file_name = Self::get_short_name(file);

            let classes =
                Self::get_css_classes(file_smells.get(file).map(|v| v.as_slice()).unwrap_or(&[]));

            if classes.is_empty() {
                output.push_str(&format!("    {}[{}]\n", node_id, file_name));
            } else {
                output.push_str(&format!("    {}[{}]:::{}\n", node_id, file_name, classes));
            }

            node_ids.insert(file.clone(), node_id);
        }
        node_ids
    }

    fn generate_edges(
        output: &mut String,
        problem_files: &HashSet<std::path::PathBuf>,
        node_ids: &HashMap<std::path::PathBuf, String>,
        graph: &DependencyGraph,
    ) {
        let mut added_edges = HashSet::new();
        for file in problem_files {
            Self::generate_file_edges(
                output,
                file,
                problem_files,
                node_ids,
                graph,
                &mut added_edges,
            );
        }
    }

    fn generate_file_edges(
        output: &mut String,
        file: &std::path::PathBuf,
        problem_files: &HashSet<std::path::PathBuf>,
        node_ids: &HashMap<std::path::PathBuf, String>,
        graph: &DependencyGraph,
        added_edges: &mut HashSet<(String, String)>,
    ) {
        let source_idx = match graph.get_node(file) {
            Some(idx) => idx,
            None => return,
        };

        let source_id = match node_ids.get(file) {
            Some(id) => id,
            None => return,
        };

        for target_idx in graph.dependencies(source_idx) {
            let target_file = match graph.get_file_path(target_idx) {
                Some(f) if problem_files.contains(f) => f,
                _ => continue,
            };

            let target_id = match node_ids.get(target_file) {
                Some(id) => id,
                None => continue,
            };

            let edge = (source_id.clone(), target_id.clone());
            if added_edges.insert(edge) {
                output.push_str(&format!("    {} --> {}\n", source_id, target_id));
            }
        }
    }

    fn append_class_definitions(output: &mut String) {
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
        output
            .push_str("    classDef layerViolation fill:#ff3333,stroke:#990000,stroke-width:3px\n");
        output.push_str("    classDef sdpViolation fill:#ffcc99,stroke:#ff6600\n");
    }

    fn get_short_name(path: &Path) -> String {
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        file_name.replace(['.', '-', ' '], "_")
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

        classes.first().copied().unwrap_or("").to_string()
    }
}
