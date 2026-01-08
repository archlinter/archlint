use super::types::{Cluster, Occurrence, WindowEntry};
use crate::parser::tokenizer::NormalizedToken;
use rustc_hash::{FxHashMap, FxHashSet};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// Context for the clone detection process.
struct DetectionContext<'a> {
    file_tokens: &'a FxHashMap<PathBuf, Vec<NormalizedToken>>,
    covered: &'a mut FxHashMap<PathBuf, FxHashSet<usize>>,
    clusters: &'a mut Vec<Cluster>,
    occ_to_cluster: &'a mut FxHashMap<(PathBuf, usize), usize>,
    min_tokens: usize,
    min_lines: usize,
}

impl DetectionContext<'_> {
    /// Processes a pair of token window locations to detect and record clones.
    fn process_location_pair(&mut self, loc1: &(PathBuf, usize), loc2: &(PathBuf, usize)) {
        let (file1, off1) = loc1;
        let (file2, off2) = loc2;

        if is_already_covered(self.covered, file1, off1, file2, off2) {
            return;
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

        mark_as_covered(self.covered, file1, start1, length);
        mark_as_covered(self.covered, file2, start2, length);

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
                self.clusters[keep].hash = range_hash;
                self.clusters[keep].token_count = length;
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
        for i in 0..=(tokens.len().saturating_sub(min_tokens)) {
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
            if last.file == occ.file {
                // Only merge if they truly overlap in source lines
                let overlaps = (occ.start_line >= last.start_line
                    && occ.start_line <= last.end_line)
                    || (occ.end_line >= last.start_line && occ.end_line <= last.end_line);

                if overlaps {
                    // Update start line/column if current occurrence starts earlier
                    if occ.start_line < last.start_line {
                        last.start_line = occ.start_line;
                        last.start_column = occ.start_column;
                    } else if occ.start_line == last.start_line {
                        last.start_column = last.start_column.min(occ.start_column);
                    }

                    // Update end line/column if current occurrence ends later
                    if occ.end_line > last.end_line {
                        last.end_line = occ.end_line;
                        last.end_column = occ.end_column;
                    } else if occ.end_line == last.end_line {
                        last.end_column = last.end_column.max(occ.end_column);
                    }

                    last.token_start = last.token_start.min(occ.token_start);
                    continue;
                }
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
    let mut covered_tokens: FxHashMap<PathBuf, FxHashSet<usize>> = FxHashMap::default();

    {
        let mut ctx = DetectionContext {
            file_tokens,
            covered: &mut covered_tokens,
            clusters: &mut clusters,
            occ_to_cluster: &mut occ_to_cluster,
            min_tokens,
            min_lines,
        };

        for (_hash, locations) in window_entries {
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

/// Checks if the current token offsets are already part of a detected clone.
fn is_already_covered(
    covered: &FxHashMap<PathBuf, FxHashSet<usize>>,
    file1: &Path,
    off1: &usize,
    file2: &Path,
    off2: &usize,
) -> bool {
    file1 != file2
        && covered.get(file1).is_some_and(|s| s.contains(off1))
        && covered.get(file2).is_some_and(|s| s.contains(off2))
}

/// Checks if the token at `start` is the beginning of a line.
fn starts_at_line_boundary(tokens: &[NormalizedToken], start: usize) -> bool {
    start == 0 || tokens[start - 1].line != tokens[start].line
}

/// Marks a range of tokens as covered to avoid redundant detection.
fn mark_as_covered(
    covered: &mut FxHashMap<PathBuf, FxHashSet<usize>>,
    file: &Path,
    start: usize,
    length: usize,
) {
    let set = covered.entry(file.to_path_buf()).or_default();
    for idx in start..start + length {
        set.insert(idx);
    }
}
