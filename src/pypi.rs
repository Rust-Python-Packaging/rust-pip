//! Warehouse PyPI API Implementation

use log::info;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Download stats
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PyPIPackageDownloadInfo {
    last_day: i32,
    last_week: i32,
    last_month: i32,
}
/// Public package information
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PyPIPackageInfo {
    pub author: String,
    pub author_email: String,
    pub bugtrack_url: serde_json::value::Value,
    pub classifiers: Vec<String>,
    pub description: String,
    pub description_content_type: String,
    pub docs_url: serde_json::value::Value,
    pub download_url: String,
    pub downloads: PyPIPackageDownloadInfo,
    pub home_page: String,
    pub keywords: String,
    pub license: String,
    pub maintainer: String,
    pub maintainer_email: String,
    /// Package name
    pub name: String,
    pub package_url: String,
    pub platform: String,
    pub project_url: String,
    pub project_urls: serde_json::value::Value,
    pub release_url: String,
    pub requires_dist: serde_json::value::Value,
    /// Minimum required python version
    pub requires_python: String,
    /// Project Summary
    pub summary: String,
    /// Latest stable version number
    pub version: String,
    pub yanked: bool,
    pub yanked_reason: serde_json::value::Value,
}

/// Set of Information describing a Python package hosted on a Warehouse instance
/// for exact details of what is contained go to <https://warehouse.pypa.io/api-reference/json.html#project>
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PyPIData {
    /// Contains data such as package name, author and license
    pub info: PyPIPackageInfo,
    pub last_serial: i32,
    /// List of releases containing Object with downloads for each release and it's versions
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
/// use pypi::request_package_info;
///
/// let data = request_package_info("numpy", "https://pypi.org/").unwrap();
/// assert_eq!(data.info.license, "BSD");
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

        assert_eq!(data.info.license, "BSD");
    }

    #[test]
    fn check_pytorch_name() {
        let data = request_package_info("pytorch", "https://pypi.org/").unwrap();

        assert_eq!(data.info.name, "pytorch");
    }

    #[test]
    fn check_numpy_download_name_v1() {
        let data = request_package_info("numpy", "https://pypi.org/").unwrap();

        assert_eq!(
            data.releases.get("1.0").unwrap()[0]
                .get("filename")
                .unwrap(),
            "numpy-1.0.1.dev3460.win32-py2.4.exe"
        );
    }

    #[test]
    #[should_panic(expected = "`Err` value: reqwest::Error")]
    fn check_fails_invalid_url() {
        let _err =
            request_package_info("numpy", "invalid_url obviously wrong").unwrap();
    }
}
