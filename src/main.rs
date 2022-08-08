use std::path::PathBuf;
use structopt::StructOpt;
use clap::{AppSettings, Parser};
use anyhow::Result;

mod pypi;
use pypi::{request_package_info, PypiData};

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

fn download_package(package_name: String, package_index: &String) -> Result<()> {
    let package_info: PypiData = request_package_info(&package_name, &package_index)?;

    // Example of getting data this will be more robust as the 
    // PypiData struct gets expanded (meaning less calls to .get())
    let latest_version = package_info.info.get("version").unwrap();
    println!("Latest Version of {} is {}", package_name, latest_version);

    Ok(())
}

fn main() {
    let opt = Opt::parse();
    println!("{:#?}", opt);

    match opt {
        Opt::Download { name, index } => {
            println!("Package name {:?}", name);
            println!("Index name: {:?}", index);
            download_package(name, &index);
        }
        _ => todo!(),
    }
}
