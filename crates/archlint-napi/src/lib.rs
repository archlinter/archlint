#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod types;
use types::*;

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
