use log::info;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// 
/// TODO: Implement more specific structs
#[derive(Debug, Serialize, Deserialize)]
pub struct PypiData {
    pub info: serde_json::value::Value,
    pub last_serial: i32,
    pub releases: serde_json::value::Value,
    pub urls: Vec<serde_json::value::Value>,
    pub vulnerabilities: Vec<serde_json::value::Value>,
}

/// Implements Warehouse Pypi API call & JSON conversion
pub fn request_package_info<T>(package_name: T, package_index: T) -> Result<PypiData, reqwest::Error>
where
    T: ToString + Display,
{
    let path = format!("{}/pypi/{}/json", package_index, package_name);

    info!("Requesting data from {}", path);
    let resp: reqwest::blocking::Response = reqwest::blocking::get(path)?;

    let decoded_json: PypiData = resp.json()?;

    Ok(decoded_json)
}
