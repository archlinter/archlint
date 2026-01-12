use crate::detectors::{Explanation, SmellWithExplanation};
use std::collections::BTreeMap;
use std::path::{Component, Path};

pub fn append_explanation(output: &mut String, explanation: &Explanation) {
    output.push_str(&format!("**Problem:** {}\n\n", explanation.problem));
    output.push_str(&format!("**Reason:** {}\n\n", explanation.reason));

    output.push_str("**Risks:**\n");
    for risk in &explanation.risks {
        output.push_str(&format!("- {}\n", risk));
    }
    output.push('\n');

    output.push_str("**Recommendations:**\n");
    for rec in &explanation.recommendations {
        output.push_str(&format!("- {}\n", rec));
    }
    output.push('\n');
}

pub fn group_smells_by_detector(
    smells: &[SmellWithExplanation],
) -> std::collections::HashMap<String, Vec<&SmellWithExplanation>> {
    let mut grouped = std::collections::HashMap::new();
    for smell in smells {
        let detector_id = smell.0.smell_type.category().to_id().to_string();
        grouped
            .entry(detector_id)
            .or_insert_with(Vec::new)
            .push(smell);
    }
    grouped
}

fn extract_short_directory(file_path: &Path) -> String {
    let parent = match file_path.parent() {
        Some(p) => p,
        None => return ".".to_string(),
    };

    let components: Vec<_> = parent
        .components()
        .rev()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            _ => None,
        })
        .take(3)
        .collect();

    let result: Vec<_> = components.into_iter().rev().collect();
    if result.is_empty() {
        return ".".to_string();
    }
    result.join("/")
}

fn extract_filename(file_path: &Path) -> String {
    file_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

pub fn group_files_by_directory(
    dead_smells: &[&SmellWithExplanation],
) -> BTreeMap<String, Vec<String>> {
    let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (smell, _) in dead_smells {
        let file_path = match smell.files.first() {
            Some(f) => f,
            None => continue,
        };

        let dir = extract_short_directory(file_path);
        let filename = extract_filename(file_path);
        grouped.entry(dir).or_default().push(filename);
    }

    for files in grouped.values_mut() {
        files.sort();
    }

    grouped
}
