//! Fuzzy matching for smell IDs to handle line number shifts.
//!
//! When code is modified (lines added/removed above a smell), the line number in the ID changes.
//! Fuzzy matching identifies these "shifted" smells to avoid false positives in diff.

use crate::snapshot::{SmellDetails, SnapshotSmell};
use std::collections::HashMap;

/// Matcher for finding corresponding smells when exact ID matching fails.
pub struct FuzzyMatcher {
    /// Maximum line difference to consider smells as the same (default: 50)
    line_tolerance: usize,
}

impl Default for FuzzyMatcher {
    fn default() -> Self {
        Self { line_tolerance: 50 }
    }
}

/// A matched pair of baseline and current smells that represent the same issue.
#[derive(Debug)]
pub struct MatchedPair<'a> {
    pub baseline: &'a SnapshotSmell,
    pub current: &'a SnapshotSmell,
}

/// Key for grouping smells: (smell_type, file, symbol_name)
type SmellKey = (String, String, String);

impl FuzzyMatcher {
    pub fn new(line_tolerance: usize) -> Self {
        Self { line_tolerance }
    }

    /// Find matching pairs between orphaned baseline and current smells.
    ///
    /// Returns pairs of smells that are likely the same issue but with shifted line numbers.
    pub fn match_orphans<'a>(
        &self,
        orphaned_baseline: &[&'a SnapshotSmell],
        orphaned_current: &[&'a SnapshotSmell],
    ) -> Vec<MatchedPair<'a>> {
        // Group by key for efficient matching
        let baseline_by_key = Self::group_by_key(orphaned_baseline);
        let current_by_key = Self::group_by_key(orphaned_current);

        let mut matched = Vec::new();
        let mut used_current_ids: std::collections::HashSet<&str> =
            std::collections::HashSet::new();

        // For each baseline smell, try to find a matching current smell
        for (key, baseline_smells) in &baseline_by_key {
            if let Some(current_smells) = current_by_key.get(key) {
                for baseline in baseline_smells {
                    let baseline_line = Self::extract_line(baseline);

                    // Find best match by line proximity
                    let best_match = current_smells
                        .iter()
                        .filter(|c| !used_current_ids.contains(c.id.as_str()))
                        .filter_map(|current| {
                            let current_line = Self::extract_line(current);
                            let diff = baseline_line.abs_diff(current_line);
                            if diff <= self.line_tolerance {
                                Some((current, diff))
                            } else {
                                None
                            }
                        })
                        .min_by_key(|(_, diff)| *diff);

                    if let Some((current, _)) = best_match {
                        used_current_ids.insert(&current.id);
                        matched.push(MatchedPair { baseline, current });
                    }
                }
            }
        }

        matched
    }

