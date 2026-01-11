use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellType, SmellWithExplanation,
};
use crate::engine::AnalysisContext;

pub fn init() {}

#[detector(
    id = "deep_nesting",
    name = "Deep Nesting Detector",
    description = "Detects functions with excessive nesting depth",
    category = DetectorCategory::FileLocal
)]
pub struct DeepNestingDetector;

impl DeepNestingDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for DeepNestingDetector {
    fn name(&self) -> &'static str {
        "DeepNesting"
    }

    fn explain(&self, smell: &ArchSmell) -> Explanation {
        let (function, depth) = match &smell.smell_type {
            SmellType::DeepNesting {
                function, depth, ..
            } => (function.clone(), *depth),
            _ => ("unknown".to_string(), 0),
        };
        Explanation {
            problem: format!("Function `{}` is too deeply nested (depth: {})", function, depth),
            reason: "Deeply nested code structures (if, for, while, etc.) make the logic hard to follow and increase the risk of bugs.".to_string(),
            risks: vec!["Increased cognitive load".to_string(), "Difficult to test all branches".to_string()],
            recommendations: vec!["Refactor using guard clauses, early returns, or extract nested blocks into separate functions".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::report::format_location_detail;
        crate::define_report_section!("Deep Nesting", smells, {
            crate::render_table!(
                vec!["Location", "Function", "Depth", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    let (function, depth): (String, usize) = match &smell.smell_type {
                        SmellType::DeepNesting {
                            function, depth, ..
                        } => (function.clone(), *depth),
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
                        depth.to_string(),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, functions) in ctx.function_complexity.as_ref() {
            let rule = match ctx.get_rule_for_file("deep_nesting", path) {
                Some(r) => r,
                None => continue,
            };

            let max_depth: usize = rule.get_option("max_depth").unwrap_or(4);

            for func in functions {
                if func.max_depth > max_depth {
                    let mut smell = ArchSmell::new_deep_nesting(
                        path.clone(),
                        func.name.to_string(),
                        func.max_depth,
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
