pub mod coinm;
pub mod spot;
pub mod usdm;

use std::str::FromStr;

use crate::{
    client::BinanceClient,
    errors::{BinanceError, BinanceResponseError},
};
use hex::encode as hexify;
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT},
    Method, Response, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, Value};
use sha2::Sha256;
use thiserror::Error;

pub trait PublicRequest<T>: Serialize {
    const ENDPOINT: &'static str;
    const METHOD: Method;
    type Response: DeserializeOwned;
}

pub trait KeyedRequest<T>: Serialize {
    const ENDPOINT: &'static str;
    const METHOD: Method;
    type Response: DeserializeOwned;
}

pub trait SignedRequest<T>: Serialize {
    const ENDPOINT: &'static str;
    const METHOD: Method;
    type Response: DeserializeOwned;

    fn timestamp(&self) -> u64;
    fn recv_window(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct BinanceResponse<T> {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub content: T,
}

#[derive(Debug, Clone, Error)]
pub enum BinanceRequestError {
    #[error("Api key is invalid")]
    InvalidApiKey,
    #[error(
        "Binance returns error (status: {:?})\ncontent: {:?}",
        status_code,
        content
    )]
    BinanceResponse {
        status_code: StatusCode,
        headers: HeaderMap,
        content: Option<BinanceResponseError>,
    },
}

impl BinanceError for BinanceRequestError {
    fn is_server_error(&self) -> bool {
        match self {
            Self::InvalidApiKey => false,
            Self::BinanceResponse { status_code, .. } => status_code.is_server_error(),
        }
    }
}

impl<T> BinanceClient<T> {
    pub async fn request<R: PublicRequest<T>>(
        &self,
        req: &R,
    ) -> Result<BinanceResponse<R::Response>, BinanceRequestError> {
        let base = &self.config.rest_base_url;
        let endpoint = R::ENDPOINT;
        let params = serde_qs::to_string(req).unwrap();
        let url = format!("{base}{endpoint}?{params}");

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("binance-async-api"));

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(headers)
            .send()
            .await
            .unwrap(); // Redirect error should not happen with correct use of binance API

        handle_response(resp).await
    }

    pub async fn keyed_request<R: KeyedRequest<T>>(
        &self,
        req: &R,
        api_key: &str,
    ) -> Result<BinanceResponse<R::Response>, BinanceRequestError> {
        let base = &self.config.rest_base_url;
        let endpoint = R::ENDPOINT;
        let params = serde_qs::to_string(req).unwrap();
        let url = format!("{base}{endpoint}?{params}");

        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-async-api"));
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(api_key).map_err(|_| BinanceRequestError::InvalidApiKey)?,
        );

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(custom_headers)
            .send()
            .await
            .unwrap(); // Redirect error should not happen with correct use of binance API

        handle_response(resp).await
    }

    pub async fn signed_request<R: PublicRequest<T>>(
        &self,
        req: &R,
        api_key: &str,
        api_secret: &str,
    ) -> Result<BinanceResponse<R::Response>, BinanceRequestError> {
        let base = &self.config.rest_base_url;
        let endpoint = R::ENDPOINT;
        let mut params = serde_qs::to_string(req).unwrap();

        let signature = signature(&params, api_secret);
        params.push_str(&format!("&signature={}", signature));

        let url = format!("{base}{endpoint}?{params}");

        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-async-api"));
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(api_key).map_err(|_| BinanceRequestError::InvalidApiKey)?,
        );

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(custom_headers)
            .send()
            .await
            .unwrap(); // Redirect error should not happen with correct use of binance API

        handle_response(resp).await
    }
}

fn signature(params: &str, secret: &str) -> String {
    // Signature: hex(HMAC_SHA256(queries + data))
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(params.as_bytes());
    hexify(mac.finalize().into_bytes())
}

async fn handle_response<O: DeserializeOwned>(
    resp: Response,
) -> Result<BinanceResponse<O>, BinanceRequestError> {
    let status_code = resp.status();
    let headers = resp.headers().clone();
    if status_code.is_success() {
        // status is success so text should be safe to unwrap
        let resp_text = resp.text().await.unwrap();
        match from_str(&resp_text) {
            Ok(content) => Ok(BinanceResponse {
                status_code,
                headers,
                content,
            }),
            Err(e) => {
                let val = Value::from_str(&resp_text).unwrap();
                eprintln!("Failed to parse response:");
                eprintln!("{:#?}", val.as_object().unwrap());
                panic!("parsing error: {}", e);
            }
        }
    } else {
        match resp.text().await {
            Ok(resp_text) => match from_str(&resp_text) {
                Ok(content) => Err(BinanceRequestError::BinanceResponse {
                    status_code,
                    headers,
                    content: Some(content),
                }),
                Err(e) => {
                    let val = Value::from_str(&resp_text).unwrap();
                    eprintln!("Failed to parse response:");
                    eprintln!("{:#?}", val.as_object().unwrap());
                    panic!("parsing error: {}", e);
                }
            },
            Err(_) => Err(BinanceRequestError::BinanceResponse {
                status_code,
                headers,
                content: None,
            }),
        }
    }
}
