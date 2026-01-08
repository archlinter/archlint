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

impl Occurrence {
    /// Checks if this occurrence overlaps with another one in terms of source lines.
    pub fn overlaps(&self, other: &Occurrence) -> bool {
        if self.file != other.file {
            return false;
        }

        // Standard overlap check for two intervals [start, end]: max(start) <= min(end)
        self.start_line.max(other.start_line) <= self.end_line.min(other.end_line)
    }

    /// Merges another occurrence into this one, expanding the boundaries.
    pub fn merge_with(&mut self, other: Occurrence) {
        // Update start line/column if other starts earlier
        if other.start_line < self.start_line {
            self.start_line = other.start_line;
            self.start_column = other.start_column;
        } else if other.start_line == self.start_line {
            self.start_column = self.start_column.min(other.start_column);
        }

        // Update end line/column if other ends later
        if other.end_line > self.end_line {
            self.end_line = other.end_line;
            self.end_column = other.end_column;
        } else if other.end_line == self.end_line {
            self.end_column = self.end_column.max(other.end_column);
        }

        self.token_start = self.token_start.min(other.token_start);
    }
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
