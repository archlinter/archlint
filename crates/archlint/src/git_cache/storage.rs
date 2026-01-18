use crate::Result;
use bincode::{
    config,
    serde::{decode_from_slice, encode_to_vec},
};
use redb::{Database, ReadableDatabase, TableDefinition};
use serde::{Deserialize, Serialize};
use std::path::Path;

const COMMITS_TABLE: TableDefinition<&[u8; 20], &[u8]> = TableDefinition::new("commits");

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitData {
    pub files_changed: Vec<String>,
}

pub struct GitStorage {
    db: Database,
}

impl GitStorage {
    pub fn open(path: &Path) -> Result<Self> {
        let db = Database::create(path).map_err(|e| {
            crate::AnalysisError::Storage(format!("Failed to open git cache db: {e}"))
        })?;

        // Ensure table exists
        let write_txn = db.begin_write().map_err(|e| {
            crate::AnalysisError::Storage(format!("Failed to begin write txn: {e}"))
        })?;
        {
            let _ = write_txn
                .open_table(COMMITS_TABLE)
                .map_err(|e| crate::AnalysisError::Storage(format!("Failed to open table: {e}")))?;
        }
        write_txn
            .commit()
            .map_err(|e| crate::AnalysisError::Storage(format!("Failed to commit txn: {e}")))?;

        Ok(Self { db })
    }

    pub fn get_commit_data(&self, oid: &[u8; 20]) -> Result<Option<CommitData>> {
        let read_txn = self
            .db
            .begin_read()
            .map_err(|e| crate::AnalysisError::Storage(format!("Failed to begin read txn: {e}")))?;
        let table = read_txn
            .open_table(COMMITS_TABLE)
            .map_err(|e| crate::AnalysisError::Storage(format!("Failed to open table: {e}")))?;

        let value = table
            .get(oid)
            .map_err(|e| crate::AnalysisError::Storage(format!("Failed to get from table: {e}")))?;

        if let Some(bytes) = value {
            let bytes_value: &[u8] = bytes.value();
            let (data, _): (CommitData, usize) = decode_from_slice(bytes_value, config::standard())
                .map_err(|e| {
                    crate::AnalysisError::Storage(format!("Failed to deserialize: {e}"))
                })?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    pub fn insert_commit_data(&self, oid: &[u8; 20], data: &CommitData) -> Result<()> {
        let write_txn = self.db.begin_write().map_err(|e| {
            crate::AnalysisError::Storage(format!("Failed to begin write txn: {e}"))
        })?;
        {
            let mut table = write_txn
                .open_table(COMMITS_TABLE)
                .map_err(|e| crate::AnalysisError::Storage(format!("Failed to open table: {e}")))?;
            let bytes = encode_to_vec(data, config::standard())
                .map_err(|e| crate::AnalysisError::Storage(format!("Failed to serialize: {e}")))?;
            table
                .insert(oid, bytes.as_slice())
                .map_err(|e| crate::AnalysisError::Storage(format!("Failed to insert: {e}")))?;
        }
        write_txn
            .commit()
            .map_err(|e| crate::AnalysisError::Storage(format!("Failed to commit txn: {e}")))?;
        Ok(())
    }
}
