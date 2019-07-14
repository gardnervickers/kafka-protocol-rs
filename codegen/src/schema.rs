use crate::error;
use crate::error::CodegenError;
use regex;
use serde::{self, Deserialize as _, Deserializer};
use serde_derive::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq)]
pub(crate) enum ParsedSchemaType {
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "response")]
    Response,
}

impl ToString for ParsedSchemaType {
    fn to_string(&self) -> String {
        match self {
            ParsedSchemaType::Request => String::from("Request"),
            ParsedSchemaType::Response => String::from("Response"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Version {
    Present { start: i16 },
    Range { start: i16, end: i16 },
    Single(i16),
}

fn deserialize_version<'de, D>(deserializer: D) -> Result<Version, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    Version::try_from_str(&s).map_err(serde::de::Error::custom)
}

fn deserialize_opt_version<'de, D>(deserializer: D) -> Result<Option<Version>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "deserialize_version")] Version);

    let v = Option::deserialize(deserializer)?;
    Ok(v.map(|Wrapper(a)| a))
}

impl Version {
    /// Try to parse the Version from a version string. Version strings
    /// are either `0+` representing "from version zero onwards" or
    /// `0-3` representing "from version 0 to version 3".
    pub(crate) fn try_from_str(vs: &str) -> Result<Self, error::CodegenError> {
        let re = regex::Regex::new(r"^(\d+)\+|(\d+)-(\d+)|(\d)$").unwrap();

        fn parse_version(s: &str) -> Result<i16, CodegenError> {
            s.parse().map_err(|_| CodegenError::InvalidVersion)
        }

        re.captures(vs)
            .ok_or_else(|| error::CodegenError::InvalidVersion)
            .and_then(|captures| {
                if let Some(present_field) = captures.get(1) {
                    Ok(Version::Present {
                        start: parse_version(present_field.as_str())?,
                    })
                } else if let (Some(start_field), Some(end_field)) =
                    (captures.get(2), captures.get(3))
                {
                    Ok(Version::Range {
                        start: parse_version(start_field.as_str())?,
                        end: parse_version(end_field.as_str())?,
                    })
                } else if let Some(locked_field) = captures.get(4) {
                    Ok(Version::Single(parse_version(locked_field.as_str())?))
                } else {
                    Err(CodegenError::InvalidVersion)
                }
            })
    }
    pub(crate) fn version_start(&self) -> i16 {
        *match self {
            Version::Present { start } => start,
            Version::Range { start, .. } => start,
            Version::Single(v) => v,
        }
    }
    pub(crate) fn version_end(&self) -> Option<i16> {
        match self {
            Version::Present { .. } => None,
            Version::Range { end, .. } => Some(*end),
            Version::Single(_v) => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ParsedSchemaField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(deserialize_with = "deserialize_version")]
    pub versions: Version,
    #[serde(
        default,
        deserialize_with = "deserialize_opt_version",
        rename = "nullableVersions"
    )]
    pub nullable_versions: Option<Version>,
    pub fields: Option<Vec<ParsedSchemaField>>,
    pub about: Option<String>,
    pub default: Option<String>,
    #[serde(default)]
    pub ignorable: bool,
    #[serde(rename = "mapKey")]
    pub map_key: Option<bool>,
}
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ParsedSchema {
    pub name: String,
    #[serde(rename = "apiKey")]
    pub api_key: i16,
    #[serde(rename = "type")]
    pub schema_type: ParsedSchemaType,
    #[serde(deserialize_with = "deserialize_version")]
    #[serde(rename = "validVersions")]
    pub valid_versions: Version,
    pub fields: Vec<ParsedSchemaField>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_test() {
        assert_eq!(
            Version::try_from_str("0-3").unwrap(),
            Version::Range { start: 0, end: 3 }
        );
        assert_eq!(
            Version::try_from_str("0+").unwrap(),
            Version::Present { start: 0 }
        );
        assert_eq!(Version::try_from_str("3").unwrap(), Version::Single(3));
    }
}
