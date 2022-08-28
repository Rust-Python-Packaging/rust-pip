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

#[derive(Debug, PartialEq, Eq)]
/// Represents the possible "operators" of a package-version pair.
///
/// For now, this is `==`, `>=`, and `<=`
pub enum PyRequirementsOperator {
    EqualTo,
    GreaterThan,
    LesserThan,
}

impl PyRequirementsOperator {
    /// Creates a new `PyRequirementsOperator`
    ///
    /// # Examples
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
#[derive(Debug)]
pub struct PyRequirementsModule {
    pub package: String,
    pub version: PackageVersion,
    pub operator: PyRequirementsOperator,
}

impl PyRequirementsModule {
    /// Represents a dependency stated in a project's `requirements.txt` file
    ///
    /// # Example
    /// ```
    /// let bs4 = PyRequirementsModule::new("bs4==10.3.2");
    /// ```
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
#[derive(Debug)]
pub struct PyRequirements {
    file: PathBuf,
    /// ALl the dependencies of a project
    pub requirements: Vec<PyRequirementsModule>,
}

impl PyRequirements {
    /// Represents a `requirements.txt` file
    ///
    /// # Example
    /// ```
    /// let req = PyRequirements::new(PathBuf::from("project/requirements.txt"));
    /// ```
    pub fn new(path: &PathBuf) -> Result<Self, String> {
        if !path.exists() {
            return Err(format!("{:?} does not exist!", path.to_str().unwrap()));
        }

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

        // I FORGOR TO HAVE IT RETURN ITSELF
        // :((((((((((((((((((((((((((((((
        Ok(Self {
            file: path.to_path_buf(),
            requirements: requirements,
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{bail, Result};
    use std::path::PathBuf;

    use super::PyRequirements;
    use super::PyRequirementsModule;
    use super::PyRequirementsOperator;

    #[test]
    fn check_py_requirements_operator() -> Result<()> {
        let eq = PyRequirementsOperator::new("==").unwrap();
        let gt = PyRequirementsOperator::new(">=").unwrap();
        let lt = PyRequirementsOperator::new("<=").unwrap();

        let e1 = PyRequirementsOperator::new("AMOGUSSSSSSSSSSSSS");

        assert_eq!(eq, PyRequirementsOperator::EqualTo);
        assert_eq!(gt, PyRequirementsOperator::GreaterThan);
        assert_eq!(lt, PyRequirementsOperator::LesserThan);

        assert!(e1.is_err(), "e1 is supposed to be an Error!");

        Ok(())
    }

    #[test]
    fn check_py_requirements_line_parser() -> Result<()> {
        let sample = "Pygments==2.11.2";
        let line = PyRequirementsModule::new(&sample);

        assert!(
            line.is_ok(),
            "Failed to parse line: {:?}",
            line.unwrap_err()
        );
        let res = line.unwrap();

        assert_eq!(res.package, "Pygments");
        assert_eq!(res.version.to_string(), "2.11.2");
        assert_eq!(res.operator, PyRequirementsOperator::EqualTo);

        Ok(())
    }

    #[test]
    fn check_py_requirements_file_parser() -> Result<()> {
        let path = PathBuf::from("test/requirements.txt");
        let raw = PyRequirements::new(&path);

        assert!(
            raw.is_ok(),
            "Unable to parse file {:?}: {:?}",
            path,
            raw.unwrap_err()
        );
        Ok(())
    }
}
