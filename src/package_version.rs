//! Handling of pep-440
use anyhow::Result;
use lazy_static::lazy_static;
use pomsky_macro::pomsky;
use serde::{Deserialize, Serialize};
use std::fmt;

static VALIDATION_REGEX: &'static str = pomsky!(
"v"?

(:epoch(['0'-'9']+)'!')?

:release(['0'-'9']+("."['0'-'9']+)*)

:pre(
    ["-" "_" "."]?

    :pre_l(
    ("preview"|"alpha"|"beta"|"pre"|"rc"|"a"|"b"|"c")
    )

    ["-" "_" "."]?

    :pre_n(['0'-'9']+)?
)?

:post(
    "-"
    :post_n1(['0'-'9']+)

    |

    ["-" "_" "."]?
    :post_l("post" | "rev" | "r")
    ["-" "_" "."]?
    :post_n2(['0'-'9']+)?
)?

:dev(
    ["-" "_" "."]?
    :dev_l("dev")
    ["-" "_" "."]?
    :dev_n(['0'-'9']+)?
)?

(
"+"
:local(
    ['a'-'z' '0'-'9']+
    ((["-" "_" "."] ['a'-'z' '0'-'9']+)+)?
)
)?
);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct DevHead {
    dev_num: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PostHead {
    Post,
    Rev,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PostHeader {
    pub post_head: Option<PostHead>,
    pub post_num: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PreHeader {
    /// Present in 1.1alpha1 or 1.1a1 both are represented the same way
    /// ```
    /// PreHeader::Alpha(Some(1))
    /// ```
    Alpha(Option<u32>),
    Beta(Option<u32>),
    ReleaseCanidate(Option<u32>),
    Preview(Option<u32>),
}

/// Tracks Major and Minor Version Numbers
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct VersionRelease {
    /// Major release such as 1.0 or breaking changes
    pub major: u32,
    /// Minor release Such as new functionality
    pub minor: u32,
}

#[derive(Debug, Serialize, Deserialize, Eq)]
pub struct PackageVersion {
    pub original: String,
    pub epoch: Option<u32>,
    pub release: VersionRelease,
    pub pre: Option<PreHeader>,
    pub post: Option<PostHeader>,
    pub dev: Option<DevHead>,
    pub local: Option<String>,
}

impl PackageVersion {
    pub fn new(version: &str) -> Result<Self> {
        lazy_static! {
            // Safe to unwrap since Regex is predefined
            // Regex as defined in PEP-0440
            static ref VERSION_VALIDATIOR: regex::Regex =
                regex::Regex::new(VALIDATION_REGEX).unwrap();
        }

        let version_match = match VERSION_VALIDATIOR.captures(version) {
            Some(v) => v,
            None => anyhow::bail!("Failed to decode version {}", version),
        };

        let epoch: Option<u32> = match version_match.name("epoch") {
            Some(v) => Some(v.as_str().parse::<u32>()?),
            None => None,
        };

        let release: VersionRelease = match version_match.name("release") {
            Some(v) => {
                if v.as_str().contains(".") {
                    let split: Vec<&str> = v.as_str().split(".").into_iter().collect();
                    VersionRelease {
                        major: split[0].parse::<u32>()?,
                        minor: split[1].parse::<u32>()?,
                    }
                } else {
                    VersionRelease {
                        major: v.as_str().parse::<u32>()?,
                        minor: 0,
                    }
                }
            }
            // There always has to be at least a major version
            None => anyhow::bail!("Failed to decode version {}", version),
        };

        let pre: Option<PreHeader> = match version_match.name("pre") {
            Some(_) => {
                let pre_n = match version_match.name("pre_n") {
                    Some(v) => Some(v.as_str().parse::<u32>()?),
                    None => None,
                };

                // Should be safe to unwrap since we already checked if pre has a value
                match version_match.name("pre_l").unwrap().as_str() {
                    "alpha" => Some(PreHeader::Alpha(pre_n)),
                    "a" => Some(PreHeader::Alpha(pre_n)),
                    "beta" => Some(PreHeader::Beta(pre_n)),
                    "b" => Some(PreHeader::Beta(pre_n)),
                    "rc" => Some(PreHeader::ReleaseCanidate(pre_n)),
                    "c" => Some(PreHeader::ReleaseCanidate(pre_n)),
                    "preview" => Some(PreHeader::Preview(pre_n)),
                    "pre" => Some(PreHeader::Preview(pre_n)),
                    _ => None,
                }
            }
            None => None,
        };

        let post: Option<PostHeader> = match version_match.name("post") {
            Some(_) => {
                let post_num: Option<u32> = match version_match.name("post_n1") {
                    Some(v) => Some(v.as_str().parse::<u32>()?),
                    None => match version_match.name("post_n2") {
                        Some(v) => Some(v.as_str().parse::<u32>()?),
                        _ => None,
                    },
                };

                let post_head: Option<PostHead> = match version_match.name("post_l") {
                    Some(v) => {
                        match v.as_str() {
                            "post" => Some(PostHead::Post),
                            "rev" => Some(PostHead::Rev),
                            "r" => Some(PostHead::Rev),
                            // This branch Should be impossible (see regex-group post_l)
                            _ => None,
                        }
                    }
                    None => None,
                };

                Some(PostHeader {
                    post_head,
                    post_num,
                })
            }
            None => None,
        };

        let dev: Option<DevHead> = match version_match.name("dev") {
            Some(_) => {
                let dev_num = match version_match.name("dev_n") {
                    Some(v) => Some(v.as_str().parse::<u32>()?),
                    None => None,
                };
                Some(DevHead { dev_num })
            }
            None => None,
        };

        let local: Option<String> = match version_match.name("local") {
            Some(v) => Some(v.as_str().to_string()),
            None => None,
        };

        Ok(Self {
            original: version.to_string(),
            epoch,
            release,
            pre,
            post,
            dev,
            local,
        })
    }
}

impl fmt::Display for PackageVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.original)
    }
}

