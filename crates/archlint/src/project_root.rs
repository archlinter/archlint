use std::path::{Path, PathBuf};

/// Detects the project root by searching upwards for project markers.
/// If no marker is found, returns the start directory.
pub fn detect_project_root(target: &Path) -> PathBuf {
    let start = if target.is_file() {
        target.parent().unwrap_or(target)
    } else {
        target
    };

    let start = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    let mut current = Some(start.as_path());

    while let Some(dir) = current {
        if is_project_root(dir) {
            return dir.to_path_buf();
        }
        current = dir.parent();
    }

    start
}

fn is_project_root(dir: &Path) -> bool {
    let markers = [
        ".git",
        "package.json",
        "pnpm-workspace.yaml",
        "tsconfig.json",
        "yarn.lock",
        "package-lock.json",
    ];

    markers.iter().any(|marker| dir.join(marker).exists())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_detect_root_with_git() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::create_dir_all(root.join("src/subdir")).unwrap();
        let file = root.join("src/subdir/file.ts");
        fs::write(&file, "").unwrap();

        assert_eq!(detect_project_root(&file), root.canonicalize().unwrap());
        assert_eq!(
            detect_project_root(&root.join("src/subdir")),
            root.canonicalize().unwrap()
        );
    }

    #[test]
    fn test_detect_root_with_package_json() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("package.json"), "{}").unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        let file = root.join("src/main.ts");
        fs::write(&file, "").unwrap();

        assert_eq!(detect_project_root(&file), root.canonicalize().unwrap());
    }

    #[test]
    fn test_detect_root_no_marker() {
        let dir = tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        fs::create_dir_all(root.join("some/path")).unwrap();
        let target = root.join("some/path");

        assert_eq!(detect_project_root(&target), target.canonicalize().unwrap());
    }
}
