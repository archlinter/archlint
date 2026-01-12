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

        SmellType::DeadSymbol { name, .. } => with_line_hash_fallback(smell, |line| {
            id_for_symbol_smell("dead", &smell.files[0], name, line, project_root)
        }),

        SmellType::HighComplexity { name, .. } => with_line_hash_fallback(smell, |line| {
            id_for_symbol_smell("cmplx", &smell.files[0], name, line, project_root)
        }),

        SmellType::HubModule => id_for_file_smell(&smell.files[0], "hub", project_root),

        SmellType::LowCohesion { class_name, .. } => with_line_hash_fallback(smell, |line| {
            id_for_symbol_smell("lcom", &smell.files[0], class_name, line, project_root)
        }),

        SmellType::HubDependency { package } => {
            format!("hub_dep:{}", package)
        }

        SmellType::VendorCoupling { package } => {
            format!("vendor:{}", package)
        }

        SmellType::SideEffectImport => with_line_hash_fallback(smell, |line| {
            let file = &smell.files[0];
            let relative = relative_path(file, project_root);
            format!("sideeffect:{}:{}", relative, line)
        }),

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

        SmellType::SharedMutableState { symbol } => with_line_hash_fallback(smell, |line| {
            id_for_symbol_smell("shared", &smell.files[0], symbol, line, project_root)
        }),

        SmellType::DeepNesting { name, .. } => with_line_hash_fallback(smell, |line| {
            id_for_symbol_smell("nest", &smell.files[0], name, line, project_root)
        }),

        SmellType::LongParameterList { name, .. } => with_line_hash_fallback(smell, |line| {
            id_for_symbol_smell("params", &smell.files[0], name, line, project_root)
        }),

        SmellType::PrimitiveObsession { name, .. } => with_line_hash_fallback(smell, |line| {
            id_for_symbol_smell("prim", &smell.files[0], name, line, project_root)
        }),

        SmellType::OrphanType { name } => with_line_hash_fallback(smell, |line| {
            id_for_symbol_smell("orphan", &smell.files[0], name, line, project_root)
        }),

        SmellType::ScatteredConfiguration { env_var, .. } => {
            format!("config:{}", env_var)
        }

        SmellType::CodeClone { clone_hash, .. } => {
            format!("clone:{}", clone_hash)
        }

        _ => {
            // Fallback: generic ID
            let type_name = format!("{:?}", smell.smell_type);
            let type_short = type_name.split('{').next().unwrap_or(&type_name).trim();
            id_generic(type_short, &smell.files, "", project_root)
        }
    }
}

/// Helper to generate ID with line number and hash fallback when line is 0.
fn with_line_hash_fallback<F>(smell: &ArchSmell, f: F) -> String
where
    F: FnOnce(usize) -> String,
{
    let location = smell.locations.first();
    let line = location.map(|l| l.line).unwrap_or(0);
    let mut id = f(line);
    if line == 0 {
        // Fallback: add hash of description (or full smell if no location) to avoid collisions
        let hash_input = match location {
            Some(l) => l.description.clone(),
            None => format!("{:?}", smell),
        };
        id = format!("{}:{}", id, short_hash(&hash_input));
    }
    id
}

