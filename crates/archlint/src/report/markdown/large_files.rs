use super::common::{append_explanation, SmellWithExplanation};
use crate::config::SeverityConfig;
use crate::detectors::Severity;
use crate::explain::ExplainEngine;

pub fn generate(large_files: &[&SmellWithExplanation], severity_config: &SeverityConfig) -> String {
    if large_files.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(&format!("## Large Files ({} files)\n\n", large_files.len()));

    if let Some((_, explanation)) = large_files.first() {
        append_explanation(&mut output, explanation);
    }

    output.push_str("| File | Lines | Severity |\n");
    output.push_str("|------|-------|----------|\n");

    for (smell, _) in large_files {
        if let Some(file_path) = smell.files.first() {
            let formatted_path = ExplainEngine::format_file_path(file_path);
            let lines = smell.lines().unwrap_or(0);
            let effective_severity = smell.effective_severity();
            let score = smell.score(severity_config);

            let severity_str = match effective_severity {
                Severity::Low => "🔵 Low",
                Severity::Medium => "🟡 Medium",
                Severity::High => "🟠 High",
                Severity::Critical => "🔴 Critical",
            };
            output.push_str(&format!(
                "| `{}` | {} | {} ({} pts) |\n",
                formatted_path, lines, severity_str, score
            ));
        }
    }
    output.push('\n');

    output
}
