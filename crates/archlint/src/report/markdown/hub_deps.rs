use super::common::{append_explanation, SmellWithExplanation};
use crate::config::SeverityConfig;
use crate::detectors::SmellType;

pub fn generate(hub_dependencies: &[&SmellWithExplanation], severity_config: &SeverityConfig) -> String {
    if hub_dependencies.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(&format!(
        "## Hub Dependencies ({} packages)\n\n",
        hub_dependencies.len()
    ));

    if let Some((_, explanation)) = hub_dependencies.first() {
        append_explanation(&mut output, explanation);
    }

    output.push_str("| Package | Dependants | pts |\n");
    output.push_str("|---------|------------|-----|\n");

    for (smell, _) in hub_dependencies {
        if let SmellType::HubDependency { package } = &smell.smell_type {
            let count = smell.dependant_count().unwrap_or(0);
            let pts = smell.score(severity_config);

            output.push_str(&format!(
                "| `{}` | {} files | {} pts |\n",
                package, count, pts
            ));
        }
    }
    output.push('\n');

    output
}
