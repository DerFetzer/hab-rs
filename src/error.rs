use std::num::ParseFloatError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HabRsError {
    #[error("could not parse string: {0}")]
    Parse(String),
    #[error("could not parse string into float: {0}")]
    ParseFloat(#[from] ParseFloatError),
    #[error("could not parse string into date/time: {0}")]
    ParseDateTime(#[from] chrono::format::ParseError),
    #[error("json error")]
    Json(#[from] serde_json::Error),
}
