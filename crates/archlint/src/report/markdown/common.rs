use crate::detectors::ArchSmell;
use crate::explain::Explanation;
use std::collections::BTreeMap;

pub type SmellWithExplanation = (ArchSmell, Explanation);

pub struct FilteredSmells<'a> {
    pub cycles: Vec<&'a SmellWithExplanation>,
    pub cycle_clusters: Vec<&'a SmellWithExplanation>,
    pub gods: Vec<&'a SmellWithExplanation>,
    pub dead: Vec<&'a SmellWithExplanation>,
    pub dead_symbols: Vec<&'a SmellWithExplanation>,
    pub high_complexity: Vec<&'a SmellWithExplanation>,
    pub large_files: Vec<&'a SmellWithExplanation>,
    pub unstable_interfaces: Vec<&'a SmellWithExplanation>,
    pub feature_envy: Vec<&'a SmellWithExplanation>,
    pub shotgun_surgery: Vec<&'a SmellWithExplanation>,
    pub hub_dependencies: Vec<&'a SmellWithExplanation>,
}

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

pub fn filter_smells(smells: &[SmellWithExplanation]) -> FilteredSmells<'_> {
    use crate::detectors::SmellType;

    FilteredSmells {
        cycles: smells
            .iter()
            .filter(|(s, _)| {
                matches!(
                    s.smell_type,
                    SmellType::CyclicDependency | SmellType::CyclicDependencyCluster
                )
            })
            .collect(),
        cycle_clusters: smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::CyclicDependencyCluster))
            .collect(),
        gods: smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::GodModule))
            .collect(),
        dead: smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::DeadCode))
            .collect(),
        dead_symbols: smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::DeadSymbol { .. }))
            .collect(),
        high_complexity: smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::HighComplexity { .. }))
            .collect(),
        large_files: smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::LargeFile))
            .collect(),
        unstable_interfaces: smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::UnstableInterface))
            .collect(),
        feature_envy: smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::FeatureEnvy { .. }))
            .collect(),
        shotgun_surgery: smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::ShotgunSurgery))
            .collect(),
        hub_dependencies: smells
            .iter()
            .filter(|(s, _)| matches!(s.smell_type, SmellType::HubDependency { .. }))
            .collect(),
    }
}

pub fn group_files_by_directory(
    dead_smells: &[&SmellWithExplanation],
) -> BTreeMap<String, Vec<String>> {
    let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (smell, _) in dead_smells {
        if let Some(file_path) = smell.files.first() {
            let path = std::path::Path::new(file_path);

            let dir = if let Some(parent) = path.parent() {
                if let Some(parent_str) = parent.to_str() {
                    let components: Vec<_> = parent_str.split('/').collect();
                    let start = if components.len() > 3 {
                        components.len() - 3
                    } else {
                        0
                    };
                    components[start..].join("/")
                } else {
                    "unknown".to_string()
                }
            } else {
                ".".to_string()
            };

            let filename = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            grouped.entry(dir).or_default().push(filename);
        }
    }

    for files in grouped.values_mut() {
        files.sort();
    }

    grouped
}
