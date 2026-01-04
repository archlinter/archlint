use crate::config::Config;
use crate::Result;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub fn file_content_hash(path: &Path) -> Result<String> {
    let content = fs::read(path)?;
    Ok(content_hash_bytes(&content))
}

pub fn content_hash(content: &str) -> String {
    content_hash_bytes(content.as_bytes())
}

fn content_hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub fn config_hash(config: &Config) -> String {
    let serialized = serde_json::to_string(config).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn get_git_head(project_root: &Path) -> Option<String> {
    let repo = git2::Repository::discover(project_root).ok()?;
    let head = repo.head().ok()?;
    head.target().map(|oid| oid.to_string())
}
