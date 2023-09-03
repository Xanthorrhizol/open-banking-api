pub mod api;
pub(crate) mod types;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    InvalidReqwestHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}
