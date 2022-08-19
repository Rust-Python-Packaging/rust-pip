use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "rust-pip", about = "Python package manager written in Rust.")]
enum Opt {
    /// Install packages.
    Install {},
    /// Download packages.
    Download {
        #[structopt(short = "n", long = "name")]
        name: String,
        #[structopt(short = "i", long = "index", default_value = "https://pypi.org/")]
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

fn download_package(_package_name: String, _package_index: &String) {}

fn main() {
    let opt = Opt::from_args();
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
