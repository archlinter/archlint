pub use crate::detectors::Explanation;
use crate::detectors::{ArchSmell, SmellType};
use crate::snapshot::SnapshotSmell;
use std::path::{Path, PathBuf};

pub struct ExplainEngine;

impl ExplainEngine {
    pub fn explain_snapshot_smell(smell: &SnapshotSmell) -> Explanation {
        let smell_type = match smell.smell_type.as_str() {
            "CyclicDependency" | "Cycles" => SmellType::CyclicDependency,
            "CyclicDependencyCluster" => SmellType::CyclicDependencyCluster,
            "GodModule" => SmellType::GodModule,
            "DeadCode" => SmellType::DeadCode,
            "DeadSymbol" | "DeadSymbols" => {
                let name = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::DeadSymbol { name, .. } => {
                            Some(name.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                let kind = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::DeadSymbol { kind, .. } => {
                            Some(kind.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_else(|| "Symbol".to_string());
                SmellType::DeadSymbol { name, kind }
            }
            "HighComplexity" | "Complexity" => {
                let name = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::Complexity { function_name, .. } => {
                            Some(function_name.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                let line = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::Complexity { line, .. } => Some(*line),
                        _ => None,
                    })
                    .unwrap_or_default();
                let complexity = smell
                    .metrics
                    .get("complexity")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as usize)
                    .unwrap_or(0);
                SmellType::HighComplexity {
                    name,
                    line,
                    complexity,
                }
            }
            "LayerViolation" => {
                let (from, to) = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::LayerViolation {
                            from_layer,
                            to_layer,
                            ..
                        } => Some((from_layer.clone(), to_layer.clone())),
                        _ => None,
                    })
                    .unwrap_or_default();
                SmellType::LayerViolation {
                    from_layer: from,
                    to_layer: to,
                }
            }
            "HubModule" | "HubDependency" => SmellType::HubModule,
            "LowCohesion" | "Lcom" => {
                let lcom = smell
                    .metrics
                    .get("lcom")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as usize)
                    .unwrap_or(0);
                SmellType::LowCohesion { lcom }
            }
            "SdpViolation" => SmellType::SdpViolation,
            "LargeFile" => SmellType::LargeFile,
            "UnstableInterface" => SmellType::UnstableInterface,
            "FeatureEnvy" => {
                let most_envied_module = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::FeatureEnvy {
                            most_envied_module, ..
                        } => Some(PathBuf::from(most_envied_module)),
                        _ => None,
                    })
                    .unwrap_or_default();
                SmellType::FeatureEnvy { most_envied_module }
            }
            "ShotgunSurgery" => SmellType::ShotgunSurgery,
            "TestLeakage" => {
                let test_file = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::TestLeakage { test_file, .. } => {
                            Some(PathBuf::from(test_file))
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                SmellType::TestLeakage { test_file }
            }
            "BarrelFileAbuse" => SmellType::BarrelFileAbuse,
            "VendorCoupling" => {
                let package = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::VendorCoupling { package, .. } => {
                            Some(package.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                SmellType::VendorCoupling { package }
            }
            "SideEffectImport" => SmellType::SideEffectImport,
            "ScatteredModule" => {
                let components = smell
                    .metrics
                    .get("components")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as usize)
                    .unwrap_or(0);
                SmellType::ScatteredModule { components }
            }
            "HighCoupling" => {
                let cbo = smell
                    .metrics
                    .get("cbo")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as usize)
                    .unwrap_or(0);
                SmellType::HighCoupling { cbo }
            }
            "PackageCycle" => {
                let packages = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::PackageCycle { packages, .. } => {
                            Some(packages.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                SmellType::PackageCycle { packages }
            }
            "SharedMutableState" => {
                let symbol = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::SharedMutableState { symbol, .. } => {
                            Some(symbol.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                SmellType::SharedMutableState { symbol }
            }
            "DeepNesting" => {
                let depth = smell
                    .metrics
                    .get("depth")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as usize)
                    .unwrap_or(0);
                let function = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::Complexity { function_name, .. } => {
                            Some(function_name.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                let line = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::Complexity { line, .. } => Some(*line),
                        _ => None,
                    })
                    .unwrap_or_default();
                SmellType::DeepNesting {
                    function,
                    depth,
                    line,
                }
            }
            "LongParameterList" => {
                let count = smell
                    .metrics
                    .get("count")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as usize)
                    .unwrap_or(0);
                let function = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::LongParameterList { function, .. } => {
                            Some(function.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                SmellType::LongParameterList { count, function }
            }
            "PrimitiveObsession" => {
                let primitives = smell
                    .metrics
                    .get("primitives")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as usize)
                    .unwrap_or(0);
                let function = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::PrimitiveObsession { function, .. } => {
                            Some(function.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                SmellType::PrimitiveObsession {
                    primitives,
                    function,
                }
            }
            "OrphanType" | "OrphanTypes" => {
                let name = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::OrphanType { name, .. } => {
                            Some(name.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                SmellType::OrphanType { name }
            }
            "CircularTypeDependency" | "CircularTypeDependencies" => {
                SmellType::CircularTypeDependency
            }
            "AbstractnessViolation" => SmellType::AbstractnessViolation,
            "ScatteredConfiguration" => {
                let env_var = smell
                    .details
                    .as_ref()
                    .and_then(|d| match d {
                        crate::snapshot::SmellDetails::ScatteredConfiguration {
                            env_var, ..
                        } => Some(env_var.clone()),
                        _ => None,
                    })
                    .unwrap_or_default();
                let files_count = smell
                    .metrics
                    .get("filesCount")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as usize)
                    .unwrap_or(0);
                SmellType::ScatteredConfiguration {
                    env_var,
                    files_count,
                }
            }
            "CodeClone" => {
                let clone_hash = smell
                    .metrics
                    .get("cloneHash")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let token_count = smell
                    .metrics
                    .get("tokenCount")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as usize)
                    .unwrap_or(0);
                SmellType::CodeClone {
                    clone_hash,
                    token_count,
                }
            }
            _ => SmellType::GodModule, // Fallback
        };

        // Create a temporary ArchSmell to reuse the explanation logic
        let mut metrics = Vec::new();
        for (name, value) in &smell.metrics {
            let val = value.as_f64();
            let metric = match name.as_str() {
                "fanIn" => crate::detectors::SmellMetric::FanIn(val as usize),
                "fanOut" => crate::detectors::SmellMetric::FanOut(val as usize),
                "complexity" => crate::detectors::SmellMetric::Complexity(val as usize),
                "lines" => crate::detectors::SmellMetric::Lines(val as usize),
                "lcom" => crate::detectors::SmellMetric::Lcom(val as usize),
                "cbo" => crate::detectors::SmellMetric::Cbo(val as usize),
                "depth" => crate::detectors::SmellMetric::Depth(val as usize),
                "distance" => crate::detectors::SmellMetric::Distance(val),
                _ => continue,
            };
            metrics.push(metric);
        }

        let arch_smell = crate::detectors::ArchSmell {
            smell_type,
            severity: smell
                .severity
                .parse()
                .unwrap_or(crate::detectors::Severity::Medium),
            files: smell.files.iter().map(PathBuf::from).collect(),
            metrics,
            locations: vec![],
            cluster: None,
        };

        Self::explain(&arch_smell)
    }

    pub fn explain(smell: &crate::detectors::ArchSmell) -> Explanation {
        // Fallback for not yet migrated explanations
        match &smell.smell_type {
            SmellType::TestLeakage { .. } => Self::explain_test_leakage(smell),
            SmellType::LayerViolation { .. } => Self::explain_layer_violation(smell),
            SmellType::SdpViolation => Self::explain_sdp_violation(smell),
            _ => {
                // Try dynamic explanation if available
                let registry = crate::detectors::DetectorRegistry::new();
                let detector_id = smell.smell_type.category().to_id();
                if let Some(detector) =
                    registry.create_detector(detector_id, &crate::config::Config::default())
                {
                    detector.explain(smell)
                } else {
                    Self::simple_explanation("Unknown Smell", "No detailed explanation available")
                }
            }
        }
    }

    fn simple_explanation(problem: &str, reason: &str) -> Explanation {
        Explanation {
            problem: problem.to_string(),
            reason: reason.to_string(),
            risks: vec!["Increased maintenance cost".to_string()],
            recommendations: vec!["Refactor code to improve architecture".to_string()],
        }
    }

    fn explain_test_leakage(_smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Test-to-Production Leakage".to_string(),
            reason: "A production module imports a test file, mock, or test utility. This can lead to test code being included in production bundles.".to_string(),
            risks: vec![
                "Increased bundle size".to_string(),
                "Potential security risks if mocks expose internal data".to_string(),
                "Code fragility: production depends on test helpers".to_string(),
            ],
            recommendations: vec![
                "Move shared utilities to a separate non-test module".to_string(),
                "Check if the import was accidental and remove it".to_string(),
            ],
        }
    }

    fn explain_layer_violation(_smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Layer Architecture Violation".to_string(),
            reason: "A module in one layer imports a module from a layer it shouldn't know about (e.g., domain depending on infrastructure).".to_string(),
            risks: vec![
                "Circular dependencies between layers".to_string(),
                "Difficult to test domain logic in isolation".to_string(),
                "Leaking implementation details into business logic".to_string(),
            ],
            recommendations: vec![
                "Use Dependency Inversion Principle (DIP)".to_string(),
                "Introduce interfaces in the stable layer".to_string(),
                "Move the code to the appropriate layer".to_string(),
            ],
        }
    }

    fn explain_sdp_violation(_smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Stable Dependency Principle (SDP) Violation".to_string(),
            reason: "A stable module (rarely changing, many dependents) depends on an unstable module (frequently changing).".to_string(),
            risks: vec![
                "Stable modules become unstable due to their dependencies".to_string(),
                "Fragile architecture: changes in unstable parts break the core".to_string(),
            ],
            recommendations: vec![
                "Identify stable interfaces and depend on them".to_string(),
                "Refactor the unstable dependency to be more stable".to_string(),
                "Invert the dependency using abstractions".to_string(),
            ],
        }
    }

    pub fn format_file_path(path: &Path) -> String {
        path.to_string_lossy().to_string()
    }
}
