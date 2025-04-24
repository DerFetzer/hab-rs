use std::num::ParseFloatError;

use thiserror::Error;

/// Errors produced by hab-rs
#[derive(Error, Debug)]
pub enum HabRsError {
    /// Could not parse a value.
    #[error("could not parse string: {0}")]
    Parse(String),
    /// Could not parse a float.
    #[error("could not parse string into float: {0}")]
    ParseFloat(#[from] ParseFloatError),
    /// Could not parse date and/or time.
    #[error("could not parse string into date/time: {0}")]
    ParseDateTime(#[from] chrono::format::ParseError),
    /// Could not serialize/deserialize a JSON value.
    #[error("json error")]
    Json(#[from] serde_json::Error),
    /// Could not decode Base64 string.
    #[error("could not decode as base64: {0}")]
    Base64(#[from] base64::DecodeError),
}
