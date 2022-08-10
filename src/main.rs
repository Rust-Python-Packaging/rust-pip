use clap::{AppSettings, Parser};

#[macro_use]
extern crate derivative;

mod package_version;

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

fn download_package(_package_name: String, _package_index: &str) {}

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
