use super::types::{Cluster, Occurrence, WindowEntry};
use crate::parser::tokenizer::NormalizedToken;
use rustc_hash::{FxHashMap, FxHashSet};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

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

    // Expand forward
    let max_forward = if is_same_file {
        let (lower, higher) = if off1 < off2 {
            (off1, off2)
        } else {
            (off2, off1)
        };
        higher - lower
    } else {
        usize::MAX
    };

    while start1 + length < tokens1.len()
        && start2 + length < tokens2.len()
        && tokens1[start1 + length].normalized == tokens2[start2 + length].normalized
    {
        if is_same_file && length >= max_forward {
            break;
        }
        length += 1;
    }

    // Expand backward
    while start1 > 0
        && start2 > 0
        && tokens1[start1 - 1].normalized == tokens2[start2 - 1].normalized
    {
        if is_same_file {
            let new_start1 = start1 - 1;
            let new_start2 = start2 - 1;
            let (new_lower, new_higher) = if new_start1 < new_start2 {
                (new_start1, new_start2)
            } else {
                (new_start2, new_start1)
            };
            if new_lower + length + 1 > new_higher {
                break;
            }
        }
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
    let mut sl = first.line;
    let mut sc = first.column;
    let mut el = first.end_line;
    let mut ec = first.end_column;

    for i in 0..length {
        let t = &tokens[start + i];
        if t.line < sl || (t.line == sl && t.column < sc) {
            sl = t.line;
            sc = t.column;
        }
        if t.end_line > el || (t.end_line == el && t.end_column > ec) {
            el = t.end_line;
            ec = t.end_column;
        }
    }
    (sl, sc, el, ec)
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

    for (_hash, locations) in window_entries {
        if locations.len() < 2 {
            continue;
        }

        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                let (file1, off1) = &locations[i];
                let (file2, off2) = &locations[j];

                if is_already_covered(&covered_tokens, file1, off1, file2, off2) {
                    continue;
                }

                let tokens1 = &file_tokens[file1];
                let tokens2 = &file_tokens[file2];

                let (start1, start2, length) =
                    expand_match(tokens1, tokens2, *off1, *off2, min_tokens, file1 == file2);

                if length < min_tokens {
                    continue;
                }

                if !starts_at_line_boundary(tokens1, start1)
                    || !starts_at_line_boundary(tokens2, start2)
                {
                    continue;
                }

                let (sl1, sc1, el1, ec1) = calculate_range_bounds(tokens1, start1, length);
                let (sl2, sc2, el2, ec2) = calculate_range_bounds(tokens2, start2, length);

                let lines1 = el1.saturating_sub(sl1) + 1;
                let lines2 = el2.saturating_sub(sl2) + 1;

                if lines1 < min_lines || lines2 < min_lines {
                    continue;
                }

                mark_as_covered(&mut covered_tokens, file1, start1, length);
                mark_as_covered(&mut covered_tokens, file2, start2, length);

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

                update_clusters(
                    &mut clusters,
                    &mut occ_to_cluster,
                    occ1,
                    occ2,
                    range_hash,
                    length,
                );
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

/// Updates or merges clusters with a newly discovered match.
fn update_clusters(
    clusters: &mut Vec<Cluster>,
    occ_to_cluster: &mut FxHashMap<(PathBuf, usize), usize>,
    occ1: Occurrence,
    occ2: Occurrence,
    range_hash: [u8; 32],
    length: usize,
) {
    let key1 = (occ1.file.clone(), occ1.token_start);
    let key2 = (occ2.file.clone(), occ2.token_start);

    let c1 = occ_to_cluster.get(&key1).copied();
    let c2 = occ_to_cluster.get(&key2).copied();

    let target_idx = match (c1, c2) {
        (Some(a), Some(b)) if a != b => {
            let (keep, drop) = if a < b { (a, b) } else { (b, a) };
            let dropped_occurrences = std::mem::take(&mut clusters[drop].occurrences);
            for occ in dropped_occurrences {
                let k = (occ.file.clone(), occ.token_start);
                if !clusters[keep]
                    .occurrences
                    .iter()
                    .any(|o| o.file == occ.file && o.token_start == occ.token_start)
                {
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
            return;
        }
    };

    let target = &mut clusters[target_idx];
    if !target
        .occurrences
        .iter()
        .any(|o| o.file == occ1.file && o.token_start == occ1.token_start)
    {
        target.occurrences.push(occ1);
        occ_to_cluster.insert(key1, target_idx);
    }
    if !target
        .occurrences
        .iter()
        .any(|o| o.file == occ2.file && o.token_start == occ2.token_start)
    {
        target.occurrences.push(occ2);
        occ_to_cluster.insert(key2, target_idx);
    }
}
