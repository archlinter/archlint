use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;

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
    fn name(&self) -> &'static str {
        "PrimitiveObsession"
    }

    fn explain(&self, smell: &ArchSmell) -> Explanation {
        let (function, count) = match &smell.smell_type {
            SmellType::PrimitiveObsession {
                primitives,
                function,
            } => (function.clone(), *primitives),
            _ => ("unknown".to_string(), 0),
        };
        Explanation {
            problem: format!("Function `{}` has too many primitive parameters ({})", function, count),
            reason: "Using too many primitive types instead of domain-specific objects can lead to logic being scattered and lack of type safety.".to_string(),
            risks: vec!["Weak type safety".to_string(), "Logic duplication across the codebase".to_string()],
            recommendations: vec!["Introduce Value Objects or Domain Types to wrap related primitives".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::report::format_location;
        crate::define_report_section!("Primitive Obsession", smells, {
            crate::render_table!(
                vec!["Location", "Function", "Primitives", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let (function, count): (String, usize) = match &smell.smell_type {
                        SmellType::PrimitiveObsession {
                            primitives,
                            function,
                        } => (function.clone(), *primitives),
                        _ => ("unknown".to_string(), 0),
                    };
                    let file_path = smell.files.first().unwrap();
                    let location = format_location(file_path, 0, None); // Should have line info
                    let pts = smell.score(severity_config);
                    vec![
                        format!("`{}`", location),
                        format!("`{}`", function),
                        count.to_string(),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

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
