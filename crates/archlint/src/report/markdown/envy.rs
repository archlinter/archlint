use super::common::{append_explanation, SmellWithExplanation};
use crate::config::SeverityConfig;
use crate::detectors::SmellType;
use crate::explain::ExplainEngine;

pub fn generate(
    feature_envy: &[&SmellWithExplanation],
    severity_config: &SeverityConfig,
) -> String {
    if feature_envy.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(&format!(
        "## Feature Envy ({} files)\n\n",
        feature_envy.len()
    ));

    if let Some((_, explanation)) = feature_envy.first() {
        append_explanation(&mut output, explanation);
    }

    output.push_str("| File | Envied Module | Ratio | pts |\n");
    output.push_str("|------|---------------|-------|-----|\n");

    for (smell, _) in feature_envy {
        if let Some(file_path) = smell.files.first() {
            let formatted_path = ExplainEngine::format_file_path(file_path);
            let ratio = smell.envy_ratio().unwrap_or(0.0);
            let pts = smell.score(severity_config);

            let envied_module = match &smell.smell_type {
                SmellType::FeatureEnvy { most_envied_module } => {
                    ExplainEngine::format_file_path(most_envied_module)
                }
                _ => "unknown".to_string(),
            };

            output.push_str(&format!(
                "| `{}` | `{}` | {:.1}x | {} pts |\n",
                formatted_path, envied_module, ratio, pts
            ));
        }
    }
    output.push('\n');

    output
}
