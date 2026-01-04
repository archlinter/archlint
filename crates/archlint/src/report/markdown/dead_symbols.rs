use super::common::SmellWithExplanation;
use crate::detectors::SmellType;
use crate::explain::ExplainEngine;
use crate::report::format_location_detail;

pub fn generate(dead_symbols: &[&SmellWithExplanation]) -> String {
    if dead_symbols.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(&format!(
        "## Dead Symbols ({} symbols)\n\n",
        dead_symbols.len()
    ));

    output.push_str("| Location | Symbol | Kind |\n");
    output.push_str("|----------|--------|------|\n");

    for (smell, _) in dead_symbols {
        if let SmellType::DeadSymbol { name, kind } = &smell.smell_type {
            let location = smell
                .locations
                .first()
                .map(format_location_detail)
                .unwrap_or_else(|| {
                    smell
                        .files
                        .first()
                        .map(|f| ExplainEngine::format_file_path(f))
                        .unwrap_or_else(|| "unknown".to_string())
                });
            output.push_str(&format!("| `{}` | `{}` | {} |\n", location, name, kind));
        }
    }
    output.push('\n');

    output
}
