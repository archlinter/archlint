use super::types::{Cluster, Occurrence, WindowEntry};
use crate::parser::tokenizer::NormalizedToken;
use rustc_hash::FxHashMap;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

/// Map to track processed ranges for each file pair.
type ProcessedRangesMap = FxHashMap<(PathBuf, PathBuf), Vec<(usize, usize, usize)>>;

/// Context for the clone detection process.
struct DetectionContext<'a> {
    file_tokens: &'a FxHashMap<PathBuf, Vec<NormalizedToken>>,
    clusters: &'a mut Vec<Cluster>,
    occ_to_cluster: &'a mut FxHashMap<(PathBuf, usize), usize>,
    /// Track already processed ranges for each file pair to avoid redundant expansions.
    /// Key is (file1, file2) where file1 <= file2 (alphabetically).
    /// Value is a list of (start1, start2, length).
    processed_ranges: ProcessedRangesMap,
    min_tokens: usize,
    min_lines: usize,
}

impl DetectionContext<'_> {
    /// Processes a pair of token window locations to detect and record clones.
    fn process_location_pair(&mut self, loc1: &(PathBuf, usize), loc2: &(PathBuf, usize)) {
        let (file1, off1) = loc1;
        let (file2, off2) = loc2;

        // Ensure deterministic key for processed_ranges
        let (f1, f2, o1, o2) = if file1 <= file2 {
            (file1, file2, *off1, *off2)
        } else {
            (file2, file1, *off2, *off1)
        };

        let pair_key = (f1.clone(), f2.clone());
        if let Some(ranges) = self.processed_ranges.get(&pair_key) {
            for &(s1, s2, len) in ranges {
                if o1 >= s1
                    && o1 <= s1 + len - self.min_tokens
                    && o2 >= s2
                    && o2 <= s2 + len - self.min_tokens
                {
                    return;
                }
            }
        }

        let tokens1 = &self.file_tokens[file1];
        let tokens2 = &self.file_tokens[file2];

        let (start1, start2, length) = expand_match(
            tokens1,
            tokens2,
            *off1,
            *off2,
            self.min_tokens,
            file1 == file2,
        );

        if length < self.min_tokens {
            return;
        }

        // Record the range to avoid re-processing overlapping windows
        let (rs1, rs2) = if file1 <= file2 {
            (start1, start2)
        } else {
            (start2, start1)
        };
        self.processed_ranges
            .entry(pair_key)
            .or_default()
            .push((rs1, rs2, length));

        if !starts_at_line_boundary(tokens1, start1) || !starts_at_line_boundary(tokens2, start2) {
            return;
        }

        let (sl1, sc1, el1, ec1) = calculate_range_bounds(tokens1, start1, length);
        let (sl2, sc2, el2, ec2) = calculate_range_bounds(tokens2, start2, length);

        let lines1 = el1.saturating_sub(sl1) + 1;
        let lines2 = el2.saturating_sub(sl2) + 1;

        if lines1 < self.min_lines || lines2 < self.min_lines {
            return;
        }

        let range_hash = hash_tokens(&tokens1[start1..start1 + length]);
        let occ1 = Occurrence {
            file: file1.clone(),
            token_start: start1,
            start_line: sl1,
            start_column: sc1,
            end_line: el1,
            end_column: ec1,
        };
        let occ2 = Occurrence {
            file: file2.clone(),
            token_start: start2,
            start_line: sl2,
            start_column: sc2,
            end_line: el2,
            end_column: ec2,
        };

        self.update_clusters(occ1, occ2, range_hash, length);
    }

    /// Updates or merges clusters with a newly discovered match.
    fn update_clusters(
        &mut self,
        occ1: Occurrence,
        occ2: Occurrence,
        range_hash: [u8; 32],
        length: usize,
    ) {
        let key1 = (occ1.file.clone(), occ1.token_start);
        let key2 = (occ2.file.clone(), occ2.token_start);

        let c1 = self.occ_to_cluster.get(&key1).copied();
        let c2 = self.occ_to_cluster.get(&key2).copied();

        let target_idx = match (c1, c2) {
            (Some(a), Some(b)) if a != b => {
                let (keep, drop) = if a < b { (a, b) } else { (b, a) };
                let dropped_occurrences = std::mem::take(&mut self.clusters[drop].occurrences);
                for occ in dropped_occurrences {
                    let k = (occ.file.clone(), occ.token_start);
                    self.add_to_cluster(keep, occ, k);
                }
                if length > self.clusters[keep].token_count {
                    self.clusters[keep].hash = range_hash;
                    self.clusters[keep].token_count = length;
                }
                keep
            }
            (Some(a), _) => a,
            (_, Some(b)) => b,
            (None, None) => {
                let idx = self.clusters.len();
                self.clusters.push(Cluster {
                    hash: range_hash,
                    token_count: length,
                    occurrences: vec![occ1.clone(), occ2.clone()],
                });
                self.occ_to_cluster.insert(key1, idx);
                self.occ_to_cluster.insert(key2, idx);
                return;
            }
        };

        self.add_to_cluster(target_idx, occ1, key1);
        self.add_to_cluster(target_idx, occ2, key2);
    }

    fn add_to_cluster(&mut self, cluster_idx: usize, occ: Occurrence, key: (PathBuf, usize)) {
        let cluster = &mut self.clusters[cluster_idx];
        if !cluster
            .occurrences
            .iter()
            .any(|o| o.file == occ.file && o.token_start == occ.token_start)
        {
            cluster.occurrences.push(occ);
            self.occ_to_cluster.insert(key, cluster_idx);
        }
    }
}

