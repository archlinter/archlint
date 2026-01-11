use crate::detectors::{
    detector, ArchSmell, Detector, DetectorCategory, Explanation, SmellWithExplanation,
};
use crate::engine::AnalysisContext;

pub fn init() {}

#[detector(
    id = "side_effect_import",
    name = "Side-Effect Import Detector",
    description = "Detects imports that execute code on load without binding any symbols",
    category = DetectorCategory::ImportBased
)]
pub struct SideEffectImportDetector;

impl SideEffectImportDetector {
    pub fn new_default(_config: &crate::config::Config) -> Self {
        Self
    }

    fn is_ignored_source(&self, source: &str) -> bool {
        source.ends_with(".css")
            || source.ends_with(".scss")
            || source.ends_with(".sass")
            || source.ends_with(".less")
            || source == "reflect-metadata"
            || source.contains("polyfill")
            || source.contains("setup")
            || source.contains("instrument")
            || source.contains("register") // common pattern for side-effects that are sometimes intentional
    }
}

impl Detector for SideEffectImportDetector {
    fn name(&self) -> &'static str {
        "SideEffectImport"
    }

    fn explain(&self, _smell: &ArchSmell) -> Explanation {
        Explanation {
            problem: "Side-Effect Import".to_string(),
            reason: "Import that executes code on load without binding any symbols. This can make the code's behavior unpredictable and difficult to test.".to_string(),
            risks: vec!["Global state pollution".to_string(), "Unpredictable execution order".to_string()],
            recommendations: vec!["Explicitly initialize the module or use named imports if possible".to_string()],
        }
    }

    fn render_markdown(
        &self,
        smells: &[&SmellWithExplanation],
        severity_config: &crate::config::SeverityConfig,
        _graph: Option<&crate::graph::DependencyGraph>,
    ) -> String {
        use crate::report::format_location;
        crate::define_report_section!("Side-Effect Imports", smells, {
            crate::render_table!(
                vec!["Location", "Source", "pts"],
                smells,
                |&(smell, _): &&SmellWithExplanation| {
                    // Side-effect import info is in locations
                    let loc = smell.locations.first().unwrap();
                    let pts = smell.score(severity_config);
                    vec![
                        format!("`{}`", format_location(&loc.file, loc.line, None)),
                        format!(
                            "`{}`",
                            loc.description.replace("Side-effect import of ", "")
                        ),
                        format!("{} pts", pts),
                    ]
                }
            )
        })
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();

        for (path, symbols) in ctx.file_symbols.as_ref() {
            let rule = match ctx.get_rule_for_file("side_effect_import", path) {
                Some(r) => r,
                None => continue,
            };

            for import in &symbols.imports {
                if import.name == "*"
                    && import.alias.is_none()
                    && !import.is_reexport
                    && !import.is_dynamic
                {
                    // Check if it's a CSS file or similar (should be ignored)
                    if self.is_ignored_source(&import.source) {
                        continue;
                    }

                    let mut smell = ArchSmell::new_side_effect_import(
                        path.clone(),
                        import.source.to_string(),
                        import.line,
                    );
                    smell.severity = rule.severity;
                    smells.push(smell);
                }
            }
        }

        smells
    }
}
