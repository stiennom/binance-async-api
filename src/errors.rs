use reqwest::{header::HeaderMap, StatusCode};
use serde::Deserialize;
use thiserror::Error;
use tokio_tungstenite::tungstenite;

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceResponseError {
    pub code: i64,
    pub msg: String,
}

#[derive(Debug, Clone)]
pub struct BinanceResponse<T> {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub content: T,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum BinanceResponseContent<T> {
    Success(T),
    Error(BinanceResponseError),
}

#[derive(Debug, Error)]
pub enum BinanceError {
    #[error("Api key is invalid")]
    InvalidApiKey,
    #[error("Error when try to connect websocket: {status_code} - {body}")]
    StartWebsocketError {
        status_code: StatusCode,
        headers: HeaderMap,
        body: String,
    },
    #[error("Binance returns error: {} - {}", content.code, content.msg)]
    BinanceResponse {
        status_code: StatusCode,
        headers: HeaderMap,
        content: BinanceResponseError,
    },

    #[error(transparent)]
    WebsocketError(#[from] tungstenite::Error),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}

impl BinanceError {
    pub fn is_server_error(&self) -> bool {
        match self {
            BinanceError::InvalidApiKey => false,
            BinanceError::StartWebsocketError { status_code, .. } => status_code.is_server_error(),
            BinanceError::BinanceResponse { status_code, .. } => status_code.is_server_error(),
            BinanceError::WebsocketError(_) => false,
            BinanceError::RequestError(e) => e.status().is_some_and(|s| s.is_server_error()),
        }
    }
}
