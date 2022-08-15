//! # Handling of pep-440
//! 
//! This module implements Pythons Package versioning system read more at <https://peps.python.org/pep-0440/>
use anyhow::Result;
use lazy_static::lazy_static;
use pomsky_macro::pomsky;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt};


/// Rulex version of 
/// Python's pep-440 Regex (<https://peps.python.org/pep-0440/#appendix-b-parsing-version-strings-with-regular-expressions>)
static VALIDATION_REGEX: &str = pomsky!(
// Version String may start with v<version_number>
// Example:
// v1.0
"v"?

// Version String may include an epoch <epoch_num>!<version>
// Example:
// 1!1.0
(:epoch(['0'-'9']+)'!')?

// Version String must include major and minor version <major>.<minor>
// Example:
// 1.0
:release(['0'-'9']+("."['0'-'9']+)*)

// Version String may include Pre-Header
// Example:
// 1.0.preview-2
// 1.0.rc2
// 1.0beta2
:pre(
    ["-" "_" "."]?

    :pre_l(
    ("preview"|"alpha"|"beta"|"pre"|"rc"|"a"|"b"|"c")
    )

    ["-" "_" "."]?

    :pre_n(['0'-'9']+)?
)?

// Version String may include Post-Header
// Examples:
// 1.0-9
// 1.0-post2
// 1.0.post.2
:post(
    "-"
    :post_n1(['0'-'9']+)

    |

    ["-" "_" "."]?
    :post_l("post" | "rev" | "r")
    ["-" "_" "."]?
    :post_n2(['0'-'9']+)?
)?

// Version string may include Dev-header
// Example:
// 1.0-dev3
// 1.0dev4
// 1.0_dev_9
:dev(
    ["-" "_" "."]?
    :dev_l("dev")
    ["-" "_" "."]?
    :dev_n(['0'-'9']+)?
)?

// Version string may include Local Version
// Local version must start with +
// Example:
// 1.0+this.can.say.anything.as.long.as.its.a.letter.or.number.231241
(
"+"
:local(
    ['a'-'z' '0'-'9']+
    ((["-" "_" "."] ['a'-'z' '0'-'9']+)+)?
)
)?
);

/// # Pep-440 Developmental release identifier
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct DevHead {
    dev_num: Option<u32>,
}

/// Pep-440 Post-Release Identifier Keyword
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PostHead {
    Post,
    Rev,
}

impl PartialOrd for PostHead {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

/// # Pep-440 Post-Release identifier
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PostHeader {
    pub post_head: Option<PostHead>,
    pub post_num: Option<u32>,
}

impl PartialOrd for PostHeader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.post_num == other.post_num {
            return Some(Ordering::Equal);
        }

        if self.post_num.is_none() && other.post_num.is_some() {
            return Some(Ordering::Less);
        } else if self.post_num.is_some() && other.post_num.is_none() {
            return Some(Ordering::Greater);
        }

