#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod types;
use types::{
    JsConfig, JsDetectorInfo, JsIncrementalResult, JsScanOptions, JsScanResult, JsStateStats,
};

#[napi]
pub async fn scan(path: String, options: Option<JsScanOptions>) -> Result<JsScanResult> {
    let opts: archlint::ScanOptions = options.map(Into::into).unwrap_or_default();

    // Run in blocking task to not block JS event loop
    let result = tokio::task::spawn_blocking(move || archlint::scan(path, opts))
        .await
        .map_err(|e| Error::from_reason(format!("Task execution failed: {}", e)))?
        .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(result.into())
}

#[napi]
pub fn scan_sync(path: String, options: Option<JsScanOptions>) -> Result<JsScanResult> {
    let opts: archlint::ScanOptions = options.map(Into::into).unwrap_or_default();
    let result = archlint::scan(path, opts).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(result.into())
}

#[napi]
pub fn load_config(path: Option<String>) -> Result<JsConfig> {
    let config =
        archlint::load_config(path.as_deref()).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(config.into())
}

#[napi]
pub fn get_detectors() -> Vec<JsDetectorInfo> {
    archlint::get_detectors()
        .into_iter()
        .map(Into::into)
        .collect()
}

#[napi]
pub fn clear_cache(path: String) -> Result<()> {
    archlint::clear_cache(path).map_err(|e| Error::from_reason(e.to_string()))
}

#[napi]
pub struct ArchlintAnalyzer {
    inner: Arc<Mutex<archlint::Analyzer>>,
}

#[napi]
impl ArchlintAnalyzer {
    #[napi(constructor)]
    pub fn new(path: String, options: Option<JsScanOptions>) -> Result<Self> {
        let opts = options.map(Into::into).unwrap_or_default();
        let analyzer =
            archlint::Analyzer::new(path, opts).map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Self {
            inner: Arc::new(Mutex::new(analyzer)),
        })
    }

    #[napi]
    pub async fn scan(&self) -> Result<JsScanResult> {
        let inner = self.inner.clone();
        tokio::task::spawn_blocking(move || {
            let mut analyzer = inner.lock().unwrap();
            analyzer.scan().map(JsScanResult::from)
        })
        .await
        .map_err(|e| Error::from_reason(format!("Task execution failed: {}", e)))?
        .map_err(|e: archlint::AnalysisError| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub async fn scan_incremental(
        &self,
        changed_files: Vec<String>,
    ) -> Result<JsIncrementalResult> {
        let inner = self.inner.clone();
        let paths: Vec<PathBuf> = changed_files.into_iter().map(PathBuf::from).collect();

        tokio::task::spawn_blocking(move || {
            let mut analyzer = inner.lock().unwrap();
            analyzer
                .scan_incremental(paths)
                .map(JsIncrementalResult::from)
        })
        .await
        .map_err(|e| Error::from_reason(format!("Task execution failed: {}", e)))?
        .map_err(|e: archlint::AnalysisError| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn scan_sync(&self) -> Result<JsScanResult> {
        let mut analyzer = self.inner.lock().unwrap();
        analyzer
            .scan()
            .map(JsScanResult::from)
            .map_err(|e: archlint::AnalysisError| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn scan_incremental_sync(&self, changed_files: Vec<String>) -> Result<JsIncrementalResult> {
        let paths: Vec<PathBuf> = changed_files.into_iter().map(PathBuf::from).collect();
        let mut analyzer = self.inner.lock().unwrap();
        analyzer
            .scan_incremental(paths)
            .map(JsIncrementalResult::from)
            .map_err(|e: archlint::AnalysisError| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn scan_incremental_with_overlay_sync(
        &self,
        changed_files: Vec<String>,
        overlays: HashMap<String, String>,
    ) -> Result<JsIncrementalResult> {
        let paths: Vec<PathBuf> = changed_files.into_iter().map(PathBuf::from).collect();
        let overlay_map: HashMap<PathBuf, String> = overlays
            .into_iter()
            .map(|(k, v)| (PathBuf::from(k), v))
            .collect();

        let mut analyzer = self.inner.lock().unwrap();
        analyzer
            .scan_incremental_with_overlays(paths, overlay_map)
            .map(JsIncrementalResult::from)
            .map_err(|e: archlint::AnalysisError| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub async fn scan_incremental_with_overlay(
        &self,
        changed_files: Vec<String>,
        overlays: HashMap<String, String>,
    ) -> Result<JsIncrementalResult> {
        let inner = self.inner.clone();
        let paths: Vec<PathBuf> = changed_files.into_iter().map(PathBuf::from).collect();
        let overlay_map: HashMap<PathBuf, String> = overlays
            .into_iter()
            .map(|(k, v)| (PathBuf::from(k), v))
            .collect();

        tokio::task::spawn_blocking(move || {
            let mut analyzer = inner.lock().unwrap();
            analyzer
                .scan_incremental_with_overlays(paths, overlay_map)
                .map(JsIncrementalResult::from)
        })
        .await
        .map_err(|e| Error::from_reason(format!("Task execution failed: {}", e)))?
        .map_err(|e: archlint::AnalysisError| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn invalidate(&self, files: Vec<String>) {
        let paths: Vec<PathBuf> = files.into_iter().map(PathBuf::from).collect();
        let mut analyzer = self.inner.lock().unwrap();
        analyzer.invalidate(&paths);
    }

    #[napi]
    pub async fn rescan(&self) -> Result<JsScanResult> {
        let inner = self.inner.clone();
        tokio::task::spawn_blocking(move || {
            let mut analyzer = inner.lock().unwrap();
            analyzer.rescan().map(JsScanResult::from)
        })
        .await
        .map_err(|e| Error::from_reason(format!("Task execution failed: {}", e)))?
        .map_err(|e: archlint::AnalysisError| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn rescan_sync(&self) -> Result<JsScanResult> {
        let mut analyzer = self.inner.lock().unwrap();
        analyzer
            .rescan()
            .map(JsScanResult::from)
            .map_err(|e: archlint::AnalysisError| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn get_affected_files(&self, changed_files: Vec<String>) -> Vec<String> {
        let paths: Vec<PathBuf> = changed_files.into_iter().map(PathBuf::from).collect();
        let analyzer = self.inner.lock().unwrap();
        analyzer
            .get_affected_files(&paths)
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }

    #[napi]
    pub fn get_state_stats(&self) -> JsStateStats {
        let analyzer = self.inner.lock().unwrap();
        analyzer.get_state_stats().into()
    }
}