impl PartialEq for PackageVersion {
    fn eq(&self, other: &Self) -> bool {
        self.epoch == other.epoch
            && self.release == other.release
            && self.pre == other.pre
            && self.post == other.post
            && self.dev == other.dev
            && self.local == other.local
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::package_version::PackageVersion;
    use anyhow::bail;
    use anyhow::Result;

    use super::DevHead;

    fn check_a_greater<T>(a: T, b: T) -> Result<()>
    where
        T: PartialEq + PartialOrd + Debug,
    {
        if a <= b {
            bail!(
                "Failed Less Than or Equal Check for A: {:?} \n<=\n B: {:?}",
                a,
                b
            )
        }
        Ok(())
    }

    #[test]
    fn check_dev_ordering() -> Result<()> {
        check_a_greater(DevHead { dev_num: Some(0) }, DevHead { dev_num: None })?;
        check_a_greater(DevHead { dev_num: Some(1) }, DevHead { dev_num: Some(0) })?;
        Ok(())
    }

    #[test]
    fn check_pep440_equality() -> Result<()> {
        assert_eq!(
            PackageVersion::new("1.0a1")?,
            PackageVersion::new("1.0alpha1")?
        );
        assert_eq!(
            PackageVersion::new("1.0b")?,
            PackageVersion::new("1.0beta")?
        );
        assert_eq!(PackageVersion::new("1.0r")?, PackageVersion::new("1.0rev")?);
        assert_eq!(PackageVersion::new("1.0c")?, PackageVersion::new("1.0rc")?);
        assert_eq!(PackageVersion::new("v1.0")?, PackageVersion::new("1.0")?);
        Ok(())
    }

    #[test]
    fn check_pep440() {
        // list of every example mentioned in pep-440
        let versions = vec![
            "1.0",
            "v1.1",
            "2.0",
            "2013.10",
            "2014.04",
            "1!1.0",
            "1!1.1",
            "1!2.0",
            "2!1.0.pre0",
            "1.0.dev456",
            "1.0a1",
            "1.0a2.dev456",
            "1.0a12.dev456",
            "1.0a12",
            "1.0b1.dev456",
            "1.0b2",
            "1.0b2.post345.dev456",
            "1.0b2.post345",
            "1.0rc1.dev456",
            "1.0rc1",
            "1.0",
            "1.0+abc.5",
            "1.0+abc.7",
            "1.0+5",
            "1.0.post456.dev34",
            "1.0.post456",
            "1.0.15",
            "1.1.dev1",
        ];

        for version in versions {
            match PackageVersion::new(version) {
                Ok(_v) => continue,
                Err(e) => panic!("Oh no {}", e),
            }
        }
    }

    #[test]
    fn check_pep440_negative() {
        let versions = vec!["not a version"];

        for version in versions {
            match PackageVersion::new(version) {
                Ok(v) => panic!("Oh no {}", v),
                Err(_e) => continue,
            }
        }
    }
}
