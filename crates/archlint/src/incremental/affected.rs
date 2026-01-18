use super::state::IncrementalState;
use std::collections::{HashSet, VecDeque};
use std::path::PathBuf;

impl IncrementalState {
    /// Get all files affected by changes (transitive importers)
    #[must_use]
    pub fn get_affected_files(&self, changed: &[PathBuf]) -> HashSet<PathBuf> {
        let mut affected = HashSet::new();
        let mut queue: VecDeque<PathBuf> = changed.iter().cloned().collect();

        while let Some(file) = queue.pop_front() {
            if affected.insert(file.clone()) {
                // Add files that import this file (reverse deps)
                if let Some(importers) = self.reverse_deps.get(&file) {
                    for importer in importers {
                        if !affected.contains(importer) {
                            queue.push_back(importer.clone());
                        }
                    }
                }
            }
        }

        affected
    }
}

#[cfg(test)]
mod tests {
    use crate::incremental::state::IncrementalState;
    use std::path::PathBuf;

    #[test]
    fn test_get_affected_files_direct() {
        let mut state = IncrementalState::new(PathBuf::from("/"), "hash".into());
        let a = PathBuf::from("/a.ts");
        let b = PathBuf::from("/b.ts");

        // b imports a
        state
            .reverse_deps
            .entry(a.clone())
            .or_default()
            .insert(b.clone());

        let changed = vec![a.clone()];
        let affected = state.get_affected_files(&changed);

        assert_eq!(affected.len(), 2);
        assert!(affected.contains(&a));
        assert!(affected.contains(&b));
    }

    #[test]
    fn test_get_affected_files_transitive() {
        let mut state = IncrementalState::new(PathBuf::from("/"), "hash".into());
        let a = PathBuf::from("/a.ts");
        let b = PathBuf::from("/b.ts");
        let c = PathBuf::from("/c.ts");

        // b imports a, c imports b
        state
            .reverse_deps
            .entry(a.clone())
            .or_default()
            .insert(b.clone());
        state
            .reverse_deps
            .entry(b.clone())
            .or_default()
            .insert(c.clone());

        let changed = vec![a.clone()];
        let affected = state.get_affected_files(&changed);

        assert_eq!(affected.len(), 3);
        assert!(affected.contains(&a));
        assert!(affected.contains(&b));
        assert!(affected.contains(&c));
    }

    #[test]
    fn test_get_affected_files_cycle() {
        let mut state = IncrementalState::new(PathBuf::from("/"), "hash".into());
        let a = PathBuf::from("/a.ts");
        let b = PathBuf::from("/b.ts");

        // a imports b, b imports a
        state
            .reverse_deps
            .entry(a.clone())
            .or_default()
            .insert(b.clone());
        state
            .reverse_deps
            .entry(b.clone())
            .or_default()
            .insert(a.clone());

        let changed = vec![a.clone()];
        let affected = state.get_affected_files(&changed);

        assert_eq!(affected.len(), 2);
        assert!(affected.contains(&a));
        assert!(affected.contains(&b));
    }
}
