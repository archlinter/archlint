use crate::detectors::{detector, ArchSmell, Detector, DetectorCategory};
use crate::engine::AnalysisContext;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(
    id = "primitive_obsession",
    name = "Primitive Obsession Detector",
    description = "Detects functions with too many primitive parameters",
    category = DetectorCategory::FileLocal,
    default_enabled = false
)]
pub struct PrimitiveObsessionDetector;

impl PrimitiveObsessionDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for PrimitiveObsessionDetector {
    crate::impl_detector_report!(
        name: "PrimitiveObsession",
        explain: smell => (
            problem: {
                if let crate::detectors::SmellType::PrimitiveObsession { primitives, function } = &smell.smell_type {
                    format!("Function `{}` has too many primitive parameters ({})", function, primitives)
                } else {
                    "Too many primitive parameters".into()
                }
            },
            reason: "Using too many primitive types instead of domain-specific objects can lead to logic being scattered and lack of type safety.",
            risks: [
                "Weak type safety",
                "Logic duplication across the codebase"
            ],
            recommendations: [
                "Introduce Value Objects or Domain Types to wrap related primitives"
            ]
        ),
        table: {
            title: "Primitive Obsession",
            columns: ["Location", "Function", "Primitives", "pts"],
            row: PrimitiveObsession { function, primitives } (smell, location, pts) => [location, function, primitives, pts]
        }
    );

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, functions) in ctx.function_complexity.as_ref() {
            let rule = match ctx.get_rule_for_file("primitive_obsession", path) {
                Some(r) => r,
                None => continue,
            };

            let max_primitives: usize = rule.get_option("max_primitives").unwrap_or(3);

            for func in functions {
                if func.primitive_params > max_primitives {
                    let mut smell = ArchSmell::new_primitive_obsession(
                        path.clone(),
                        func.name.to_string(),
                        func.primitive_params,
                    );
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