    /// Group smells by their matching key.
    fn group_by_key<'a>(smells: &[&'a SnapshotSmell]) -> HashMap<SmellKey, Vec<&'a SnapshotSmell>> {
        let mut groups: HashMap<SmellKey, Vec<&'a SnapshotSmell>> = HashMap::new();

        for smell in smells {
            if let Some(key) = Self::extract_key(smell) {
                groups.entry(key).or_default().push(smell);
            }
        }

        groups
    }

    /// Extract matching key from smell: (smell_type, file, symbol_name).
    ///
    /// Returns None if key cannot be extracted (smell won't participate in fuzzy matching).
    fn extract_key(smell: &SnapshotSmell) -> Option<SmellKey> {
        let smell_type = smell.smell_type.clone();
        let file = smell.files.first()?.clone();

        let symbol_name = Self::extract_symbol_name(smell)?;

        Some((smell_type, file, symbol_name))
    }

    /// Extract symbol/function name from smell details or ID.
    fn extract_symbol_name(smell: &SnapshotSmell) -> Option<String> {
        // First, try to get from details
        if let Some(ref details) = smell.details {
            let name = match details {
                SmellDetails::Complexity { function_name, .. } => Some(function_name.clone()),
                SmellDetails::DeadSymbol { name, .. } => Some(name.clone()),
                SmellDetails::LongParameterList { function } => Some(function.clone()),
                SmellDetails::PrimitiveObsession { function } => Some(function.clone()),
                SmellDetails::SharedMutableState { symbol } => Some(symbol.clone()),
                SmellDetails::OrphanType { name } => Some(name.clone()),
                // These don't have symbol names that would shift
                SmellDetails::Cycle { .. }
                | SmellDetails::LayerViolation { .. }
                | SmellDetails::FeatureEnvy { .. }
                | SmellDetails::TestLeakage { .. }
                | SmellDetails::VendorCoupling { .. }
                | SmellDetails::PackageCycle { .. }
                | SmellDetails::ScatteredConfiguration { .. } => None,
            };

            if name.is_some() {
                return name;
            }
        }

        // Fallback: try to extract from ID
        // ID formats: "cmplx:path:name:line", "nest:path:desc:line", etc.
        Self::extract_name_from_id(&smell.id)
    }

    /// Try to extract symbol name from ID string.
    fn extract_name_from_id(id: &str) -> Option<String> {
        let parts: Vec<&str> = id.split(':').collect();

        // Expected formats:
        // - "cmplx:path:name:line" (4 parts)
        // - "nest:path:desc:line" (4 parts)
        // - "params:path:name:line" (4 parts)
        // - "dead:path:name:line" (4 parts)
        // - "sideeffect:path:line:hash" (4 parts, but name is in description)

        match parts.first() {
            Some(&"cmplx") | Some(&"nest") | Some(&"params") | Some(&"prim") | Some(&"dead")
            | Some(&"shared") | Some(&"orphan") => {
                // Format: prefix:path:name:line
                if parts.len() >= 4 {
                    Some(parts[2].to_string())
                } else {
                    None
                }
            }
            Some(&"sideeffect") => {
                // SideEffectImport uses hash, not symbol name - skip fuzzy matching
                None
            }
            _ => None,
        }
    }

    /// Extract line number from smell.
    fn extract_line(smell: &SnapshotSmell) -> usize {
        // First, try locations
        if let Some(loc) = smell.locations.first() {
            return loc.line;
        }

        // Then, try details
        if let Some(SmellDetails::Complexity { line, .. }) = &smell.details {
            return *line;
        }

        // Fallback: try to extract from ID (last part is usually line)
        Self::extract_line_from_id(&smell.id).unwrap_or(0)
    }

    /// Try to extract line number from ID string.
    fn extract_line_from_id(id: &str) -> Option<usize> {
        let parts: Vec<&str> = id.split(':').collect();
        parts.last()?.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_smell(id: &str, smell_type: &str, file: &str, line: usize) -> SnapshotSmell {
        SnapshotSmell {
            id: id.to_string(),
            smell_type: smell_type.to_string(),
            severity: "Medium".to_string(),
            files: vec![file.to_string()],
            metrics: HashMap::new(),
            details: Some(SmellDetails::Complexity {
                function_name: "testFunc".to_string(),
                line,
            }),
            locations: vec![crate::snapshot::Location {
                file: file.to_string(),
                line,
                column: None,
                range: None,
                description: None,
            }],
        }
    }

    #[test]
    fn test_shifted_smell_matches() {
        let baseline = make_smell(
            "cmplx:src/foo.ts:testFunc:10",
            "HighComplexity",
            "src/foo.ts",
            10,
        );
        let current = make_smell(
            "cmplx:src/foo.ts:testFunc:15",
            "HighComplexity",
            "src/foo.ts",
            15,
        );

        let matcher = FuzzyMatcher::new(10);
        let pairs = matcher.match_orphans(&[&baseline], &[&current]);

        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].baseline.id, baseline.id);
        assert_eq!(pairs[0].current.id, current.id);
    }

    #[test]
    fn test_different_functions_no_match() {
        let mut baseline = make_smell(
            "cmplx:src/foo.ts:funcA:10",
            "HighComplexity",
            "src/foo.ts",
            10,
        );
        baseline.details = Some(SmellDetails::Complexity {
            function_name: "funcA".to_string(),
            line: 10,
        });

        let mut current = make_smell(
            "cmplx:src/foo.ts:funcB:15",
            "HighComplexity",
            "src/foo.ts",
            15,
        );
        current.details = Some(SmellDetails::Complexity {
            function_name: "funcB".to_string(),
            line: 15,
        });

        let matcher = FuzzyMatcher::new(10);
        let pairs = matcher.match_orphans(&[&baseline], &[&current]);

        assert_eq!(pairs.len(), 0);
    }

    #[test]
    fn test_too_far_shift_no_match() {
        let baseline = make_smell(
            "cmplx:src/foo.ts:testFunc:10",
            "HighComplexity",
            "src/foo.ts",
            10,
        );
        let current = make_smell(
            "cmplx:src/foo.ts:testFunc:100",
            "HighComplexity",
            "src/foo.ts",
            100,
        );

        let matcher = FuzzyMatcher::new(10); // Only 10 lines tolerance
        let pairs = matcher.match_orphans(&[&baseline], &[&current]);

        assert_eq!(pairs.len(), 0);
    }

    #[test]
    fn test_extract_line_from_id() {
        assert_eq!(
            FuzzyMatcher::extract_line_from_id("cmplx:path:func:42"),
            Some(42)
        );
        assert_eq!(
            FuzzyMatcher::extract_line_from_id("nest:path:desc:100"),
            Some(100)
        );
        assert_eq!(FuzzyMatcher::extract_line_from_id("invalid"), None);
    }

    #[test]
    fn test_extract_name_from_id() {
        assert_eq!(
            FuzzyMatcher::extract_name_from_id("cmplx:src/foo.ts:myFunc:42"),
            Some("myFunc".to_string())
        );
        assert_eq!(
            FuzzyMatcher::extract_name_from_id("dead:src/bar.ts:unusedVar:10"),
            Some("unusedVar".to_string())
        );
        assert_eq!(
            FuzzyMatcher::extract_name_from_id("sideeffect:path:10:hash"),
            None
        );
    }
}
