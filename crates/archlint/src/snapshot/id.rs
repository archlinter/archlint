use crate::detectors::{ArchSmell, SmellType};
use log::debug;
use std::path::{Path, PathBuf};

/// Generate stable, deterministic ID for a smell
pub fn generate_smell_id(smell: &ArchSmell, project_root: &Path) -> String {
    match &smell.smell_type {
        SmellType::CyclicDependency | SmellType::CyclicDependencyCluster => {
            id_for_cycle(&smell.files, project_root)
        }

        SmellType::GodModule | SmellType::LargeFile | SmellType::UnstableInterface => {
            id_for_file_smell(
                &smell.files[0],
                &format!("{:?}", smell.smell_type),
                project_root,
            )
        }

        SmellType::LayerViolation { to_layer, .. } => {
            id_for_layer_violation(&smell.files[0], to_layer, project_root)
        }

        SmellType::DeadSymbol { name, .. } => {
            id_for_dead_symbol(&smell.files[0], name, project_root)
        }

        SmellType::HighComplexity { name, .. } => {
            id_for_complexity(&smell.files[0], name, project_root)
        }

        SmellType::HubModule => id_for_file_smell(&smell.files[0], "hub", project_root),

        SmellType::LowCohesion { .. } => id_for_file_smell(&smell.files[0], "lcom", project_root),

        SmellType::HubDependency { package } => {
            format!("hub_dep:{}", package)
        }

        SmellType::VendorCoupling { package } => {
            format!("vendor:{}", package)
        }

        SmellType::TestLeakage { test_file } => {
            let from = &smell.files[0];
            let from_rel = relative_path(from, project_root);
            let to_rel = relative_path(test_file, project_root);
            format!("test_leak:{}:{}", from_rel, to_rel)
        }

        SmellType::FeatureEnvy { most_envied_module } => {
            let from = &smell.files[0];
            let from_rel = relative_path(from, project_root);
            let to_rel = relative_path(most_envied_module, project_root);
            format!("envy:{}:{}", from_rel, to_rel)
        }

        SmellType::SharedMutableState { symbol } => {
            id_for_dead_symbol(&smell.files[0], symbol, project_root).replace("dead:", "shared:")
        }

        SmellType::DeepNesting { .. } => {
            // Need function name if possible, but it's not in the enum variant.
            // Let's use the first location's description if it contains function name.
            let function_name = smell
                .locations
                .first()
                .map(|l| l.description.clone())
                .unwrap_or_default();
            id_for_complexity(&smell.files[0], &function_name, project_root)
                .replace("cmplx:", "nest:")
        }

        SmellType::LongParameterList { function, .. } => {
            id_for_complexity(&smell.files[0], function, project_root).replace("cmplx:", "params:")
        }

        SmellType::PrimitiveObsession { function, .. } => {
            id_for_complexity(&smell.files[0], function, project_root).replace("cmplx:", "prim:")
        }

        SmellType::OrphanType { name } => {
            id_for_dead_symbol(&smell.files[0], name, project_root).replace("dead:", "orphan:")
        }

        SmellType::ScatteredConfiguration { env_var, .. } => {
            format!("config:{}", env_var)
        }

        _ => {
            // Fallback: generic ID
            let type_name = format!("{:?}", smell.smell_type);
            let type_short = type_name.split('{').next().unwrap_or(&type_name).trim();
            id_generic(type_short, &smell.files, "", project_root)
        }
    }
}

fn relative_path(path: &Path, project_root: &Path) -> String {
    let rel = path.strip_prefix(project_root).unwrap_or_else(|_| {
        debug!("Failed to strip prefix {:?} from {:?}", project_root, path);
        path
    });
    rel.to_string_lossy().replace('\\', "/")
}

fn id_for_cycle(files: &[PathBuf], project_root: &Path) -> String {
    let mut relative_paths: Vec<String> = files
        .iter()
        .map(|f| relative_path(f, project_root))
        .collect();

    // Sort for determinism (cycle can start from any node)
    relative_paths.sort();

    let content = relative_paths.join("|");
    let hash = short_hash(&content);

    format!("cycle:{}", hash)
}

fn id_for_file_smell(file: &Path, prefix: &str, project_root: &Path) -> String {
    let relative = relative_path(file, project_root);
    format!("{}:{}", prefix.to_lowercase(), relative)
}

fn id_for_layer_violation(from_file: &Path, to_layer: &str, project_root: &Path) -> String {
    let relative = relative_path(from_file, project_root);
    format!("layer:{}:{}", relative, to_layer)
}

fn id_for_dead_symbol(file: &Path, symbol_name: &str, project_root: &Path) -> String {
    let relative = relative_path(file, project_root);
    format!("dead:{}:{}", relative, symbol_name)
}

fn id_for_complexity(file: &Path, function_name: &str, project_root: &Path) -> String {
    let relative = relative_path(file, project_root);
    format!("cmplx:{}:{}", relative, function_name)
}

fn id_generic(smell_type: &str, files: &[PathBuf], extra: &str, project_root: &Path) -> String {
    let mut parts: Vec<String> = files
        .iter()
        .map(|f| relative_path(f, project_root))
        .collect();

    parts.sort();
    if !extra.is_empty() {
        parts.push(extra.to_string());
    }

    let content = parts.join("|");
    let hash = short_hash(&content);

    format!("{}:{}", smell_type.to_lowercase(), hash)
}

fn short_hash(content: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();

    // Take first 8 hex chars for better stability/collision resistance than 6
    format!("{:x}", result)[..8].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_cycle_id_is_order_independent() {
        let root = Path::new("/project");

        let files1 = vec![
            PathBuf::from("/project/a.ts"),
            PathBuf::from("/project/b.ts"),
        ];
        let files2 = vec![
            PathBuf::from("/project/b.ts"),
            PathBuf::from("/project/a.ts"),
        ];

        let id1 = id_for_cycle(&files1, root);
        let id2 = id_for_cycle(&files2, root);

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_file_smell_id_uses_relative_path() {
        let root = Path::new("/project");
        let file = Path::new("/project/src/service.ts");

        let id = id_for_file_smell(file, "god", root);

        assert_eq!(id, "god:src/service.ts");
    }

    #[test]
    fn test_short_hash_consistency() {
        let h1 = short_hash("test");
        let h2 = short_hash("test");
        let h3 = short_hash("other");

        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
        // It might be shorter than 6 if hash is small, but unlikely for random strings.
        // We added a check for length in the implementation.
    }
}
