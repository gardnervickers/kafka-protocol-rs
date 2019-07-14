use serde_json;
use std::fmt;

#[derive(Debug)]
pub(crate) enum CodegenError {
    MalformedSchema(serde_json::Error),
    InvalidVersion,
    #[allow(dead_code)] // Satisfy rustc
    InvalidName,
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            CodegenError::MalformedSchema(inner) => fmt::Display::fmt(inner, f),
            CodegenError::InvalidVersion => f.write_str("Invalid version"),
            CodegenError::InvalidName => f.write_str("Invalid schema name"),
        }
    }
}
