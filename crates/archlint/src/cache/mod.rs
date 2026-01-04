use crate::config::Config;
use crate::parser::ParsedFile;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
pub mod hash;
use self::hash::config_hash;

#[derive(Serialize, Deserialize)]
pub struct CacheMeta {
    pub version: String,
    pub app_version: String,
    pub config_hash: String,
    pub git_head: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub content_hash: String,
    pub parsed: ParsedFile,
}

#[derive(Serialize, Deserialize)]
pub struct CacheData {
    pub meta: CacheMeta,
    pub entries: HashMap<PathBuf, FileEntry>,
    #[serde(default)]
    pub churn_map: HashMap<PathBuf, usize>,
}

pub struct AnalysisCache {
    cache_file: PathBuf,
    data: CacheData,
    is_dirty: bool,
}

impl AnalysisCache {
    const CACHE_DIR: &'static str = ".archlint-cache";
    const CACHE_FILE: &'static str = "cache.json";
    const VERSION: &'static str = "2"; // v2: Added type reference tracking in interfaces/type aliases
    const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

    fn resolve_cache_dir(project_root: &Path) -> PathBuf {
        let node_modules = project_root.join("node_modules");
        if node_modules.exists() && node_modules.is_dir() {
            node_modules.join(".cache").join("archlint")
        } else {
            project_root.join(Self::CACHE_DIR)
        }
    }

    pub fn load(project_root: &Path, config: &Config) -> Result<Self> {
        let cache_dir = Self::resolve_cache_dir(project_root);
        let cache_file = cache_dir.join(Self::CACHE_FILE);

        let data = if cache_file.exists() {
            let content = fs::read_to_string(&cache_file)?;
            match serde_json::from_str::<CacheData>(&content) {
                Ok(mut data)
                    if data.meta.version == Self::VERSION
                        && data.meta.app_version == Self::APP_VERSION
                        && data.meta.config_hash == config_hash(config) =>
                {
                    // Only check git head if git is enabled
                    let current_head = if config.enable_git {
                        crate::cache::hash::get_git_head(project_root)
                    } else {
                        None
                    };
                    if data.meta.git_head != current_head {
                        data.churn_map.clear();
                        data.meta.git_head = current_head;
                    }
                    data
                }
                _ => Self::empty_data(project_root, config),
            }
        } else {
            Self::empty_data(project_root, config)
        };

        Ok(Self {
            cache_file,
            data,
            is_dirty: false,
        })
    }

    fn empty_data(project_root: &Path, config: &Config) -> CacheData {
        CacheData {
            meta: CacheMeta {
                version: Self::VERSION.to_string(),
                app_version: Self::APP_VERSION.to_string(),
                config_hash: config_hash(config),
                git_head: if config.enable_git {
                    crate::cache::hash::get_git_head(project_root)
                } else {
                    None
                },
                created_at: chrono::Utc::now(),
            },
            entries: HashMap::new(),
            churn_map: HashMap::new(),
        }
    }

    pub fn get(&self, path: &Path, content_hash: &str) -> Option<&ParsedFile> {
        self.data.entries.get(path).and_then(|entry| {
            if entry.content_hash == content_hash {
                Some(&entry.parsed)
            } else {
                None
            }
        })
    }

    pub fn insert(&mut self, path: PathBuf, content_hash: String, parsed: ParsedFile) {
        self.data.entries.insert(
            path,
            FileEntry {
                content_hash,
                parsed,
            },
        );
        self.is_dirty = true;
    }

    pub fn get_churn_map(&self) -> Option<&HashMap<PathBuf, usize>> {
        if self.data.churn_map.is_empty() {
            None
        } else {
            Some(&self.data.churn_map)
        }
    }

    pub fn insert_churn_map(&mut self, churn_map: HashMap<PathBuf, usize>) {
        self.data.churn_map = churn_map;
        self.is_dirty = true;
    }

    pub fn save(&self) -> Result<()> {
        if !self.is_dirty {
            return Ok(());
        }

        if let Some(parent) = self.cache_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.cache_file, content)?;
        Ok(())
    }

    pub fn clear(project_root: &Path) -> Result<()> {
        let _ = crate::git_cache::GitHistoryCache::clear(project_root);
        let locations = [
            project_root.join(Self::CACHE_DIR),
            project_root.join("node_modules/.cache/archlint"),
        ];

        for cache_dir in locations {
            if cache_dir.exists() {
                fs::remove_dir_all(cache_dir)?;
            }
        }
        Ok(())
    }
}
