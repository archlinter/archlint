use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;

pub fn init() {}

#[detector(
    id = "long_params",
    name = "Long Parameter List Detector",
    description = "Detects functions with too many parameters",
    category = DetectorCategory::FileLocal
)]
pub struct LongParameterListDetector;

impl LongParameterListDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for LongParameterListDetector {
    fn name(&self) -> &'static str {
        "LongParameterList"
    }

    fn explain(&self, smell: &ArchSmell) -> Explanation {
        let (function, count) = match &smell.smell_type {
            SmellType::LongParameterList { count, function } => (function.clone(), *count),
            _ => ("unknown".to_string(), 0),
        };
        Explanation {
            problem: format!("Function `{}` has too many parameters ({})", function, count),
            reason: "Functions with too many parameters are difficult to use and maintain. They often indicate that the function has too many responsibilities.".to_string(),
            risks: vec!["Violation of SRP".to_string(), "Difficult to test and mock".to_string()],
            recommendations: vec!["Group related parameters into an object or split the function".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::report::format_location_detail;
        crate::define_report_section!("Long Parameter Lists", smells, {
            crate::render_table!(
                vec!["Location", "Function", "Params", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let (function, count): (String, usize) = match &smell.smell_type {
                        SmellType::LongParameterList { count, function } => {
                            (function.clone(), *count)
                        }
                        _ => ("unknown".to_string(), 0),
                    };
                    let location = smell
                        .locations
                        .first()
                        .map(format_location_detail)
                        .unwrap_or_default();
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
            let rule = match ctx.get_rule_for_file("long_params", path) {
                Some(r) => r,
                None => continue,
            };

            let ignore_constructors: bool = rule.get_option("ignore_constructors").unwrap_or(true);
            let max_params: usize = rule.get_option("max_params").unwrap_or(5);

            for func in functions {
                if ignore_constructors && func.is_constructor {
                    continue;
                }

                if func.param_count > max_params {
                    let mut smell = ArchSmell::new_long_params(
                        path.clone(),
                        func.name.to_string(),
                        func.param_count,
                        func.line,
                        func.range,
                    );
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
