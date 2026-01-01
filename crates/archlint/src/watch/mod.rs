use crate::Result;
use notify::{RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

pub mod diff;
pub mod runner;
pub mod ui;

pub struct WatchConfig {
    pub debounce_ms: u64,
    pub ignore_patterns: Vec<String>,
    pub clear_screen: bool,
    pub extensions: Vec<String>,
}

pub struct FileWatcher {
    config: WatchConfig,
    project_path: PathBuf,
}

impl FileWatcher {
    pub fn new(path: PathBuf, config: WatchConfig) -> Self {
        Self {
            config,
            project_path: path,
        }
    }

    pub fn watch<F>(&self, mut on_change: F) -> Result<()>
    where
        F: FnMut(Vec<PathBuf>) -> Result<()>,
    {
        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        })?;

        watcher.watch(&self.project_path, RecursiveMode::Recursive)?;

        let mut pending_changes: Vec<PathBuf> = Vec::new();
        let mut last_change = Instant::now();

        loop {
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(event) => {
                    for path in event.paths {
                        if self.should_process(&path) {
                            pending_changes.push(path);
                            last_change = Instant::now();
                        }
                    }
                }
                Err(_) => {
                    // Timeout - check if debounce period passed
                    if !pending_changes.is_empty()
                        && last_change.elapsed() > Duration::from_millis(self.config.debounce_ms)
                    {
                        let changes = std::mem::take(&mut pending_changes);
                        on_change(changes)?;
                    }
                }
            }
        }
    }

    fn should_process(&self, path: &Path) -> bool {
        // Check file extension
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !self.config.extensions.iter().any(|e| e == ext) {
            return false;
        }

        // Check ignore patterns
        for pattern in &self.config.ignore_patterns {
            if path.to_string_lossy().contains(pattern) {
                return false;
            }
        }

        true
    }
}
