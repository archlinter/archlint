use crate::detectors::{ArchSmell, SmellType};
use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Explanation {
    pub problem: String,
    pub reason: String,
    pub risks: Vec<String>,
    pub recommendations: Vec<String>,
}

pub struct ExplainEngine;

impl ExplainEngine {
    pub fn explain(smell: &ArchSmell) -> Explanation {
        match &smell.smell_type {
            SmellType::CyclicDependency => Self::explain_cycle(smell),
            SmellType::CyclicDependencyCluster => Self::explain_cycle(smell),
            SmellType::GodModule => Self::explain_god_module(smell),
            SmellType::DeadCode => Self::explain_dead_code(smell),
            SmellType::DeadSymbol { .. } => Self::explain_dead_symbol(smell),
            SmellType::HighComplexity { .. } => Self::explain_high_complexity(smell),
            SmellType::LargeFile => Self::explain_large_file(smell),
            SmellType::UnstableInterface => Self::explain_unstable_interface(smell),
            SmellType::FeatureEnvy { .. } => Self::explain_feature_envy(smell),
            SmellType::ShotgunSurgery => Self::explain_shotgun_surgery(smell),
            SmellType::HubDependency { .. } => Self::explain_hub_dependency(smell),
            SmellType::TestLeakage { .. } => Self::explain_test_leakage(smell),
            SmellType::LayerViolation { .. } => Self::explain_layer_violation(smell),
            SmellType::SdpViolation => Self::explain_sdp_violation(smell),
            SmellType::BarrelFileAbuse => {
                Self::simple_explanation("Barrel File Abuse", "Excessive re-exports in index file")
            }
            SmellType::VendorCoupling { .. } => Self::simple_explanation(
                "Vendor Coupling",
                "Direct usage of third-party package in many files",
            ),
            SmellType::SideEffectImport => {
                Self::simple_explanation("Side-Effect Import", "Import that executes code on load")
            }
            SmellType::HubModule => {
                Self::simple_explanation("Hub Module", "Module acting as a pass-through hub")
            }
            SmellType::LowCohesion { .. } => {
                Self::simple_explanation("Low Cohesion", "Class methods are not cohesive")
            }
            SmellType::ScatteredModule { .. } => {
                Self::simple_explanation("Scattered Module", "Module exports are not related")
            }
            SmellType::HighCoupling { .. } => {
                Self::simple_explanation("High Coupling", "Module has too many dependencies")
            }
            SmellType::PackageCycle { .. } => {
                Self::simple_explanation("Package Cycle", "Circular dependency between packages")
            }
            SmellType::SharedMutableState { .. } => Self::simple_explanation(
                "Shared Mutable State",
                "Exported state that can be mutated",
            ),
            SmellType::DeepNesting { .. } => {
                Self::simple_explanation("Deep Nesting", "Code is too deeply nested")
            }
            SmellType::LongParameterList { .. } => {
                Self::simple_explanation("Long Parameter List", "Function has too many parameters")
            }
            SmellType::PrimitiveObsession { .. } => {
                Self::simple_explanation("Primitive Obsession", "Too many primitive parameters")
            }
            SmellType::OrphanType { .. } => {
                Self::simple_explanation("Orphan Type", "Type is defined but never used")
            }
            SmellType::CircularTypeDependency => Self::simple_explanation(
                "Circular Type Dependency",
                "Circular dependency between types",
            ),
            SmellType::AbstractnessViolation => Self::simple_explanation(
                "Abstractness Violation",
                "Module distance from main sequence is too high",
            ),
            SmellType::ScatteredConfiguration { .. } => Self::simple_explanation(
                "Scattered Configuration",
                "Configuration is spread across many files",
            ),
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
            reason: "A stable module (rarely changing, many dependants) depends on an unstable module (frequently changing).".to_string(),
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

    fn explain_cycle(smell: &ArchSmell) -> Explanation {
        let cycle_length = smell.cycle_length().unwrap_or(0);

        Explanation {
            problem: format!(
                "Circular dependency detected between {} files",
                cycle_length
            ),
            reason: "Files form a dependency cycle where A depends on B, B depends on C, and C depends back on A (or similar pattern). This creates tight coupling between modules.".to_string(),
            risks: vec![
                "Difficult to reason about initialization order".to_string(),
                "Changes in one module can cascade unpredictably to others".to_string(),
                "Testing becomes difficult due to interdependencies".to_string(),
                "Refactoring is risky and error-prone".to_string(),
                "May cause compilation or runtime initialization issues".to_string(),
            ],
            recommendations: vec![
                "Extract shared logic into a separate, independent module".to_string(),
                "Use dependency injection to break direct dependencies".to_string(),
                "Introduce interfaces/abstractions to invert dependencies".to_string(),
                "Apply the Dependency Inversion Principle (DIP)".to_string(),
                "Consider using event-driven architecture for loose coupling".to_string(),
            ],
        }
    }

    fn explain_god_module(smell: &ArchSmell) -> Explanation {
        let fan_in = smell.fan_in().unwrap_or(0);
        let fan_out = smell.fan_out().unwrap_or(0);
        let churn = smell.churn().unwrap_or(0);

        Explanation {
            problem: format!(
                "Module has excessive responsibilities (fan-in: {}, fan-out: {}, churn: {})",
                fan_in, fan_out, churn
            ),
            reason: "This module is imported by many files (high fan-in), imports many files (high fan-out), and changes frequently (high churn). This indicates it's doing too much and violates the Single Responsibility Principle.".to_string(),
            risks: vec![
                "Single point of failure in the system".to_string(),
                "Difficult to understand and maintain".to_string(),
                "High risk of merge conflicts".to_string(),
                "Changes affect many parts of the system".to_string(),
                "Hard to test in isolation".to_string(),
                "Performance bottleneck potential".to_string(),
            ],
            recommendations: vec![
                "Split the module by domain or functionality".to_string(),
                "Apply Single Responsibility Principle (SRP)".to_string(),
                "Extract utility functions into focused, single-purpose modules".to_string(),
                "Use facade pattern if the module serves as an integration point".to_string(),
                "Identify cohesive groups of functions and separate them".to_string(),
                "Consider creating a layered architecture to reduce coupling".to_string(),
            ],
        }
    }

    fn explain_dead_code(_smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Unused file detected (no incoming dependencies)".to_string(),
            reason: "This file is not imported by any other module in the codebase. It may be leftover code from refactoring, experimental code that was never integrated, or a genuinely unused module.".to_string(),
            risks: vec![
                "Increases codebase size and maintenance burden".to_string(),
                "Causes confusion about what code is actually in use".to_string(),
                "May contain outdated patterns or security vulnerabilities".to_string(),
                "Wastes developer time when searching or refactoring".to_string(),
                "Can lead to accidental usage of outdated code".to_string(),
            ],
            recommendations: vec![
                "Verify the file is truly unused (check dynamic imports, tests, configs)".to_string(),
                "Remove the file if confirmed as dead code".to_string(),
                "If keeping for reference, move to an archive or documentation".to_string(),
                "Add the file to entry_points config if it's an intentional entry point".to_string(),
                "Review recent refactorings to understand why it became unused".to_string(),
            ],
        }
    }

    fn explain_dead_symbol(smell: &ArchSmell) -> Explanation {
        let (name, kind) = match &smell.smell_type {
            SmellType::DeadSymbol { name, kind } => (name.as_str(), kind.as_str()),
            _ => ("unknown", "Symbol"),
        };

        Explanation {
            problem: format!("Unused {} detected", kind),
            reason: format!(
                "The {} '{}' is defined but not imported by any other file or used locally.",
                kind, name
            ),
            risks: vec![
                "Increases cognitive load when reading the file".to_string(),
                "Dead code can hide bugs and complicate refactoring".to_string(),
                "May lead to confusion about the intended API of the module".to_string(),
            ],
            recommendations: vec![
                format!("Remove the unused {} if it is truly no longer needed", kind),
                "Check if it should be an internal helper or if it was meant to be exported and used".to_string(),
                "Use a tool like 'ts-unused-exports' or similar for detailed symbol tracking if this is common".to_string(),
            ],
        }
    }

    fn explain_high_complexity(smell: &ArchSmell) -> Explanation {
        let (name, _line) = match &smell.smell_type {
            SmellType::HighComplexity { name, line } => (name.clone(), *line),
            _ => ("unknown".to_string(), 0),
        };

        let complexity = smell.complexity().unwrap_or(0);

        Explanation {
            problem: format!(
                "Function `{}` has high cyclomatic complexity ({})",
                name, complexity
            ),
            reason: "High cyclomatic complexity indicates that the function has too many decision points (if, for, while, etc.), making it difficult to understand, test, and maintain.".to_string(),
            risks: vec![
                "Higher probability of bugs due to complex logic".to_string(),
                "Difficult to achieve high test coverage".to_string(),
                "Hard for other developers to read and understand".to_string(),
                "Refactoring becomes dangerous and difficult".to_string(),
            ],
            recommendations: vec![
                "Extract complex nested logic into smaller, focused helper functions".to_string(),
                "Use early returns to reduce nesting depth".to_string(),
                "Simplify logical expressions".to_string(),
                "Consider using design patterns like Strategy or Command for complex branching".to_string(),
                "Break down large switch statements".to_string(),
            ],
        }
    }

    fn explain_large_file(smell: &ArchSmell) -> Explanation {
        let lines = smell.lines().unwrap_or(0);

        Explanation {
            problem: format!("File has {} lines, exceeding the recommended limit", lines),
            reason: "Large files are difficult to understand, navigate, and maintain. They often indicate a violation of the Single Responsibility Principle.".to_string(),
            risks: vec![
                "Difficult to understand and navigate".to_string(),
                "Higher chance of merge conflicts".to_string(),
                "Slower code reviews and longer review times".to_string(),
                "Often indicates mixed responsibilities".to_string(),
                "IDE performance may be impacted".to_string(),
            ],
            recommendations: vec![
                "Split the file by domain or functionality".to_string(),
                "Extract utility functions into separate modules".to_string(),
                "Identify cohesive groups of code and separate them".to_string(),
                "Consider using barrel files to re-export split modules".to_string(),
                "Apply Single Responsibility Principle (SRP)".to_string(),
            ],
        }
    }

    fn explain_unstable_interface(smell: &ArchSmell) -> Explanation {
        let churn = smell.churn().unwrap_or(0);
        let dependants = smell.fan_in().unwrap_or(0);
        let score = smell.instability_score().unwrap_or(0);

        Explanation {
            problem: format!(
                "Unstable interface detected (churn: {}, dependants: {}, score: {})",
                churn, dependants, score
            ),
            reason: "This module changes frequently and is used by many other modules. This means changes here have a high probability of breaking other parts of the system.".to_string(),
            risks: vec![
                "Frequent regressions in dependant modules".to_string(),
                "High cost of maintenance due to cascading changes".to_string(),
                "Difficult to stabilize the overall architecture".to_string(),
            ],
            recommendations: vec![
                "Identify why the module changes so frequently and extract stable parts".to_string(),
                "Introduce a stable interface (API) and keep implementation details hidden".to_string(),
                "Reduce the number of dependants by using events or a message bus".to_string(),
            ],
        }
    }

    fn explain_feature_envy(smell: &ArchSmell) -> Explanation {
        let ratio = smell.envy_ratio().unwrap_or(0.0);
        let external_refs = smell.fan_in().unwrap_or(0);
        let internal_refs = smell.fan_out().unwrap_or(0);

        let envied_module = match &smell.smell_type {
            SmellType::FeatureEnvy { most_envied_module } => {
                most_envied_module.to_string_lossy().to_string()
            }
            _ => "unknown".to_string(),
        };

        Explanation {
            problem: format!(
                "Feature Envy: Module uses external symbols (ratio: {:.1}x)",
                ratio
            ),
            reason: format!(
                "This module uses {} symbols from `{}` but only {} internal symbols. It seems more interested in the details of another module than its own functionality.",
                external_refs, envied_module, internal_refs
            ),
            risks: vec![
                "Violation of encapsulation and data hiding".to_string(),
                "Tight coupling between the two modules".to_string(),
                "Increased difficulty in testing and refactoring".to_string(),
            ],
            recommendations: vec![
                "Move the code that uses the external symbols into the envied module".to_string(),
                "Extract a new module that contains the logic and data together".to_string(),
                "Pass only necessary data as arguments instead of accessing many properties".to_string(),
            ],
        }
    }

    fn explain_shotgun_surgery(smell: &ArchSmell) -> Explanation {
        let avg_co_changes = smell.avg_co_changes().unwrap_or(0.0);
        let dependant_count = smell.dependant_count().unwrap_or(0);
        let primary_file = smell
            .files
            .first()
            .and_then(|f| f.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("this file");

        Explanation {
            problem: format!(
                "Shotgun Surgery: {} is highly coupled with {} other files (avg: {:.1} files per change)",
                primary_file, dependant_count, avg_co_changes
            ),
            reason: format!(
                "When {} is modified, it usually requires simultaneous changes in many other files. This 'shotgun' effect suggests that a single logical responsibility is fragmented across the codebase.",
                primary_file
            ),
            risks: vec![
                "High maintenance effort: one logical change requires many physical edits".to_string(),
                "Partial updates: forgetting to update one of the related files leads to inconsistent state".to_string(),
                "Knowledge fragmentation: the full logic is not visible in a single place".to_string(),
            ],
            recommendations: vec![
                format!("Consolidate the related logic from coupled files into {} or a new shared module", primary_file),
                "Apply the 'Move Method' or 'Move Field' refactoring to bring related parts together".to_string(),
                "Consider if the coupling is due to shared data structures that could be abstracted".to_string(),
            ],
        }
    }

    fn explain_hub_dependency(smell: &ArchSmell) -> Explanation {
        let count = smell.dependant_count().unwrap_or(0);

        let package = match &smell.smell_type {
            SmellType::HubDependency { package } => package.clone(),
            _ => "unknown".to_string(),
        };

        Explanation {
            problem: format!(
                "Hub Dependency: Too many files ({}) depend on package `{}`",
                count, package
            ),
            reason: format!(
                "The package `{}` is used by {} different files in the project. This makes it a critical dependency that is hard to replace or update.",
                package, count
            ),
            risks: vec![
                "Difficulty in upgrading the package due to widespread usage".to_string(),
                "High impact if the package becomes deprecated or has security issues".to_string(),
                "Tightly coupled to a specific external library's API".to_string(),
            ],
            recommendations: vec![
                "Create a wrapper/abstraction around the package to isolate its usage".to_string(),
                "Evaluate if the dependency is truly necessary in all those files".to_string(),
                "Use dependency injection to provide the functionality if possible".to_string(),
            ],
        }
    }

    pub fn format_file_path(path: &Path) -> String {
        path.to_string_lossy().to_string()
    }
}
