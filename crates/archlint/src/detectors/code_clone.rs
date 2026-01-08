use crate::config::Config;
use crate::detectors::{
    ArchSmell, CodeRange, Detector, DetectorCategory, DetectorFactory, DetectorInfo, LocationDetail,
};
use crate::engine::AnalysisContext;
use crate::parser::tokenizer::{tokenize_and_normalize, CloneTokenizationMode, NormalizedToken};
use oxc_span::SourceType;
use rustc_hash::{FxHashMap, FxHashSet};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

pub fn init() {}

pub struct CodeCloneDetector {
    min_tokens: usize,
    min_lines: usize,
    mode: CloneTokenizationMode,
}

pub struct CodeCloneDetectorFactory;

impl DetectorFactory for CodeCloneDetectorFactory {
    fn info(&self) -> DetectorInfo {
        DetectorInfo {
            id: "code_clone",
            name: "Code Clone Detector",
            description:
                "Detects duplicated code blocks across the project (Type-1 and Type-2 clones)",
            default_enabled: true,
            is_deep: true,
            category: DetectorCategory::Global,
        }
    }

    fn create(&self, config: &Config) -> Box<dyn Detector> {
        let rule = crate::rule_resolver::ResolvedRuleConfig::resolve(config, "code_clone", None);
        let min_tokens = rule.get_option("min_tokens").unwrap_or(50);
        let min_lines = rule.get_option("min_lines").unwrap_or(6);
        let mode = match rule
            .get_option::<String>("mode")
            .unwrap_or_else(|| "exact".to_string())
            .to_lowercase()
            .as_str()
        {
            "exact" | "type1" => CloneTokenizationMode::Exact,
            _ => CloneTokenizationMode::Type2,
        };

        Box::new(CodeCloneDetector {
            min_tokens,
            min_lines,
            mode,
        })
    }
}

inventory::submit! {
    &CodeCloneDetectorFactory as &dyn DetectorFactory
}

impl CodeCloneDetector {
    pub fn create_for_test(min_tokens: usize, min_lines: usize) -> Self {
        Self {
            min_tokens,
            min_lines,
            mode: CloneTokenizationMode::Type2,
        }
    }

    pub fn create_for_test_with_mode(
        min_tokens: usize,
        min_lines: usize,
        mode: CloneTokenizationMode,
    ) -> Self {
        Self {
            min_tokens,
            min_lines,
            mode,
        }
    }

    fn hash_window(&self, window: &[NormalizedToken]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for token in window {
            hasher.update(token.normalized.as_bytes());
            hasher.update([0]);
        }
        hasher.finalize().into()
    }

    fn hash_range(&self, tokens: &[NormalizedToken]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for token in tokens {
            hasher.update(token.normalized.as_bytes());
            hasher.update([0]);
        }
        hasher.finalize().into()
    }
}

