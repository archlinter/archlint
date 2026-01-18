use crate::Result;
use std::path::{Path, PathBuf};

pub struct GlobExpansion {
    pub base_path: PathBuf,
    pub files: Vec<PathBuf>,
}

pub fn expand_glob(pattern: &str, extensions: &[&str]) -> Result<GlobExpansion> {
    // Find the base path (part before the first wildcard)
    let base_path = extract_base_path(pattern);
    let base_path = base_path.canonicalize().unwrap_or(base_path);

    let paths = glob::glob(pattern).map_err(|e| {
        crate::error::AnalysisError::PathResolution(format!("Invalid glob pattern: {e}"))
    })?;

    let files = paths
        .flatten()
        .filter(|p| p.is_file())
        .filter(|p| {
            p.extension()
                .is_some_and(|ext| extensions.iter().any(|e| *e == ext.to_string_lossy()))
        })
        .filter_map(|p| p.canonicalize().ok())
        .collect();

    Ok(GlobExpansion { base_path, files })
}

fn extract_base_path(pattern: &str) -> PathBuf {
    let wildcard_pos = pattern.find(['*', '?', '[']);

    let path_before_wildcard = match wildcard_pos {
        Some(pos) => &pattern[..pos],
        None => pattern,
    };

    if path_before_wildcard.is_empty() {
        return PathBuf::from(".");
    }

    let path = Path::new(path_before_wildcard);
    if path_before_wildcard.ends_with('/') || path_before_wildcard.ends_with('\\') || path.is_dir()
    {
        path.to_path_buf()
    } else {
        path.parent()
            .map_or_else(|| PathBuf::from("."), std::path::Path::to_path_buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_base_path() {
        assert_eq!(extract_base_path("src/**/*.ts"), PathBuf::from("src/"));
        assert_eq!(extract_base_path("*.ts"), PathBuf::from("."));
        assert_eq!(extract_base_path("src/main.ts"), PathBuf::from("src"));
        assert_eq!(
            extract_base_path("../proj/src/**/*.ts"),
            PathBuf::from("../proj/src/")
        );
    }
}
