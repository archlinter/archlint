use crate::config::Config;
use crate::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub struct FileScanner {
    root: PathBuf,
    extensions: Vec<String>,
}

impl FileScanner {
    pub fn new<P: AsRef<Path>>(root: P, extensions: Vec<String>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            extensions,
        }
    }

    pub fn scan(&self, config: &Config) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        let mut walker = WalkBuilder::new(&self.root);
        walker
            .standard_filters(true)
            .hidden(false)
            .add_custom_ignore_filename(".archsmellignore");

        // Add custom ignore patterns from config
        let mut override_builder = ignore::overrides::OverrideBuilder::new(&self.root);
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