/// Hashes a sequence of tokens into a deterministic 32-byte hash.
pub fn hash_tokens(tokens: &[NormalizedToken]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for token in tokens {
        hasher.update(token.normalized.as_bytes());
        hasher.update([0]);
    }
    hasher.finalize().into()
}

/// Builds a map of token window hashes to their locations in the project.
///
/// This is the first step in the sliding window algorithm.
pub fn build_window_map(
    file_tokens: &FxHashMap<PathBuf, Vec<NormalizedToken>>,
    min_tokens: usize,
) -> FxHashMap<[u8; 32], Vec<(PathBuf, usize)>> {
    let mut window_map: FxHashMap<[u8; 32], Vec<(PathBuf, usize)>> = FxHashMap::default();
    let mut paths: Vec<_> = file_tokens.keys().collect();
    paths.sort();

    for path in paths {
        let tokens = &file_tokens[path];
        if tokens.len() < min_tokens {
            continue;
        }
        for i in 0..=(tokens.len() - min_tokens) {
            let hash = hash_tokens(&tokens[i..i + min_tokens]);
            window_map.entry(hash).or_default().push((path.clone(), i));
        }
    }
    window_map
}

/// Merges occurrences that overlap in the source code.
///
/// This avoids reporting the same duplicated block multiple times if it spans across
/// several overlapping token windows.
pub fn merge_overlapping_occurrences(mut occurrences: Vec<Occurrence>) -> Vec<Occurrence> {
    occurrences.sort_by(|a, b| a.file.cmp(&b.file).then(a.token_start.cmp(&b.token_start)));

    let mut merged: Vec<Occurrence> = Vec::new();
    for occ in occurrences {
        if let Some(last) = merged.last_mut() {
            if last.overlaps(&occ) {
                last.merge_with(occ);
                continue;
            }
        }
        merged.push(occ);
    }
    merged
}

/// Expands a match forward and backward to find the maximum duplicated range.
fn expand_match(
    tokens1: &[NormalizedToken],
    tokens2: &[NormalizedToken],
    off1: usize,
    off2: usize,
    min_tokens: usize,
    is_same_file: bool,
) -> (usize, usize, usize) {
    let mut start1 = off1;
    let mut start2 = off2;
    let mut length = min_tokens;

    let dist = if is_same_file {
        off1.abs_diff(off2)
    } else {
        usize::MAX
    };

    // Expand forward
    while start1 + length < tokens1.len()
        && start2 + length < tokens2.len()
        && tokens1[start1 + length].normalized == tokens2[start2 + length].normalized
        && (!is_same_file || length < dist)
    {
        length += 1;
    }

    // Expand backward
    while start1 > 0
        && start2 > 0
        && tokens1[start1 - 1].normalized == tokens2[start2 - 1].normalized
        && (!is_same_file || length < dist)
    {
        start1 -= 1;
        start2 -= 1;
        length += 1;
    }

    (start1, start2, length)
}

/// Calculates the precise line and column boundaries for a token range.
fn calculate_range_bounds(
    tokens: &[NormalizedToken],
    start: usize,
    length: usize,
) -> (usize, usize, usize, usize) {
    let first = &tokens[start];
    let last = &tokens[start + length - 1];
    (first.line, first.column, last.end_line, last.end_column)
}

/// Main logic for detecting clone clusters.
///
/// Iterates over matching token windows and expands them forward and backward
/// to find the maximum possible duplicated range.
pub fn detect_clusters(
    file_tokens: &FxHashMap<PathBuf, Vec<NormalizedToken>>,
    window_map: FxHashMap<[u8; 32], Vec<(PathBuf, usize)>>,
    min_tokens: usize,
    min_lines: usize,
) -> Vec<Cluster> {
    let mut window_entries: Vec<WindowEntry> = window_map.into_iter().collect();
    window_entries.sort_by(|a, b| a.0.cmp(&b.0));

    for entry in &mut window_entries {
        entry.1.sort();
    }

    let mut clusters: Vec<Cluster> = Vec::new();
    let mut occ_to_cluster: FxHashMap<(PathBuf, usize), usize> = FxHashMap::default();

    {
        let mut ctx = DetectionContext {
            file_tokens,
            clusters: &mut clusters,
            occ_to_cluster: &mut occ_to_cluster,
            processed_ranges: FxHashMap::default(),
            min_tokens,
            min_lines,
        };

        for (_hash, locations) in window_entries {
            // Skip extremely large buckets to avoid O(n^2) complexity on generated code
            if locations.len() > 1000 {
                continue;
            }

            if locations.len() < 2 {
                continue;
            }

            for i in 0..locations.len() {
                for j in (i + 1)..locations.len() {
                    ctx.process_location_pair(&locations[i], &locations[j]);
                }
            }
        }
    }

    clusters
}

/// Checks if the token at `start` is the beginning of a line.
fn starts_at_line_boundary(tokens: &[NormalizedToken], start: usize) -> bool {
    start == 0 || tokens[start - 1].line != tokens[start].line
}
