use crate::detectors::ArchSmell;
use crate::report::AnalysisReport;

pub struct ReportDiff {
    pub new_smells: Vec<ArchSmell>,
    pub fixed_smells: Vec<ArchSmell>,
    pub unchanged_smells: Vec<ArchSmell>,
}

impl ReportDiff {
    #[must_use]
    pub fn calculate(old: &AnalysisReport, new: &AnalysisReport) -> Self {
        let old_smells: Vec<&ArchSmell> = old.smells.iter().map(|(s, _)| s).collect();
        let new_smells: Vec<&ArchSmell> = new.smells.iter().map(|(s, _)| s).collect();

        let mut new_items: Vec<ArchSmell> = Vec::new();
        let mut fixed_items: Vec<ArchSmell> = Vec::new();
        let mut unchanged_items: Vec<ArchSmell> = Vec::new();

        // Find new and unchanged
        for &new_smell in &new_smells {
            if old_smells.contains(&new_smell) {
                unchanged_items.push(new_smell.clone());
            } else {
                new_items.push(new_smell.clone());
            }
        }

        // Find fixed
        for &old_smell in &old_smells {
            if !new_smells.contains(&old_smell) {
                fixed_items.push(old_smell.clone());
            }
        }

        Self {
            new_smells: new_items,
            fixed_smells: fixed_items,
            unchanged_smells: unchanged_items,
        }
    }
}
