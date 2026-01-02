use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Information about an analyzed file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    /// Absolute path
    pub path: PathBuf,

    /// Relative path from project root
    pub relative_path: PathBuf,

    /// Import statements
    pub imports: Vec<ImportInfo>,

    /// Export statements
    pub exports: Vec<ExportInfo>,

    /// File metrics
    pub metrics: FileMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportInfo {
    /// Import source (relative or package name)
    pub source: String,

    /// Imported names
    pub names: Vec<String>,

    /// Line number
    pub line: usize,

    /// Is default import
    pub is_default: bool,

    /// Is namespace import (import * as)
    pub is_namespace: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportInfo {
    /// Exported name
    pub name: String,

    /// Export kind
    pub kind: ExportKind,

    /// Is default export
    pub is_default: bool,

    /// Re-export source (if re-exporting)
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportKind {
    Function,
    Class,
    Variable,
    Type,
    Interface,
    Enum,
    Reexport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMetrics {
    /// Lines of code
    pub lines: usize,

    /// Max cyclomatic complexity in file
    pub complexity: Option<usize>,

    /// Number of files importing this file
    pub fan_in: usize,

    /// Number of files this file imports
    pub fan_out: usize,
}
