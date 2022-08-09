mod requirements {
    use std::fmt::Display;
    use std::fs::{read_to_string, File};
    use std::path::PathBuf;

    pub enum PyRequirementsOperator {
        EqualTo,
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
                }
            )
        }
    }

    /// Represents a module in a `requirements.txt` file
    pub struct PyRequirementsModule {
        pub package: String,
        pub version: String,
        pub operator: PyRequirementsOperator,
    }

    impl PyRequirementsModule {
        fn new(raw: String) -> Self {
            todo!();
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
            if !path.is_file() {
                return Err(format!("{:?} is not a file!", path.to_str().unwrap()));
            } else {
                let raw: Vec<String> = read_to_string(&path)
                    .expect(
                        format!("Unable to read file: {:?}", path.to_str().unwrap())
                            .as_str(),
                    )
                    .split("\n")
                    .map(|item| item.to_string())
                    .collect();
            }

            return Ok(());
        }
    }
}
