use crate::snapshot::types::{Snapshot, SnapshotError};
use std::fs;
use std::path::Path;

/// Write snapshot to JSON file
pub fn write_snapshot(snapshot: &Snapshot, path: &Path) -> Result<(), SnapshotError> {
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
