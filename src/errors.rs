use reqwest::{header::InvalidHeaderValue, StatusCode};
use serde::Deserialize;
use thiserror::Error;
use tokio_tungstenite::tungstenite;


#[derive(Deserialize, Debug, Clone)]
pub(crate) struct BinanceResponseError {
    pub code: i64,
    pub msg: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum BinanceResponse<T> {
    Success(T),
    Error(BinanceResponseError),
}

impl<T> BinanceResponse<T> {
    pub(crate) fn to_result(self) -> Result<T, BinanceError> {
        match self {
            BinanceResponse::Success(t) => Ok(t),
            BinanceResponse::Error(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, Error)]
pub enum BinanceError {
    #[error("No Api key set for private api")]
    MissingApiKey,
    #[error("No Api secret set for private api")]
    MissingApiSecret,
    #[error("Error when try to connect websocket: {status_code} - {body}")]
    StartWebsocketError { status_code: StatusCode, body: String },
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
