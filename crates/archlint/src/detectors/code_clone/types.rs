use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Occurrence {
    pub file: PathBuf,
    pub token_start: usize,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Debug, Clone)]
pub struct Cluster {
    pub hash: [u8; 32],
    pub token_count: usize,
    pub occurrences: Vec<Occurrence>,
}

pub type WindowEntry = ([u8; 32], Vec<(PathBuf, usize)>);
