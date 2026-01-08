use std::path::PathBuf;

/// Represents a single occurrence of a duplicated code block.
#[derive(Debug, Clone)]
pub struct Occurrence {
    /// Absolute path to the file containing the clone.
    pub file: PathBuf,
    /// Starting index of the clone in the token stream.
    pub token_start: usize,
    /// Starting line in the source file.
    pub start_line: usize,
    /// Starting column in the source file.
    pub start_column: usize,
    /// Ending line in the source file.
    pub end_line: usize,
    /// Ending column in the source file.
    pub end_column: usize,
}

/// Represents a set of duplicated code blocks (a clone class).
#[derive(Debug, Clone)]
pub struct Cluster {
    /// Hash of the normalized tokens in this clone class.
    pub hash: [u8; 32],
    /// Number of tokens in the duplicated block.
    pub token_count: usize,
    /// All detected occurrences of this duplicate.
    pub occurrences: Vec<Occurrence>,
}

/// Type alias for a entry in the window map: (Token Hash, Vec<(File, Offset)>).
pub type WindowEntry = ([u8; 32], Vec<(PathBuf, usize)>);
