use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use structopt::StructOpt;
use reqwest::Client;
use termprogress::progress::Bar;

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
    digests: String,
    url: String,
    python_version: String, // Again, semver tuple can be used here
    size: u8, // Can be helpful in progress bars
    filename: String
}

impl PyPackageEntry {
    fn new(json: &HashMap<String, String>, version: &String) -> Result<Self, String> {
        PyPackageEntry {
            version: version,
            digest: json.get("digests")?.get("md5")?,
            url: json.get("url")?,
            python_version: json.get("requires_python")?,
            size: json.get("size") as u8,
            filename: json.get("filename")?
    }
}

struct PythonPackage {
    versions: HashMap<PyPackageEntry, VersionOps>
}

impl PythonPackage {
    fn new(name: &String, version: &String, client: &Client) -> Self {
        let json = client.get(format!("https://pypi.org/pypi/{}/{}/json", name, version))
                   .expect(format!("Unable to get metadata for {:?}", name))
                   .json::<HashMap<String, String>>()
                   .expect("Unable to parse metadata");
        let metadata = &json.get("info");
        let downloads = &json.get("urls");
        
        for entry in downloads
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


fn download_package(pkg: &PyPackageEntry, client: &Client) -> Result<(), String> {
    let resp = client.get(pkg.url)
               .or(Err(format!("Unable to download {:?}", pkg.filename)));
    
}

fn main() {
    let client = Client::new();
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
