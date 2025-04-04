use thiserror::Error;

#[derive(Error, Debug)]
pub enum HabRsError {
    #[error("could not parse string: {0}")]
    Parse(String),
    #[error("json error")]
    Json(#[from] serde_json::Error),
}
