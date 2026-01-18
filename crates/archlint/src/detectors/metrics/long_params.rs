use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

#[detector(SmellType::LongParameterList)]
pub struct LongParameterListDetector;

impl LongParameterListDetector {
    #[must_use]
    pub const fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for LongParameterListDetector {
    crate::impl_detector_report!(
        explain: smell => (
            problem: if let crate::detectors::SmellType::LongParameterList { count, name } = &smell.smell_type {
                format!("Function `{name}` has too many parameters ({count})")
            } else {
                "Too many parameters".into()
            },
            reason: "Functions with too many parameters are difficult to use and maintain. They often indicate that the function has too many responsibilities.",
            risks: ["Violation of SRP", "Difficult to test and mock"],
            recommendations: ["Group related parameters into an object or split the function"]
        ),
        table: {
            title: "Long Parameter Lists",
            columns: ["Location", "Function", "Params", "pts"],
            row: LongParameterList { name, count } (smell, location, pts) => [location, name, count, pts]
        }
    );

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