fn relative_path(path: &Path, project_root: &Path) -> String {
    let rel = path.strip_prefix(project_root).unwrap_or_else(|_| {
        debug!("Failed to strip prefix {:?} from {:?}", project_root, path);
        path
    });
    // Normalise path separators and handle potential colons in paths (Windows)
    // Note: Fuzzy matching in fuzzy.rs uses right-splitting to remain resilient to colons in paths.
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

fn id_for_symbol_smell(
    prefix: &str,
    file: &Path,
    symbol_name: &str,
    line: usize,
    project_root: &Path,
) -> String {
    let relative = relative_path(file, project_root);
    format!("{}:{}:{}:{}", prefix, relative, symbol_name, line)
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

    #[test]
    fn test_id_collision_fallback_when_line_is_zero() {
        use crate::detectors::{LocationDetail, Severity};

        let root = Path::new("/project");
        let file = PathBuf::from("/project/src/service.ts");

        let smell1 = ArchSmell {
            smell_type: SmellType::DeadSymbol {
                name: "unused".to_string(),
                kind: "function".to_string(),
            },
            severity: Severity::Low,
            files: vec![file.clone()],
            metrics: vec![],
            locations: vec![LocationDetail::new(file.clone(), 0, "Desc 1".to_string())],
            cluster: None,
        };

        let smell2 = ArchSmell {
            smell_type: SmellType::DeadSymbol {
                name: "unused".to_string(),
                kind: "function".to_string(),
            },
            severity: Severity::Low,
            files: vec![file.clone()],
            metrics: vec![],
            locations: vec![LocationDetail::new(file.clone(), 0, "Desc 2".to_string())],
            cluster: None,
        };

        let id1 = generate_smell_id(&smell1, root);
        let id2 = generate_smell_id(&smell2, root);

        assert_ne!(
            id1, id2,
            "IDs should differ when descriptions differ even if line is 0"
        );
        assert!(id1.contains(&short_hash("Desc 1")));
        assert!(id2.contains(&short_hash("Desc 2")));
    }

    #[test]
    fn test_id_generation_no_hash_when_line_positive() {
        use crate::detectors::{LocationDetail, Severity};

        let root = Path::new("/project");
        let file = PathBuf::from("/project/src/service.ts");

        let smell = ArchSmell {
            smell_type: SmellType::DeadSymbol {
                name: "unused".to_string(),
                kind: "function".to_string(),
            },
            severity: Severity::Low,
            files: vec![file.clone()],
            metrics: vec![],
            locations: vec![LocationDetail::new(
                file.clone(),
                10,
                "Some desc".to_string(),
            )],
            cluster: None,
        };

        let id = generate_smell_id(&smell, root);
        // ID format: dead:src/service.ts:unused:10
        assert_eq!(id, "dead:src/service.ts:unused:10");
        assert!(!id.contains(&short_hash("Some desc")));
    }

    #[test]
    fn test_id_generation_empty_locations_uses_debug_hash() {
        use crate::detectors::Severity;

        let root = Path::new("/project");
        let file = PathBuf::from("/project/src/service.ts");

        let smell = ArchSmell {
            smell_type: SmellType::DeadSymbol {
                name: "unused".to_string(),
                kind: "function".to_string(),
            },
            severity: Severity::Low,
            files: vec![file.clone()],
            metrics: vec![],
            locations: vec![], // Empty locations
            cluster: None,
        };

        let id = generate_smell_id(&smell, root);
        let expected_hash = short_hash(&format!("{:?}", smell));
        assert!(id.contains(&expected_hash));
    }

    #[test]
    fn test_id_generation_side_effect_import_hash() {
        use crate::detectors::{LocationDetail, Severity};

        let root = Path::new("/project");
        let file = PathBuf::from("/project/src/service.ts");

        let smell = ArchSmell {
            smell_type: SmellType::SideEffectImport,
            severity: Severity::Low,
            files: vec![file.clone()],
            metrics: vec![],
            locations: vec![LocationDetail::new(
                file.clone(),
                0,
                "Side effect".to_string(),
            )],
            cluster: None,
        };

        let id = generate_smell_id(&smell, root);
        assert!(id.starts_with("sideeffect:src/service.ts:0:"));
        assert!(id.contains(&short_hash("Side effect")));
    }

    #[test]
    fn test_id_generation_side_effect_import_no_hash_when_line_positive() {
        use crate::detectors::{LocationDetail, Severity};

        let root = Path::new("/project");
        let file = PathBuf::from("/project/src/service.ts");

        let smell = ArchSmell {
            smell_type: SmellType::SideEffectImport,
            severity: Severity::Low,
            files: vec![file.clone()],
            metrics: vec![],
            locations: vec![LocationDetail::new(
                file.clone(),
                10,
                "Side effect".to_string(),
            )],
            cluster: None,
        };

        let id = generate_smell_id(&smell, root);
        assert_eq!(id, "sideeffect:src/service.ts:10");
        assert!(!id.contains(&short_hash("Side effect")));
    }
}
