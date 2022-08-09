//! Warehouse PyPI API Implementation

use log::info;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Set of Information describing a Python package hosted on a Warehouse instance
/// for exact details of what is contained go to https://warehouse.pypa.io/api-reference/json.html#project
#[derive(Debug, Serialize, Deserialize)]
pub struct PyPIData {
    /// Contains data such as Package Name, Author,
    pub info: serde_json::value::Value,
    pub last_serial: i32,
    /// List of releases containing data such as
    pub releases: serde_json::value::Value,
    /// Link and related data to sdist & bdist_wheel
    pub urls: Vec<serde_json::value::Value>,
    /// Vector of known vulnerabilities of the package
    pub vulnerabilities: Vec<serde_json::value::Value>,
}

/// Implements Warehouse PyPI API call & JSON conversion
///
/// # Example
/// ```
/// use rust-pip::PyPI::request_package_info;
///
/// let data = request_package_info("numpy", "https://pypi.org/").unwrap();
/// assert_eq!(data.info.get("license").unwrap(), "BSD");
/// ```
pub fn request_package_info<T>(
    package_name: T,
    package_index: T,
) -> Result<PyPIData, reqwest::Error>
where
    T: ToString + Display,
{
    let path = format!("{}/pypi/{}/json", package_index, package_name);

    info!("Requesting data from {}", path);
    let resp: reqwest::blocking::Response = reqwest::blocking::get(path)?;

    let decoded_json: PyPIData = resp.json()?;

    Ok(decoded_json)
}

#[cfg(test)]
mod tests {
    use crate::pypi::request_package_info;

    #[test]
    fn check_numpy_licence() {
        let data = request_package_info("numpy", "https://pypi.org/").unwrap();

        assert_eq!(data.info.get("license").unwrap(), "BSD");
    }

    #[test]
    fn check_pytorch_name() {
        let data = request_package_info("pytorch", "https://pypi.org/").unwrap();

        assert_eq!(data.info.get("name").unwrap(), "pytorch");
    }

    #[test]
    fn check_pytorch_download_v1() {
        let data = request_package_info("numpy", "https://pypi.org/").unwrap();

        assert_eq!(
            data.releases.get("1.0").unwrap()[0]
                .get("filename")
                .unwrap(),
            "numpy-1.0.1.dev3460.win32-py2.4.exe"
        );
    }
}
