use super::types::{Cluster, Occurrence, WindowEntry};
use crate::parser::tokenizer::NormalizedToken;
use rustc_hash::{FxHashMap, FxHashSet};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

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
                let mut length = min_tokens;

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
                    && tokens1[start1 + length].normalized == tokens2[start2 + length].normalized
                {
                    if file1 == file2 && length >= max_length_for_same_file {
                        break;
                    }
                    length += 1;
                }

                // Expand backward
                while start1 > 0
                    && start2 > 0
                    && tokens1[start1 - 1].normalized == tokens2[start2 - 1].normalized
                {
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

                if length < min_tokens {
                    continue;
                }

                let first1 = &tokens1[start1];
                let first2 = &tokens2[start2];
                let starts_at_line1 = start1 == 0 || tokens1[start1 - 1].line != first1.line;
                let starts_at_line2 = start2 == 0 || tokens2[start2 - 1].line != first2.line;
                if !starts_at_line1 || !starts_at_line2 {
                    continue;
                }

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

                if lines1 < min_lines || lines2 < min_lines {
                    continue;
                }

                for idx in start1..end1 {
                    covered_tokens.entry(file1.clone()).or_default().insert(idx);
                }
                for idx in start2..end2 {
                    covered_tokens.entry(file2.clone()).or_default().insert(idx);
                }

                let range_hash = hash_tokens(&tokens1[start1..end1]);

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

                let target_cluster = match (c1, c2) {
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

    clusters
}
