use std::collections::HashMap;

use clap::{AppSettings, Parser};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct PythonPackageIndexProjectJsonInfo {
    /// Author(s) of the package.
    author: String,
    /// Author(s) emails.
    author_email: String,
    /// URL to the project bug tracker.
    bugtrack_url: String,
    /// Classifiers describing a package.
    classifiers: Vec<String>,
    /// Package description.
    description: String,
    /// Description content type.
    description_content_type: String,
    /// URL to the package documentation.
    docs_url: String,
    /// Additional links to package download.
    download_url: String,
    /// Project download information.
    downloads: PythonPackageIndexProjectDownloadInfo,
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
    project_urls: PythonPackageIndexProjectProjectUrls,
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

#[derive(Debug, Serialize, Deserialize)]
struct PythonPackageIndexProjectDownloadInfo {
    last_day: i32,
    last_month: i32,
    last_week: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PythonPackageIndexProjectProjectUrls {
    /// Download URL.
    download: String,
    /// Homepage URL.
    homepage: String,
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
    let body = reqwest::Client::new()
        .get("https://pypi.org/pypi/sgai/json")
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let opt = Opt::parse();

    let body = reqwest::Client::new()
        .get("https://pypi.org/pypi/sgai/json")
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", body);
    println!("Bruh");
    Ok(())
    // match opt {
    //     Opt::Download { name, index } => {
    //         download_package(name, &index);
    //     }
    //     _ => todo!(),
    // }
}
