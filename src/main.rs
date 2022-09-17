use std::collections::HashMap;

use clap::{AppSettings, Parser};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct PackageData {
    info: PackageInfo,
    last_serial: i32,
    releases: HashMap<String, Vec<PackageReleaseInfo>>,
    urls: Vec<serde_json::value::Value>,
    vulnerabilities: Vec<serde_json::value::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageDownloadInfo {
    last_day: i32,
    last_month: i32,
    last_week: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageProjectUrls {
    /// Homepage URL.
    #[serde(rename = "Homepage")]
    homepage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackageReleaseInfo {
    comment_text: String,
    digests: PackageHashValues,
    downloads: i32,
    filename: String,
    has_sig: bool,
    md5_digest: String,
    packagetype: String,
    python_version: String,
    requires_python: String,
    size: u32,
    upload_time: String,
    upload_time_iso_8601: String,
    url: String,
    yanked: bool,
    yanked_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackageHashValues {
    md5: String,
    sha256: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageInfo {
    /// Author(s) of the package.
    author: String,
    /// Author(s) emails.
    author_email: String,
    /// URL to the project bug tracker.
    bugtrack_url: serde_json::value::Value,
    /// Classifiers describing a package.
    classifiers: Vec<String>,
    /// Package description.
    description: String,
    /// Description content type.
    description_content_type: String,
    /// URL to the package documentation.
    docs_url: serde_json::value::Value,
    /// Additional links to package download.
    download_url: String,
    /// Project download information.
    downloads: PackageDownloadInfo,
    /// URL to the projects home page.
    home_page: String,
    /// Search keywords for the project.
    keywords: String,
    /// Projects license.
    license: String,
    /// Projects maintainer(s).
    maintainer: String,
    /// Project maintainer(s') email(s').
    maintainer_email: String,
    /// Packages name.
    name: String,
    /// Projects URL.
    package_url: String,
    /// Platform information (more TBD).
    platform: String,
    /// Projects package URL.
    project_url: String,
    /// Project URLs.
    project_urls: PackageProjectUrls,
    /// URL of the latest stable release.
    release_url: String,
    /// Information on requireing dists.
    requires_dist: serde_json::value::Value,
    /// Python versions required.
    requires_python: String,
    /// Projects summary.
    summary: String,
    /// Latest stable version of the package.
    version: String,
    /// Yanking information.
    yanked: bool,
    /// Reason for yanking.
    yanked_reason: serde_json::value::Value,
}

/// Python package manager written in Rust
#[derive(Parser, Debug)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
enum Opt {
    /// Install packages.
    Install {},
    /// Download packages.
    Download {
        #[clap(short = 'n', long = "name")]
        name: String,
        #[clap(short = 'i', long = "index", default_value = "https://pypi.org/")]
        index: String,
    },
    /// Uninstall packages.
    Uninstall {},
    /// List installed packages.
    List {},
    /// Show information about installed packages.
    Show {},
    /// Output installed packages in requirements format.
    Freeze {},
    /// Verify installed packages have compatible dependencies.
    Check {},
    /// Manage local and global configuration.
    Config {},
    /// Search PyPI for packages.
    Search {},
    /// Inspect and manage pip's wheel cache.
    Cache {},
    /// Inspect information available from package indexes.
    Index {},
    /// Build wheels from your requirements.
    Wheel {},
    /// Compute hashes of package archives.
    Hash {},
    /// A helper command used for command completion.
    Completion {},
    /// Show information useful for debugging.
    Debug {},
    /// Show help for commands.
    Help {},
}

async fn download_package(
    _package_name: String,
    _package_index: &str,
) -> Result<(), reqwest::Error> {
    // "https://pypi.org/pypi/sgai/json"
    let a = format!("{}pypi/{}/json", _package_index, _package_name);
    println!("{}", a);
    let body: PackageData = reqwest::Client::new()
        .get(format!("{}pypi/{}/json", _package_index, _package_name))
        .send()
        .await?
        .json()
        .await?;
    // println!("{:#?}", body);
    let dow = body.releases.get("0.0.4").and_then(|v| v.get(0));
    let dow = dow.unwrap();
    // .map(|p| &p.url);
    println!("{:?}", dow);
    let resp = reqwest::get(&dow.url).await?.bytes().await?;
    std::fs::write(&dow.filename, resp).unwrap();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let opt = Opt::parse();
    match opt {
        Opt::Download { name, index } => {
            download_package(name, &index).await?;
        }
        _ => todo!(),
    }
    Ok(())
}
