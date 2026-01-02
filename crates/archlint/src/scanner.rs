use crate::config::Config;
use crate::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub struct FileScanner {
    project_root: PathBuf,
    scan_root: PathBuf,
    extensions: Vec<String>,
}

impl FileScanner {
    pub fn new<P: AsRef<Path>>(project_root: P, scan_root: P, extensions: Vec<String>) -> Self {
        Self {
            project_root: project_root.as_ref().to_path_buf(),
            scan_root: scan_root.as_ref().to_path_buf(),
            extensions,
        }
    }

    pub fn scan(&self, config: &Config) -> Result<Vec<PathBuf>> {
        if self.scan_root.is_file() {
            if let Some(ext) = self.scan_root.extension() {
                if self
                    .extensions
                    .iter()
                    .any(|e| e == ext.to_string_lossy().as_ref())
                {
                    if let Ok(canonical) = self.scan_root.canonicalize() {
                        return Ok(vec![canonical]);
                    }
                }
            }
            return Ok(Vec::new());
        }

        let mut files = Vec::new();

        let mut walker = WalkBuilder::new(&self.scan_root);
        walker
            .standard_filters(true)
            .hidden(false)
            .add_custom_ignore_filename(".archsmellignore");

        // Add custom ignore patterns from config
        let mut override_builder = ignore::overrides::OverrideBuilder::new(&self.project_root);
        for pattern in &config.ignore {
            // ignore crate uses ! for negation, but here we want to exclude
            // so we add patterns as "!pattern" to exclude them
            override_builder.add(&format!("!{}", pattern))?;
        }

        // Exclude standard directories
        override_builder.add("!**/node_modules/**")?;
        override_builder.add("!**/dist/**")?;
        override_builder.add("!**/build/**")?;
        override_builder.add("!**/.next/**")?;
        override_builder.add("!**/coverage/**")?;

        walker.overrides(override_builder.build()?);

        for entry in walker.build() {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path();

            if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                continue;
            }

            if let Some(ext) = path.extension() {
                if self
                    .extensions
                    .iter()
                    .any(|e| e == ext.to_string_lossy().as_ref())
                {
                    // Canonicalize path to ensure consistency
                    if let Ok(canonical) = path.canonicalize() {
                        files.push(canonical);
                    }
                }
            }
        }

        Ok(files)
    }
}