        if self.post_num < other.post_num {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

/// # Pep-440 Pre-Release identifier
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub enum PreHeader {
    /// Present in versions like 1.1beta1 or 1.0b1 both are represented the same way
    /// ```
    /// PreHeader::Beta(Some(1))
    /// ```
    Beta(Option<u32>),
    /// Present in versions like 1.0alpha2 or 1.0a2 both are represented the same way
    /// ```
    /// PreHeader::Alpha(Some(2))
    /// ```
    Alpha(Option<u32>),
    /// Present in versions like 1.1pre3
    /// ```
    /// PreHeader::Preview(Some(3))
    /// ```
    Preview(Option<u32>),
    /// Present in versions like 1.1-rc-4 or 1.1c-4
    /// ```
    /// PreHeader::ReleaseCanidate(Some(4))
    /// ```
    ReleaseCanidate(Option<u32>),
}

/// Pep-440 Release numbers
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct ReleaseHeader {
    /// Major release such as 1.0 or breaking changes
    pub major: u32,
    /// Minor release Such as new functionality
    pub minor: u32,
}

/// Pep-440 Compliant versioning system
/// 
/// This struct is sorted so that PartialOrd
/// corretly interpets priority
///
/// Lower == More important
///
/// # Example Usage
/// ```
/// let _ = PackageVersion::new("v1.0");
/// ```
#[derive(Derivative, Debug, Serialize, Deserialize)]
#[derivative(PartialOrd, PartialEq)]
pub struct PackageVersion {
    #[derivative(PartialOrd = "ignore", PartialEq = "ignore")]
    pub original: String,

    /// # Pep-440 Local version identifier
    /// Local version sorting will have to be it's own issue
    /// since there are no limits to what a local version can be
    ///
    /// For those who can read regex here it is for the local version:
    /// `[a-z0-9]+(?:(?:[\-_.][a-z0-9]+)+)?`
    ///
    /// Here in Rulex:
    /// ```
    ///  ['a'-'z' '0'-'9']+
    ///  ((["-" "_" "."] ['a'-'z' '0'-'9']+)+)?
    /// ```
    #[derivative(PartialOrd = "ignore", PartialEq = "ignore")]
    pub local: Option<String>,

    /// # Pep-440 Developmental release identifier
    pub dev: Option<DevHead>,

    /// # Pep-440 Post-Release identifier
    pub post: Option<PostHeader>,

    /// # Pep-440 Pre-Release identifier
    pub pre: Option<PreHeader>,

    /// # Pep-440 Release number
    pub release: ReleaseHeader,

    /// # Pep-440 Version-Epoch
    pub epoch: Option<u32>,
}

impl PackageVersion {
    pub fn new(version: &str) -> Result<Self> {
        lazy_static! {
            // Safe to unwrap since Regex is predefined
            // Regex as defined in PEP-0440
            static ref VERSION_VALIDATIOR: regex::Regex =
                regex::Regex::new(VALIDATION_REGEX).unwrap();
        }

        // Capture each group of the regex
        // Groups are:
        // epoch, release, pre, pre_l, pre_n, post, post_l, post_n1, post_n2,
        // dev, dev_l, dev_n, local
        let version_match = match VERSION_VALIDATIOR.captures(version) {
            Some(v) => v,
            None => anyhow::bail!("Failed to decode version {}", version),
        };

        let epoch: Option<u32> = match version_match.name("epoch") {
            // Convert Epoch String to Epoch Number
            Some(v) => Some(v.as_str().parse::<u32>()?),
            None => None,
        };

        let release: ReleaseHeader = match version_match.name("release") {
            Some(v) => {
                // Does Release String contain minor version
                if v.as_str().contains('.') {
                    let split: Vec<&str> = v.as_str().split('.').into_iter().collect();
                    ReleaseHeader {
                        major: split[0].parse::<u32>()?,
                        minor: split[1].parse::<u32>()?,
                    }
                } else {
                    ReleaseHeader {
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
                // since pre_n has to exist 
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

        let local: Option<String> =
            version_match.name("local").map(|v| v.as_str().to_string());

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

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::package_version::PackageVersion;
    use anyhow::bail;
    use anyhow::Result;

    use super::DevHead;
    use super::PostHead;
    use super::PostHeader;
    use super::PreHeader;
    use super::ReleaseHeader;

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
    fn check_pep440_ordering() -> Result<()> {
        check_a_greater(
            PackageVersion::new("v1!1.0-preview-921.post-516.dev-241+yeah.this.is.the.problem.with.local.versions")?,
            PackageVersion::new("1.0")?,
        )?;
        Ok(())
    }

    #[test]
    fn check_release_ordering() -> Result<()> {
        check_a_greater(
            ReleaseHeader { major: 1, minor: 0 },
            ReleaseHeader { major: 0, minor: 0 },
        )?;
        check_a_greater(
            ReleaseHeader { major: 1, minor: 1 },
            ReleaseHeader { major: 1, minor: 0 },
        )?;
        check_a_greater(
            ReleaseHeader { major: 2, minor: 1 },
            ReleaseHeader {
                major: 1,
                minor: 52,
            },
        )?;
        Ok(())
    }

    #[test]
    fn check_pre_ordering() -> Result<()> {
        check_a_greater(PreHeader::ReleaseCanidate(None), PreHeader::Preview(None))?;
        check_a_greater(PreHeader::Preview(None), PreHeader::Alpha(None))?;
        check_a_greater(PreHeader::Alpha(None), PreHeader::Beta(None))?;

        check_a_greater(
            PreHeader::ReleaseCanidate(Some(2)),
            PreHeader::ReleaseCanidate(Some(1)),
        )?;
        check_a_greater(PreHeader::Preview(Some(50)), PreHeader::Preview(Some(3)))?;
        check_a_greater(PreHeader::Alpha(Some(504)), PreHeader::Alpha(Some(0)))?;
        check_a_greater(PreHeader::Beta(Some(1234)), PreHeader::Beta(Some(1)))?;

        check_a_greater(
            PreHeader::ReleaseCanidate(Some(1)),
            PreHeader::Beta(Some(45067885)),
        )?;
        Ok(())
    }

    #[test]
    fn check_post_ordering() -> Result<()> {
        check_a_greater(
            PostHeader {
                post_head: Some(PostHead::Post),
                post_num: Some(0),
            },
            PostHeader {
                post_head: Some(PostHead::Post),
                post_num: None,
            },
        )?;
        check_a_greater(
            PostHeader {
                post_head: Some(PostHead::Post),
                post_num: Some(1),
            },
            PostHeader {
                post_head: Some(PostHead::Post),
                post_num: Some(0),
            },
        )?;
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