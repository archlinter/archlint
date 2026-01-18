//! Fuzzy matching for smell IDs to handle line number shifts.
//!
//! When code is modified (lines added/removed above a smell), the line number in the ID changes.
//! Fuzzy matching identifies these "shifted" smells to avoid false positives in diff.

use crate::detectors::SmellType;
use crate::snapshot::SnapshotSmell;
use std::collections::{BTreeMap, HashSet};

/// Smell type prefixes that support symbol-based fuzzy matching.
/// Side-effect smells use hash-based IDs and are excluded.
const SYMBOL_BASED_PREFIXES: &[&str] = &[
    "cmplx", "ccycl", "ccog", "nest", "params", "prim", "dead", "shared", "orphan", "lcom",
];

/// Matcher for finding corresponding smells when exact ID matching fails.
///
/// It uses a "fuzzy" approach by grouping smells by their type and symbol name,
/// then matching them based on line proximity within a given tolerance.
pub struct FuzzyMatcher {
    /// Maximum line difference to consider smells as the same (default: 50)
    line_tolerance: usize,
}

impl Default for FuzzyMatcher {
    /// Creates a `FuzzyMatcher` with default tolerance (50 lines).
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

/// Key for grouping smells for fuzzy matching.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SmellKey {
    /// The type of architectural smell.
    smell_type: String,
    /// The file path where the smell was detected.
    file: String,
}

impl FuzzyMatcher {
    /// Create a new `FuzzyMatcher` with the specified line tolerance.
    #[must_use]
    pub const fn new(line_tolerance: usize) -> Self {
        Self { line_tolerance }
    }

    /// Find matching pairs between orphaned baseline and current smells.
    ///
    /// Returns pairs of smells that are likely the same issue but with shifted line numbers.
    /// Orphaned smells are those that didn't have an exact ID match.
    #[must_use]
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
    /// within the configured tolerance. Matching symbol names are preferred.
    fn find_best_match<'a>(
        &self,
        baseline: &SnapshotSmell,
        candidates: &[&'a SnapshotSmell],
        used_ids: &HashSet<String>,
    ) -> Option<&'a SnapshotSmell> {
        let baseline_line = Self::extract_line(baseline)?;
        let baseline_name = Self::extract_symbol_name(baseline);

