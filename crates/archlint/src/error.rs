use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalysisError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Config error: {0}")]
    Config(#[from] serde_yaml::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("JSON5 error: {0}")]
    Json5(#[from] json5::Error),

    #[error("Ignore error: {0}")]
    Ignore(#[from] ignore::Error),

    #[error("Notify error: {0}")]
    Notify(#[from] notify::Error),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Internal error: {0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("Path resolution error: {0}")]
    PathResolution(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Snapshot error: {0}")]
    Snapshot(#[from] crate::snapshot::SnapshotError),

    #[error("Git command error: {0}")]
    GitCommand(String),

    #[error("No project path provided")]
    NoProjectPath,
}

pub type Result<T> = std::result::Result<T, AnalysisError>;
