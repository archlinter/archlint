use crate::snapshot::types::{Snapshot, SnapshotError};
use std::fs;
use std::path::Path;

/// Write snapshot to JSON file
///
/// Validated before writing so an unreadable snapshot is never produced: `diff`
/// validates on read, and a file that only fails there is far harder to diagnose.
pub fn write_snapshot(snapshot: &Snapshot, path: &Path) -> Result<(), SnapshotError> {
    snapshot.validate()?;
    let json = serde_json::to_string_pretty(snapshot)?;
    fs::write(path, json)?;
    Ok(())
}

/// Read snapshot from JSON file
pub fn read_snapshot(path: &Path) -> Result<Snapshot, SnapshotError> {
    let content = fs::read_to_string(path)?;
    let snapshot: Snapshot = serde_json::from_str(&content)?;
    snapshot.validate()?;
    Ok(snapshot)
}

/// Load snapshot from file (detects format if needed)
pub fn load_snapshot(path: &Path) -> Result<Snapshot, SnapshotError> {
    let content = fs::read(path)?;

    // For now, assume JSON. In future, we can check magic bytes for binary format.
    let snapshot: Snapshot = serde_json::from_slice(&content)?;
    snapshot.validate()?;

    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::types::{SnapshotSmell, SnapshotSummary, SCHEMA_VERSION};
    use std::collections::HashMap;

    fn snapshot_with_ids(ids: &[&str]) -> Snapshot {
        Snapshot {
            schema_version: SCHEMA_VERSION,
            archlint_version: "0.0.0".to_string(),
            generated_at: "2026-01-05T12:00:00Z".to_string(),
            commit: None,
            smells: ids
                .iter()
                .map(|id| SnapshotSmell {
                    id: (*id).to_string(),
                    smell_type: "PackageCycle".to_string(),
                    severity: "High".to_string(),
                    files: Vec::new(),
                    metrics: HashMap::new(),
                    details: None,
                    locations: Vec::new(),
                })
                .collect(),
            summary: SnapshotSummary::default(),
            grade: "B".to_string(),
        }
    }

    #[test]
    fn test_write_snapshot_rejects_duplicate_ids() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("snapshot.json");

        // Writing must fail here rather than leaving a file that `archlint diff`
        // later refuses to read.
        let result = write_snapshot(&snapshot_with_ids(&["dup:1", "dup:1"]), &path);

        assert!(
            matches!(result, Err(SnapshotError::DuplicateId(_))),
            "expected a duplicate-ID error, got {result:?}"
        );
        assert!(!path.exists(), "no snapshot file should be left behind");
    }

    #[test]
    fn test_write_snapshot_accepts_unique_ids() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("snapshot.json");

        write_snapshot(&snapshot_with_ids(&["a:1", "b:2"]), &path).unwrap();

        assert_eq!(read_snapshot(&path).unwrap().smells.len(), 2);
    }
}