        candidates
            .iter()
            .filter(|c| !used_ids.contains(&c.id))
            .filter_map(|current| {
                let current_line = Self::extract_line(current)?;
                let diff = baseline_line.abs_diff(current_line);
                if diff <= self.line_tolerance {
                    let current_name = Self::extract_symbol_name(current);
                    let name_matches = baseline_name.is_some()
                        && current_name.is_some()
                        && baseline_name == current_name;
                    Some((current, name_matches, diff))
                } else {
                    None
                }
            })
            .min_by_key(|(_, name_matches, diff)| (!name_matches, *diff))
            .map(|(current, _, _)| *current)
    }

    /// Group smells by their matching key for efficient lookup.
    ///
    /// Key format: (`smell_type`, `first_file`).
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

    /// Extract matching key from smell: (`smell_type`, file).
    ///
    /// Fuzzy matching is only supported for "symbol-based" smells that affect a single file.
    /// Multi-file smells (like cyclic dependencies between multiple files) are excluded
    /// because line shifts in one file don't unambiguously represent a shift of the
    /// entire multi-file architectural issue.
    ///
    /// Returns None if key cannot be extracted (smell won't participate in fuzzy matching).
    fn extract_key(smell: &SnapshotSmell) -> Option<SmellKey> {
        // Multi-file smells are not supported for fuzzy matching yet
        if smell.files.len() != 1 {
            return None;
        }

        let smell_type = smell.smell_type.clone();
        let file = smell.files[0].clone();

        Some(SmellKey { smell_type, file })
    }

    /// Extract symbol/function name from smell details or ID.
    #[must_use]
    pub fn extract_symbol_name(smell: &SnapshotSmell) -> Option<String> {
        // First, try to get from details
        if let Some(ref details) = smell.details {
            let name = match details {
                SmellType::HighCyclomaticComplexity { name, .. } => Some(name.clone()),
                SmellType::HighCognitiveComplexity { name, .. } => Some(name.clone()),
                SmellType::DeadSymbol { name, .. } => Some(name.clone()),
                SmellType::LongParameterList { name, .. } => Some(name.clone()),
                SmellType::PrimitiveObsession { name, .. } => Some(name.clone()),
                SmellType::SharedMutableState { symbol } => Some(symbol.clone()),
                SmellType::OrphanType { name } => Some(name.clone()),
                SmellType::LowCohesion { class_name, .. } => Some(class_name.clone()),
                SmellType::DeepNesting { name, .. } => Some(name.clone()),
                // These don't have symbol names that would shift
                SmellType::CyclicDependency
                | SmellType::CyclicDependencyCluster
                | SmellType::GodModule
                | SmellType::DeadCode
                | SmellType::LargeFile
                | SmellType::UnstableInterface
                | SmellType::FeatureEnvy { .. }
                | SmellType::ShotgunSurgery
                | SmellType::HubDependency { .. }
                | SmellType::TestLeakage { .. }
                | SmellType::LayerViolation { .. }
                | SmellType::SdpViolation
                | SmellType::BarrelFileAbuse
                | SmellType::VendorCoupling { .. }
                | SmellType::SideEffectImport
                | SmellType::HubModule
                | SmellType::ScatteredModule { .. }
                | SmellType::HighCoupling { .. }
                | SmellType::PackageCycle { .. }
                | SmellType::CircularTypeDependency
                | SmellType::AbstractnessViolation
                | SmellType::ScatteredConfiguration { .. }
                | SmellType::CodeClone { .. }
                | SmellType::Unknown { .. } => None,
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
    /// Expected format from right: ...:`symbol_name:line`
    fn extract_name_from_id(id: &str) -> Option<String> {
        // We expect at least 3 parts: prefix:path...:name:line
        let mut it = id.rsplitn(3, ':');
        let _line_part = it.next()?;
        let name_part = it.next()?;
        let prefix_part = it.next().unwrap_or("");

        // Extract prefix from the remaining left part
        let prefix = prefix_part.split(':').next()?;

        if SYMBOL_BASED_PREFIXES.contains(&prefix) {
            Some(name_part.to_string())
        } else {
            // Side-effect and other smells don't use symbol names for fuzzy matching
            None
        }
    }

    /// Extract line number from smell.
    #[must_use]
    pub fn extract_line(smell: &SnapshotSmell) -> Option<usize> {
        // First, try locations
        if let Some(loc) = smell.locations.first() {
            return Some(loc.line);
        }

        // Then, try details
        if let Some(ref details) = smell.details {
            match details {
                SmellType::HighCyclomaticComplexity { line, .. } => return Some(*line),
                SmellType::HighCognitiveComplexity { line, .. } => return Some(*line),
                SmellType::DeepNesting { line, .. } => return Some(*line),
                _ => {}
            }
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
            details: Some(SmellType::HighCyclomaticComplexity {
                name: "testFunc".to_string(),
                line,
                complexity: 0,
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
            "HighCyclomaticComplexity",
            "src/foo.ts",
            10,
        );
        let current = make_smell(
            "cmplx:src/foo.ts:testFunc:15",
            "HighCyclomaticComplexity",
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
    fn test_renamed_function_matches_by_proximity() {
        let mut baseline = make_smell(
            "cmplx:src/foo.ts:funcA:10",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            10,
        );
        baseline.details = Some(SmellType::HighCyclomaticComplexity {
            name: "funcA".to_string(),
            line: 10,
            complexity: 0,
        });

        let mut current = make_smell(
            "cmplx:src/foo.ts:funcX:12",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            12,
        );
        current.details = Some(SmellType::HighCyclomaticComplexity {
            name: "funcX".to_string(),
            line: 12,
            complexity: 0,
        });

        let matcher = FuzzyMatcher::new(10);
        let pairs = matcher.match_orphans(&[&baseline], &[&current]);

        assert_eq!(
            pairs.len(),
            1,
            "Renamed functions should match if close enough"
        );
        assert_eq!(pairs[0].baseline.id, baseline.id);
        assert_eq!(pairs[0].current.id, current.id);
    }

    #[test]
    fn test_prefer_matching_name_over_closer_proximity() {
        let mut baseline = make_smell(
            "cmplx:src/foo.ts:target:10",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            10,
        );
        baseline.details = Some(SmellType::HighCyclomaticComplexity {
            name: "target".to_string(),
            line: 10,
            complexity: 0,
        });

        // Candidate 1: matching name but further (diff 5)
        let mut current1 = make_smell(
            "cmplx:src/foo.ts:target:15",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            15,
        );
        current1.details = Some(SmellType::HighCyclomaticComplexity {
            name: "target".to_string(),
            line: 15,
            complexity: 0,
        });

        // Candidate 2: different name but closer (diff 2)
        let mut current2 = make_smell(
            "cmplx:src/foo.ts:other:12",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            12,
        );
        current2.details = Some(SmellType::HighCyclomaticComplexity {
            name: "other".to_string(),
            line: 12,
            complexity: 0,
        });

        let matcher = FuzzyMatcher::new(10);
        let pairs = matcher.match_orphans(&[&baseline], &[&current1, &current2]);

        assert_eq!(pairs.len(), 1);
        assert_eq!(
            pairs[0].current.id, current1.id,
            "Should prefer matching name even if slightly further away"
        );
    }

    #[test]
    fn test_too_far_shift_no_match() {
        let baseline = make_smell(
            "cmplx:src/foo.ts:testFunc:10",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            10,
        );
        let current = make_smell(
            "cmplx:src/foo.ts:testFunc:100",
            "HighCyclomaticComplexity",
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

    #[test]
    fn test_deterministic_selection_same_distance() {
        let baseline = make_smell(
            "cmplx:src/foo.ts:test:10",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            10,
        );
        // Two candidates at the same distance (5 lines away)
        let current1 = make_smell(
            "cmplx:src/foo.ts:test:5",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            5,
        );
        let current2 = make_smell(
            "cmplx:src/foo.ts:test:15",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            15,
        );

        let matcher = FuzzyMatcher::new(10);
        let pairs = matcher.match_orphans(&[&baseline], &[&current1, &current2]);

        assert_eq!(pairs.len(), 1);
        // Should select current1 because it appears first in the list (min_by_key stability or list order)
        // min_by_key returns the first one found if values are equal.
        assert_eq!(pairs[0].current.id, current1.id);
    }

    #[test]
    fn test_already_matched_not_reused() {
        let baseline1 = make_smell(
            "cmplx:src/foo.ts:test:10",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            10,
        );
        let baseline2 = make_smell(
            "cmplx:src/foo.ts:test:12",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            12,
        );
        let current = make_smell(
            "cmplx:src/foo.ts:test:11",
            "HighCyclomaticComplexity",
            "src/foo.ts",
            11,
        );

        let matcher = FuzzyMatcher::new(10);
        let pairs = matcher.match_orphans(&[&baseline1, &baseline2], &[&current]);

        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].baseline.id, baseline1.id);
        assert_eq!(pairs[0].current.id, current.id);
    }

    #[test]
    fn test_empty_lists() {
        let matcher = FuzzyMatcher::new(10);
        let pairs = matcher.match_orphans(&[], &[]);
        assert!(pairs.is_empty());
    }

    #[test]
    fn test_multi_file_smell_excluded() {
        let mut multi_file = make_smell("cycle:abc", "CyclicDependency", "src/a.ts", 10);
        multi_file.files.push("src/b.ts".to_string());

        let matcher = FuzzyMatcher::new(10);
        let pairs = matcher.match_orphans(&[&multi_file], &[&multi_file]);

        assert!(
            pairs.is_empty(),
            "Multi-file smells should be excluded from fuzzy matching"
        );
    }

    #[test]
    fn test_low_cohesion_symbol_extraction() {
        let smell = SnapshotSmell {
            id: "lcom:src/file.ts:MyClass:10".to_string(),
            smell_type: "LowCohesion".to_string(),
            severity: "Medium".to_string(),
            files: vec!["src/file.ts".to_string()],
            metrics: HashMap::new(),
            details: Some(SmellType::LowCohesion {
                lcom: 0,
                class_name: "MyClass".to_string(),
            }),
            locations: vec![],
        };

        assert_eq!(
            FuzzyMatcher::extract_symbol_name(&smell),
            Some("MyClass".to_string())
        );
    }

    #[test]
    fn test_hub_dependency_excluded() {
        let smell = SnapshotSmell {
            id: "hub_dep:axios".to_string(),
            smell_type: "HubDependency".to_string(),
            severity: "High".to_string(),
            files: vec![], // HubDependency has no project files
            metrics: HashMap::new(),
            details: Some(SmellType::HubDependency {
                package: "axios".to_string(),
            }),
            locations: vec![],
        };

        assert!(
            FuzzyMatcher::extract_symbol_name(&smell).is_none(),
            "HubDependency should not have a symbol name for fuzzy matching"
        );
        // HubDependency returns None from extract_key because files.is_empty(),
        // not specifically because of the HubDependency type itself.
        assert!(
            FuzzyMatcher::extract_key(&smell).is_none(),
            "HubDependency should be excluded from fuzzy matching (no files)"
        );
    }

    #[test]
    fn test_cognitive_complexity_fuzzy_matching() {
        let baseline = make_smell(
            "ccog:src/foo.ts:test:10",
            "HighCognitiveComplexity",
            "src/foo.ts",
            10,
        );
        let current = make_smell(
            "ccog:src/foo.ts:test:12",
            "HighCognitiveComplexity",
            "src/foo.ts",
            12,
        );

        let matcher = FuzzyMatcher::new(10);
        let pairs = matcher.match_orphans(&[&baseline], &[&current]);

        assert_eq!(
            pairs.len(),
            1,
            "Cognitive complexity should match by proximity"
        );
        assert_eq!(pairs[0].baseline.id, baseline.id);
        assert_eq!(pairs[0].current.id, current.id);
    }
}
