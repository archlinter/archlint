use super::common::{append_explanation, SmellWithExplanation};
use crate::config::SeverityConfig;
use crate::explain::ExplainEngine;

pub fn generate(
    unstable_interfaces: &[&SmellWithExplanation],
    severity_config: &SeverityConfig,
) -> String {
    if unstable_interfaces.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(&format!(
        "## Unstable Interfaces ({} files)\n\n",
        unstable_interfaces.len()
    ));

    if let Some((_, explanation)) = unstable_interfaces.first() {
        append_explanation(&mut output, explanation);
    }

    output.push_str("| File | Churn | Dependants | Score | pts |\n");
    output.push_str("|------|-------|------------|-------|-----|\n");

    for (smell, _) in unstable_interfaces {
        if let Some(file_path) = smell.files.first() {
            let formatted_path = ExplainEngine::format_file_path(file_path);
            let churn = smell.churn().unwrap_or(0);
            let dependants = smell.fan_in().unwrap_or(0);
            let score = smell.instability_score().unwrap_or(0);
            let pts = smell.score(severity_config);

            output.push_str(&format!(
                "| `{}` | {} | {} | {} | {} pts |\n",
                formatted_path, churn, dependants, score, pts
            ));
        }
    }
    output.push('\n');

    output
}
