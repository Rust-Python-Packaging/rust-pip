mod package_version;

use std::fmt::Display;
use std::fs::{read_to_string, File};
use std::path::PathBuf;

use anyhow::{bail, Result};
use package_version::PackageVersion;
use pomsky_macro::pomsky;
use regex::Regex;

static REQUIREMENTS_LINE_PARSER: &str = pomsky!(
    "v"?
    (
        :op("==" | ">=" | "<=")
    )
);

pub enum PyRequirementsOperator {
    EqualTo,
    GreaterThan,
    LesserThan,
}

impl PyRequirementsOperator {
    /// Creates a new `PyRequirementsOperator`
    ///
    /// # Usage
    /// ```
    /// let a  = PyRequirementsOperator::new("==").unwrap(); // Returns PyRequirementsOperator::EqualTo
    /// let b  = PyRequirementsOperator::new("BigChungus"); // Returns an Err
    /// let c  = PyRequirementsOperator::new("!!").unwrap(); // Also returns an Err
    /// ```
    fn new(op: &str) -> Result<Self, String> {
        if op.len() > 2 {
            return Err(format!("Operator is {} long", op.len()));
        }

        match op {
            "==" => Ok(Self::EqualTo),
            ">=" => Ok(Self::GreaterThan),
            "<=" => Ok(Self::LesserThan),
            _ => Err(format!("Unknown Operator: {}", op)),
        }
    }
}

impl Default for PyRequirementsOperator {
    fn default() -> Self {
        Self::EqualTo
    }
}

impl Display for PyRequirementsOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EqualTo => "==",
                Self::GreaterThan => ">=",
                Self::LesserThan => "<=",
            }
        )
    }
}

/// Represents a module in a `requirements.txt` file
pub struct PyRequirementsModule {
    pub package: String,
    pub version: PackageVersion,
    pub operator: PyRequirementsOperator,
}

impl PyRequirementsModule {
    fn new(raw: &str) -> Result<Self> {
        let regex = Regex::new(REQUIREMENTS_LINE_PARSER).unwrap();
        let res = match regex.captures(raw) {
            Some(caps) => caps,
            None => bail!("unable to parse line"),
        };

        let op = res.name("op").unwrap();
        let (op_start, op_end) = (op.start(), op.end());

        Ok(Self {
            operator: match PyRequirementsOperator::new(
                res.name("op").unwrap().as_str(),
            ) {
                Ok(op) => op,
                Err(err) => bail!("Op Parsing returned an error: {}", err),
            },
            package: raw[..op_start].to_string(),
            version: match PackageVersion::new(&raw[op_end..]) {
                Ok(ver) => ver,
                Err(err) => bail!("Package Versioner returned an error: {}", err),
            },
        })
    }
}

impl Display for PyRequirementsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.package, self.operator, self.version)
    }
}

/// Represents a `requirements.txt` file
pub struct PyRequirements {
    file: File,
    pub requirements: Vec<PyRequirementsModule>,
}

impl PyRequirements {
    pub fn new(path: &PathBuf) -> Result<(), String> {
        // Check if the path specified is a file
        if !path.is_file() {
            return Err(format!("{:?} is not a file!", path.to_str().unwrap()));
        }

        // Then check if that file is a "requirements.txt" file
        // TODO: Use some magic to see if the file can be parsed
        //       and then use that to check instead of this
        if !path.ends_with("requirements.txt") {
            return Err(format!(
                "File specified is not a 'requirements.txt' file: {:?}",
                path.to_str().unwrap()
            ));
        }

        let binding = read_to_string(&path).expect(
            format!("Unable to read file: {:?}", path.to_str().unwrap()).as_str(),
        );

        let raw: Vec<&str> = binding.split("\n").collect();
        let mut requirements = Vec::<PyRequirementsModule>::new();

        for (lineno, line) in raw.iter().enumerate() {
            match PyRequirementsModule::new(line) {
                Ok(py_mod) => requirements.push(py_mod),
                Err(err) => {
                    println!("Unable to parse line {}: {}", lineno, err)
                }
            }
        }

        Ok(())
    }
}
