use thiserror::Error;

#[derive(Error, Debug)]
pub enum RadarError {
    #[error("api token error: {0}")]
    TokenError(String),

    #[error("invalid params: {0}")]
    InvalidParamsError(String),

    #[error("network request error: {0}")]
    RequestError(#[from] reqwest::Error),
}