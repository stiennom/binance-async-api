use reqwest::{header::InvalidHeaderValue, StatusCode};
use serde::Deserialize;
use thiserror::Error;
use tokio_tungstenite::tungstenite;

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceResponseError {
    pub code: i64,
    pub msg: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BinanceResponse<T> {
    Success(T),
    Error(BinanceResponseError),
}

impl<T: for<'a> Deserialize<'a>> BinanceResponse<T> {
    pub fn to_result(self) -> Result<T, BinanceResponseError> {
        match self {
            BinanceResponse::Success(t) => Ok(t),
            BinanceResponse::Error(e) => Err(e),
        }
    }
}

#[derive(Debug, Error)]
pub enum BinanceError {
    #[error("No Api key set for private api")]
    MissingApiKey,
    #[error("No Api secret set for private api")]
    MissingApiSecret,
    #[error("Error when try to connect websocket: {0} - {1}")]
    StartWebsocketError(StatusCode, String),
    #[error("Binance returns error: {code} - {msg}")]
    BinanceResponse { code: i64, msg: String },

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

impl From<BinanceResponseError> for BinanceError {
    fn from(v: BinanceResponseError) -> Self {
        Self::BinanceResponse {
            code: v.code,
            msg: v.msg,
        }
    }
}
