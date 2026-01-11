//! Fuzzy matching for smell IDs to handle line number shifts.
//!
//! When code is modified (lines added/removed above a smell), the line number in the ID changes.
//! Fuzzy matching identifies these "shifted" smells to avoid false positives in diff.

use crate::snapshot::{SmellDetails, SnapshotSmell};
use std::collections::{BTreeMap, HashSet};

/// Matcher for finding corresponding smells when exact ID matching fails.
///
/// It uses a "fuzzy" approach by grouping smells by their type and symbol name,
/// then matching them based on line proximity within a given tolerance.
pub struct FuzzyMatcher {
    /// Maximum line difference to consider smells as the same (default: 50)
    line_tolerance: usize,
}

impl Default for FuzzyMatcher {
    /// Creates a FuzzyMatcher with default tolerance (50 lines).
    fn default() -> Self {
        Self { line_tolerance: 50 }
    }
}

/// A matched pair of baseline and current smells that represent the same issue.
#[derive(Debug)]
pub struct MatchedPair<'a> {
    /// The smell from the baseline snapshot.
    pub baseline: &'a SnapshotSmell,
    /// The corresponding smell from the current snapshot.
    pub current: &'a SnapshotSmell,
}

/// Key for grouping smells: (smell_type, file, symbol_name)
type SmellKey = (String, String, String);

impl FuzzyMatcher {
    /// Create a new FuzzyMatcher with the specified line tolerance.
    pub fn new(line_tolerance: usize) -> Self {
        Self { line_tolerance }
    }

    /// Find matching pairs between orphaned baseline and current smells.
    ///
    /// Returns pairs of smells that are likely the same issue but with shifted line numbers.
    /// Orphaned smells are those that didn't have an exact ID match.
    pub fn match_orphans<'a>(
        &self,
        orphaned_baseline: &[&'a SnapshotSmell],
        orphaned_current: &[&'a SnapshotSmell],
    ) -> Vec<MatchedPair<'a>> {
        let baseline_by_key = Self::group_by_key(orphaned_baseline);
        let current_by_key = Self::group_by_key(orphaned_current);

        let mut matched = Vec::new();
        let mut used_current_ids = HashSet::new();

        for (key, baseline_smells) in &baseline_by_key {
            let Some(current_smells) = current_by_key.get(key) else {
                continue;
            };

            for baseline in baseline_smells {
                if let Some(current) =
                    self.find_best_match(baseline, current_smells, &used_current_ids)
                {
                    used_current_ids.insert(current.id.clone());
                    matched.push(MatchedPair { baseline, current });
                }
            }
        }

        matched
    }

    /// Find the best matching current smell for a given baseline smell from a list of candidates.
    ///
    /// Best match is defined as the one with the smallest line number difference
    /// within the configured tolerance.
    fn find_best_match<'a>(
        &self,
        baseline: &SnapshotSmell,
        candidates: &[&'a SnapshotSmell],
        used_ids: &HashSet<String>,
    ) -> Option<&'a SnapshotSmell> {
        let baseline_line = Self::extract_line(baseline)?;

        candidates
            .iter()
            .filter(|c| !used_ids.contains(&c.id))
            .filter_map(|current| {
                let current_line = Self::extract_line(current)?;
                let diff = baseline_line.abs_diff(current_line);
                (diff <= self.line_tolerance).then_some((current, diff))
            })
            .min_by_key(|(_, diff)| *diff)
            .map(|(current, _)| *current)
    }

    /// Group smells by their matching key for efficient lookup.
    ///
    /// Key format: (smell_type, first_file, symbol_name).
    fn group_by_key<'a>(
        smells: &[&'a SnapshotSmell],
    ) -> BTreeMap<SmellKey, Vec<&'a SnapshotSmell>> {
        let mut groups: BTreeMap<SmellKey, Vec<&'a SnapshotSmell>> = BTreeMap::new();

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
        // Multi-file smells are not supported for fuzzy matching yet
        if smell.files.len() != 1 {
            return None;
        }

        let smell_type = smell.smell_type.clone();
        let file = smell.files[0].clone();

        let symbol_name = Self::extract_symbol_name(smell)?;

        Some((smell_type, file, symbol_name))
    }

    /// Extract symbol/function name from smell details or ID.
    pub fn extract_symbol_name(smell: &SnapshotSmell) -> Option<String> {
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
        Self::extract_name_from_id(&smell.id)
    }

    /// Try to extract symbol name from ID string.
    ///
    /// Resilience: parses from the right to handle ':' in paths or descriptions.
    /// Expected format from right: ...:symbol_name:line
    fn extract_name_from_id(id: &str) -> Option<String> {
        // We expect at least 3 parts: prefix:path...:name:line
        let mut it = id.rsplitn(3, ':');
        let _line_part = it.next()?;
        let name_part = it.next()?;
        let prefix_part = it.next().unwrap_or("");

        // Extract prefix from the remaining left part
        let prefix = prefix_part.split(':').next()?;

        match prefix {
            "cmplx" | "nest" | "params" | "prim" | "dead" | "shared" | "orphan" => {
                Some(name_part.to_string())
            }
            "sideeffect" => {
                // SideEffectImport uses hash, not symbol name - skip fuzzy matching
                None
            }
            _ => None,
        }
    }

    /// Extract line number from smell.
    pub fn extract_line(smell: &SnapshotSmell) -> Option<usize> {
        // First, try locations
        if let Some(loc) = smell.locations.first() {
            return Some(loc.line);
        }

        // Then, try details
        if let Some(SmellDetails::Complexity { line, .. }) = &smell.details {
            return Some(*line);
        }

        // Fallback: try to extract from ID
        Self::extract_line_from_id(&smell.id)
    }

    /// Try to extract line number from ID string.
    ///
    /// Resilience: tries last segment, then second-to-last (to handle ...:line:hash).
    fn extract_line_from_id(id: &str) -> Option<usize> {
        let mut it = id.rsplit(':');
        let last = it.next()?;

        // Case 1: Standard format prefix:path:name:line
        if let Ok(line) = last.parse::<usize>() {
            return Some(line);
        }

        // Case 2: Side-effect format sideeffect:path:line:hash
        let second_last = it.next()?;
        second_last.parse::<usize>().ok()
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
        assert_eq!(
            FuzzyMatcher::extract_line_from_id("sideeffect:path:10:hash"),
            Some(10)
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

    #[test]
    fn test_fallback_matching_from_ids() {
        // Smell with NO locations and NO details, only standard ID
        let baseline = SnapshotSmell {
            id: "dead:src/foo.ts:unusedVar:10".to_string(),
            smell_type: "DeadSymbol".to_string(),
            severity: "Medium".to_string(),
            files: vec!["src/foo.ts".to_string()],
            metrics: HashMap::new(),
            details: None,
            locations: vec![],
        };

        let current = SnapshotSmell {
            id: "dead:src/foo.ts:unusedVar:15".to_string(),
            smell_type: "DeadSymbol".to_string(),
            severity: "Medium".to_string(),
            files: vec!["src/foo.ts".to_string()],
            metrics: HashMap::new(),
            details: None,
            locations: vec![],
        };

        let matcher = FuzzyMatcher::new(10);
        let pairs = matcher.match_orphans(&[&baseline], &[&current]);

        assert_eq!(pairs.len(), 1, "Should match using fallbacks from ID");
        assert_eq!(pairs[0].baseline.id, baseline.id);
        assert_eq!(pairs[0].current.id, current.id);
    }
}
