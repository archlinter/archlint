use super::common::{append_explanation, SmellWithExplanation};
use crate::config::SeverityConfig;
use crate::explain::ExplainEngine;

pub fn generate(shotgun_surgery: &[&SmellWithExplanation], severity_config: &SeverityConfig) -> String {
    if shotgun_surgery.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(&format!(
        "## Shotgun Surgery ({} files)\n\n",
        shotgun_surgery.len()
    ));

    if let Some((_, explanation)) = shotgun_surgery.first() {
        append_explanation(&mut output, explanation);
    }

    output.push_str("| File | Avg Co-changes | Related Files (Top 5) | pts |\n");
    output.push_str("|------|----------------|-----------------------|-----|\n");

    for (smell, _) in shotgun_surgery {
        if let Some(file_path) = smell.files.first() {
            let formatted_path = ExplainEngine::format_file_path(file_path);
            let avg = smell.avg_co_changes().unwrap_or(0.0);
            let pts = smell.score(severity_config);

            let related = smell
                .locations
                .iter()
                .filter(|l| l.file != *file_path)
                .map(|l| {
                    let path = ExplainEngine::format_file_path(&l.file);
                    if l.description.is_empty() {
                        format!("`{}`", path)
                    } else {
                        format!("`{}` ({})", path, l.description)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");

            output.push_str(&format!(
                "| `{}` | {:.1} | {} | {} pts |\n",
                formatted_path, avg, related, pts
            ));
        }
    }
    output.push('\n');

    output
}
