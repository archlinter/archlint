use super::common::SmellWithExplanation;
use crate::config::SeverityConfig;
use crate::detectors::SmellType;
use crate::report::format_location_detail;

pub fn generate(
    high_complexity: &[&SmellWithExplanation],
    severity_config: &SeverityConfig,
) -> String {
    if high_complexity.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(&format!(
        "## High Complexity Functions ({} functions)\n\n",
        high_complexity.len()
    ));

    output.push_str("| Location | Function | Complexity | Score |\n");
    output.push_str("|----------|----------|------------|-------|\n");

    for (smell, _) in high_complexity {
        if let SmellType::HighComplexity { name, line, .. } = &smell.smell_type {
            if let Some(file_path) = smell.files.first() {
                let location = smell
                    .locations
                    .first()
                    .map(format_location_detail)
                    .unwrap_or_else(|| crate::report::format_location(file_path, *line, None));
                let complexity = smell.complexity().unwrap_or(0);
                let score = smell.score(severity_config);
                output.push_str(&format!(
                    "| `{}` | `{}` | {} | {} pts |\n",
                    location, name, complexity, score
                ));
            }
        }
    }
    output.push('\n');

    output
}
