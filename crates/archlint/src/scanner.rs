use crate::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Scans the filesystem for source files based on configuration.
///
/// It respects `.gitignore` and `.archsmellignore` files and applies
/// additional exclusion filters for system directories.
pub struct FileScanner {
    project_root: PathBuf,
    scan_root: PathBuf,
    extensions: Vec<String>,
}

impl FileScanner {
    /// Create a new scanner for the given directories and extensions.
    pub fn new<P: AsRef<Path>>(project_root: P, scan_root: P, extensions: Vec<String>) -> Self {
        Self {
            project_root: project_root.as_ref().to_path_buf(),
            scan_root: scan_root.as_ref().to_path_buf(),
            extensions,
        }
    }

    /// Perform the scan and return a list of found files.
    pub fn scan(&self) -> Result<Vec<PathBuf>> {
        if self.scan_root.is_file() {
            return Ok(self.scan_single_file());
        }

        let mut files = Vec::new();
        let walker = self.create_walker()?;

        for entry in walker.build() {
            if let Some(path) = self.process_entry(entry) {
                files.push(path);
            }
        }

        Ok(files)
    }

    fn scan_single_file(&self) -> Vec<PathBuf> {
        if let Some(ext) = self.scan_root.extension() {
            if self
                .extensions
                .iter()
                .any(|e| e == ext.to_string_lossy().as_ref())
            {
                if let Ok(canonical) = self.scan_root.canonicalize() {
                    return vec![canonical];
                }
            }
        }
        Vec::new()
    }

    fn create_walker(&self) -> Result<WalkBuilder> {
        let mut walker = WalkBuilder::new(&self.scan_root);
        walker
            .standard_filters(true)
            .hidden(false)
            .add_custom_ignore_filename(".archsmellignore");

        let mut override_builder = ignore::overrides::OverrideBuilder::new(&self.project_root);

        // We no longer add config.ignore patterns here because we want to scan them
        // for dependency resolution, but filter out their smells later in the engine.

        // Exclude only hard-coded system directories that should NEVER be scanned
        let defaults = [
            "**/node_modules/**",
            "**/dist/**",
            "**/build/**",
            "**/.next/**",
            "**/coverage/**",
        ];
        for def in defaults {
            override_builder.add(&format!("!{}", def))?;
        }

        walker.overrides(override_builder.build()?);
        Ok(walker)
    }

    fn process_entry(
        &self,
        entry: std::result::Result<ignore::DirEntry, ignore::Error>,
    ) -> Option<PathBuf> {
        let entry = entry.ok()?;
        let path = entry.path();

        if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            return None;
        }

        let ext = path.extension()?;
        if self
            .extensions
            .iter()
            .any(|e| e == ext.to_string_lossy().as_ref())
        {
            return path.canonicalize().ok();
        }
        None
    }
}