impl Detector for CodeCloneDetector {
    fn name(&self) -> &'static str {
        "Code Clone"
    }

    fn detect(&self, ctx: &AnalysisContext) -> Vec<ArchSmell> {
        let mut smells = Vec::new();
        let mut file_tokens: FxHashMap<PathBuf, Vec<NormalizedToken>> = FxHashMap::default();

        // 1. Tokenize files
        for path in ctx.file_metrics.keys() {
            let rule = ctx.resolve_rule("code_clone", Some(path));
            if !rule.enabled || ctx.is_excluded(path, &rule.exclude) {
                continue;
            }

            if let Ok(source) = fs::read_to_string(path) {
                let source_type = SourceType::from_path(path).unwrap_or_default();
                let tokens = tokenize_and_normalize(&source, source_type, self.mode);
                if tokens.len() >= self.min_tokens {
                    file_tokens.insert(path.clone(), tokens);
                }
            }
        }

        if file_tokens.is_empty() {
            return smells;
        }

        // 2. Find matching window pairs
        // Map: Hash -> Vec<(File, Offset)>
        let mut window_map: FxHashMap<[u8; 32], Vec<(PathBuf, usize)>> = FxHashMap::default();
        let mut paths: Vec<_> = file_tokens.keys().collect();
        paths.sort();

        for path in paths {
            let tokens = &file_tokens[path];
            for i in 0..=(tokens.len().saturating_sub(self.min_tokens)) {
                let hash = self.hash_window(&tokens[i..i + self.min_tokens]);
                window_map.entry(hash).or_default().push((path.clone(), i));
            }
        }

        // 3. Expand matches and build clusters (one smell per clone-set)
        type WindowEntry = ([u8; 32], Vec<(PathBuf, usize)>);
        let mut window_entries: Vec<WindowEntry> = window_map.into_iter().collect();
        window_entries.sort_by(|a, b| a.0.cmp(&b.0));

        for entry in &mut window_entries {
            entry.1.sort();
        }

        #[derive(Debug, Clone)]
        struct Occurrence {
            file: PathBuf,
            token_start: usize,
            start_line: usize,
            start_column: usize,
            end_line: usize,
            end_column: usize,
        }

        #[derive(Debug, Clone)]
        struct Cluster {
            hash: [u8; 32],
            token_count: usize,
            occurrences: Vec<Occurrence>,
        }

        fn merge_overlapping_occurrences(
            mut occurrences: Vec<Occurrence>,
            _token_count: usize,
        ) -> Vec<Occurrence> {
            occurrences.sort_by(|a, b| a.file.cmp(&b.file).then(a.token_start.cmp(&b.token_start)));

            let mut merged: Vec<Occurrence> = Vec::new();
            for occ in occurrences {
                if let Some(last) = merged.last_mut() {
                    if last.file == occ.file {
                        // Only merge if they truly overlap in source lines
                        let overlaps = (occ.start_line >= last.start_line
                            && occ.start_line <= last.end_line)
                            || (occ.end_line >= last.start_line && occ.end_line <= last.end_line);

                        if overlaps {
                            last.end_line = last.end_line.max(occ.end_line);
                            last.token_start = last.token_start.min(occ.token_start);
                            continue;
                        }
                    }
                }
                merged.push(occ);
            }
            merged
        }

        let mut clusters: Vec<Cluster> = Vec::new();
        let mut occ_to_cluster: FxHashMap<(PathBuf, usize), usize> = FxHashMap::default();
        // file -> set of token indices already covered by some cluster
        let mut covered_tokens: FxHashMap<PathBuf, FxHashSet<usize>> = FxHashMap::default();

        for (_hash, locations) in window_entries {
            if locations.len() < 2 {
                continue;
            }

            for i in 0..locations.len() {
                for j in (i + 1)..locations.len() {
                    let (file1, off1) = &locations[i];
                    let (file2, off2) = &locations[j];

                    // For same-file clones, never skip based on covered tokens
                    // Different occurrences in the same file should all be detected
                    // For cross-file clones, skip if both positions are already covered
                    // (they would be part of a larger clone already detected)
                    if file1 != file2
                        && covered_tokens.get(file1).is_some_and(|s| s.contains(off1))
                        && covered_tokens.get(file2).is_some_and(|s| s.contains(off2))
                    {
                        continue;
                    }

                    let tokens1 = &file_tokens[file1];
                    let tokens2 = &file_tokens[file2];

                    // Expand forward
                    let mut start1 = *off1;
                    let mut start2 = *off2;
                    let mut length = self.min_tokens;

                    // For same-file clones: limit expansion to prevent overlapping occurrences
                    // We need to ensure start1+length doesn't reach start2 (or vice versa)
                    let (lower_start, higher_start) = if start1 < start2 {
                        (start1, start2)
                    } else {
                        (start2, start1)
                    };
                    let max_length_for_same_file = if file1 == file2 {
                        higher_start - lower_start
                    } else {
                        usize::MAX
                    };

                    while start1 + length < tokens1.len()
                        && start2 + length < tokens2.len()
                        && tokens1[start1 + length].normalized
                            == tokens2[start2 + length].normalized
                    {
                        // For same-file clones: stop before occurrences overlap
                        if file1 == file2 && length >= max_length_for_same_file {
                            break;
                        }
                        length += 1;
                    }

                    // Expand backward - but recalculate max_length for same-file clones
                    while start1 > 0
                        && start2 > 0
                        && tokens1[start1 - 1].normalized == tokens2[start2 - 1].normalized
                    {
                        // Check if backward expansion would cause overlap
                        let new_start1 = start1 - 1;
                        let new_start2 = start2 - 1;
                        let new_length = length + 1;

                        if file1 == file2 {
                            let (new_lower, new_higher) = if new_start1 < new_start2 {
                                (new_start1, new_start2)
                            } else {
                                (new_start2, new_start1)
                            };
                            if new_lower + new_length > new_higher {
                                break;
                            }
                        }

                        start1 = new_start1;
                        start2 = new_start2;
                        length = new_length;
                    }

                    let end1 = start1 + length;
                    let end2 = start2 + length;

                    if length < self.min_tokens {
                        continue;
                    }

                    // Skip clones that start in the middle of a line (not at a statement boundary)
                    let first1 = &tokens1[start1];
                    let first2 = &tokens2[start2];
                    let starts_at_line1 = start1 == 0 || tokens1[start1 - 1].line != first1.line;
                    let starts_at_line2 = start2 == 0 || tokens2[start2 - 1].line != first2.line;
                    if !starts_at_line1 || !starts_at_line2 {
                        continue;
                    }

                    // Calculate actual line boundaries by scanning all tokens in range
                    // This ensures we capture the full extent including closing brackets
                    let mut start_line1 = first1.line;
                    let mut start_column1 = first1.column;
                    let mut end_line1 = first1.end_line;
                    let mut end_column1 = first1.end_column;

                    let mut start_line2 = first2.line;
                    let mut start_column2 = first2.column;
                    let mut end_line2 = first2.end_line;
                    let mut end_column2 = first2.end_column;

                    for i in 0..length {
                        let t1 = &tokens1[start1 + i];
                        if t1.line < start_line1
                            || (t1.line == start_line1 && t1.column < start_column1)
                        {
                            start_line1 = t1.line;
                            start_column1 = t1.column;
                        }
                        if t1.end_line > end_line1
                            || (t1.end_line == end_line1 && t1.end_column > end_column1)
                        {
                            end_line1 = t1.end_line;
                            end_column1 = t1.end_column;
                        }

                        let t2 = &tokens2[start2 + i];
                        if t2.line < start_line2
                            || (t2.line == start_line2 && t2.column < start_column2)
                        {
                            start_line2 = t2.line;
                            start_column2 = t2.column;
                        }
                        if t2.end_line > end_line2
                            || (t2.end_line == end_line2 && t2.end_column > end_column2)
                        {
                            end_line2 = t2.end_line;
                            end_column2 = t2.end_column;
                        }
                    }

                    let lines1 = end_line1.saturating_sub(start_line1) + 1;
                    let lines2 = end_line2.saturating_sub(start_line2) + 1;

                    if lines1 < self.min_lines || lines2 < self.min_lines {
                        continue;
                    }

                    // Mark all tokens in the range as covered to avoid redundant sub-clones
                    for idx in start1..end1 {
                        covered_tokens.entry(file1.clone()).or_default().insert(idx);
                    }
                    for idx in start2..end2 {
                        covered_tokens.entry(file2.clone()).or_default().insert(idx);
                    }

                    let range_hash = self.hash_range(&tokens1[start1..end1]);

                    let occ1 = Occurrence {
                        file: file1.clone(),
                        token_start: start1,
                        start_line: start_line1,
                        start_column: start_column1,
                        end_line: end_line1,
                        end_column: end_column1,
                    };
                    let occ2 = Occurrence {
                        file: file2.clone(),
                        token_start: start2,
                        start_line: start_line2,
                        start_column: start_column2,
                        end_line: end_line2,
                        end_column: end_column2,
                    };

                    let key1 = (occ1.file.clone(), occ1.token_start);
                    let key2 = (occ2.file.clone(), occ2.token_start);

                    let c1 = occ_to_cluster.get(&key1).copied();
                    let c2 = occ_to_cluster.get(&key2).copied();

                    let target_cluster =
                        match (c1, c2) {
                            (Some(a), Some(b)) if a != b => {
                                let (keep, drop) = if a < b { (a, b) } else { (b, a) };
                                let dropped = clusters[drop].clone();
                                for occ in dropped.occurrences {
                                    let k = (occ.file.clone(), occ.token_start);
                                    if !clusters[keep].occurrences.iter().any(|o| {
                                        o.file == occ.file && o.token_start == occ.token_start
                                    }) {
                                        clusters[keep].occurrences.push(occ);
                                    }
                                    occ_to_cluster.insert(k, keep);
                                }
                                clusters[keep].hash = range_hash;
                                clusters[keep].token_count = length;
                                keep
                            }
                            (Some(a), _) => a,
                            (_, Some(b)) => b,
                            (None, None) => {
                                let idx = clusters.len();
                                clusters.push(Cluster {
                                    hash: range_hash,
                                    token_count: length,
                                    occurrences: vec![occ1.clone(), occ2.clone()],
                                });
                                occ_to_cluster.insert(key1, idx);
                                occ_to_cluster.insert(key2, idx);
                                continue;
                            }
                        };

                    if !clusters[target_cluster]
                        .occurrences
                        .iter()
                        .any(|o| o.file == occ1.file && o.token_start == occ1.token_start)
                    {
                        clusters[target_cluster].occurrences.push(occ1.clone());
                        occ_to_cluster.insert(key1, target_cluster);
                    }
                    if !clusters[target_cluster]
                        .occurrences
                        .iter()
                        .any(|o| o.file == occ2.file && o.token_start == occ2.token_start)
                    {
                        clusters[target_cluster].occurrences.push(occ2.clone());
                        occ_to_cluster.insert(key2, target_cluster);
                    }
                }
            }
        }

        for cluster in clusters.into_iter().filter(|c| c.occurrences.len() >= 2) {
            let hash_str = cluster
                .hash
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();

            // Merge overlaps inside each file to avoid reporting the same block many times.
            let merged_occurrences =
                merge_overlapping_occurrences(cluster.occurrences, cluster.token_count);

            if merged_occurrences.len() < 2 {
                continue;
            }

            let mut location_details = Vec::new();
            if let Some(occ) = merged_occurrences.first() {
                // De-duplicate other refs (file:line) and sort for stable output
                let mut seen = FxHashSet::<String>::default();
                let mut other_refs = merged_occurrences
                    .iter()
                    .filter(|o| !(o.file == occ.file && o.token_start == occ.token_start))
                    .filter_map(|o| {
                        let rel = o
                            .file
                            .strip_prefix(&ctx.project_path)
                            .unwrap_or(&o.file)
                            .to_string_lossy();
                        let key = if o.start_line == o.end_line {
                            format!("{}:{}", rel, o.start_line)
                        } else {
                            format!("{}:{}-{}", rel, o.start_line, o.end_line)
                        };
                        if seen.insert(key.clone()) {
                            Some(key)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                other_refs.sort();
                let other_refs = other_refs.join(", ");

                let description = format!(
                    "Duplicated code ({} tokens, lines {}-{}). Also found in: {}",
                    cluster.token_count, occ.start_line, occ.end_line, other_refs
                );

                let range = CodeRange {
                    start_line: occ.start_line,
                    start_column: occ.start_column,
                    end_line: occ.end_line,
                    end_column: occ.end_column,
                };

                location_details.push(
                    LocationDetail::new(occ.file.clone(), occ.start_line, description)
                        .with_range(range),
                );
            }

            smells.push(ArchSmell::new_code_clone(
                location_details,
                cluster.token_count,
                hash_str,
            ));
        }

        smells
    }
}
