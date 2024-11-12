use reqwest::{header::InvalidHeaderValue, StatusCode, header::HeaderMap};
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
    #[error("No Api key set for private api")]
    MissingApiKey,
    #[error("No Api secret set for private api")]
    MissingApiSecret,
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
    Websocket(#[from] tungstenite::Error),
    #[error(transparent)]
    SerdeQs(#[from] serde_qs::Error),
    #[error(transparent)]
    HttpHeader(#[from] InvalidHeaderValue),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
