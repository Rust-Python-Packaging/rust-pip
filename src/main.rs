use std::path::PathBuf;
use std::collections::HashMap;

use structopt::StructOpt;
use reqwest::blocking::get;

// TODO: Discover all types of python package types
enum PackageType {
    BDistWheel,
    SDist
}

enum VersionOps {
    /// Operator is ">"
    GreaterThan,
    
    /// Operator is "<"
    LesserThan,
    
    /// Operator is "=="
    EqualTo,
    
    // Operator is ">="
    GreaterEqual,
    
    // Operator is "<="
    LesserEqual
}


struct PyPackageEntry {
    version: String, // Or a tuple of MAJOR, MINOR, PATCH assuming that every python package uses semver
    md5_hash: String,
    url: String,
    python_version: String, // Again, semver tuple can be used here
    size: u8, // Can be helpful in progress bars
    filename: String
}

impl PyPackageEntry {
    fn new(json: &HashMap<String, String>) {
        let digests = match json.get("digests")?.get("md5")?;
    }
}

struct PythonPackage {
    versions: HashMap<PyPackageEntry, VersionOps>
}

impl PythonPackage {
    fn new(name: &String) -> Self {
        let json = get(format!("https://pypi.org/pypi/{}/json", name))
                   .expect(format!("Unable to get metadata for {:?}", name))
                   .json::<HashMap<String, String>>()
                   .expect("Unable to parse metadata");
        let metadata = &json.get("info");
        let downloads = &json.get("urls");

}

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



fn download_package(pkg: &PythonPackage) {}

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
