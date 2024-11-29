use hmac::digest::InvalidLength;
use reqwest::{
    header::{HeaderMap, InvalidHeaderValue},
    StatusCode,
};
use serde::Deserialize;
use thiserror::Error;
use tokio_tungstenite::tungstenite;

#[derive(Debug, Clone, Error)]
#[error("Error status {} ({})", status, content)]
pub struct ResponseError {
    pub status: StatusCode,
    pub headers: Box<HeaderMap>,
    pub content: ContentError,
}

#[derive(Deserialize, Debug, Clone, Error)]
#[error("code: {} - msg: {}", code, msg)]
pub struct ContentError {
    pub code: i64,
    pub msg: String,
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Invalid API key: {0}")]
    InvalidApiKey(#[from] InvalidHeaderValue),
    #[error("Invalid API secret: {0}")]
    InvalidApiSecret(#[from] InvalidLength),
    #[error(transparent)]
    Response(#[from] ResponseError),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum WsConnectionError {
    #[error("Ws connection error: {0}")]
    Connection(#[from] Box<tungstenite::Error>),
}
