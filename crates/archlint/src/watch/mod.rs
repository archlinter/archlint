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
                    Self::process_event(event, &mut pending_changes, &mut last_change, self);
                }
                Err(_) => {
                    if Self::should_trigger_debounce(
                        &pending_changes,
                        last_change,
                        self.config.debounce_ms,
                    ) {
                        let changes = std::mem::take(&mut pending_changes);
                        on_change(changes)?;
                    }
                }
            }
        }
    }

    fn should_process(&self, path: &Path) -> bool {
        Self::has_valid_extension(path, &self.config.extensions)
            && !Self::matches_ignore_pattern(path, &self.config.ignore_patterns)
    }

    fn has_valid_extension(path: &Path, extensions: &[String]) -> bool {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        extensions.iter().any(|e| e == ext)
    }

    fn matches_ignore_pattern(path: &Path, patterns: &[String]) -> bool {
        let path_str = path.to_string_lossy();
        patterns.iter().any(|pattern| path_str.contains(pattern))
    }

    fn process_event(
        event: notify::Event,
        pending_changes: &mut Vec<PathBuf>,
        last_change: &mut Instant,
        watcher: &FileWatcher,
    ) {
        for path in event.paths {
            if watcher.should_process(&path) {
                pending_changes.push(path);
                *last_change = Instant::now();
            }
        }
    }

    fn should_trigger_debounce(
        pending_changes: &[PathBuf],
        last_change: Instant,
        debounce_ms: u64,
    ) -> bool {
        !pending_changes.is_empty() && last_change.elapsed() > Duration::from_millis(debounce_ms)
    }
}
