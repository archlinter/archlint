use crate::detectors::{detector, ArchSmell, Detector};
use crate::engine::AnalysisContext;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub fn init() {}

#[detector(SmellType::DeepNesting)]
pub struct DeepNestingDetector;

impl DeepNestingDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }
}

impl Detector for DeepNestingDetector {
    crate::impl_detector_report!(
        explain: smell => (
            problem: {
                if let crate::detectors::SmellType::DeepNesting { name, depth, .. } = &smell.smell_type {
                    format!("Function `{}` is too deeply nested (depth: {})", name, depth)
                } else {
                    "Too deeply nested".into()
                }
            },
            reason: "Deeply nested code structures (if, for, while, etc.) make the logic hard to follow and increase the risk of bugs.",
            risks: [
                "Increased cognitive load",
                "Difficult to test all branches"
            ],
            recommendations: [
                "Refactor using guard clauses, early returns, or extract nested blocks into separate functions"
            ]
        ),
        table: {
            title: "Deep Nesting",
            columns: ["Location", "Function", "Depth", "pts"],
            row: DeepNesting { name, depth } (smell, location, pts) => [location, name, depth, pts]
        }
    );

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
