pub mod coinm;
pub mod spot;
pub mod usdm;

use std::str::FromStr;

use crate::{
    client::{BinanceClient, Product},
    errors::{BinanceError, BinanceResponse, BinanceResponseContent},
};
use chrono::Utc;
use hex::encode as hexify;
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT},
    Method, Response,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, Value};
use sha2::Sha256;

pub trait Request: Serialize + KeyedRequest {}

pub trait KeyedRequest: Serialize + SignedRequest {}

pub trait SignedRequest: Serialize {
    const PRODUCT: Product;
    const ENDPOINT: &'static str;
    const METHOD: Method;
    type Response: DeserializeOwned;
}

impl BinanceClient {
    pub async fn request<R: Request>(
        &self,
        req: &R,
    ) -> Result<BinanceResponse<R::Response>, BinanceError> {
        let params = build_params(req);
        let body = build_body(req);
        let path = R::ENDPOINT;
        let base = match R::PRODUCT {
            Product::Spot => self.config.rest_api_endpoint,
            Product::UsdMFutures => self.config.usdm_futures_rest_api_endpoint,
            Product::CoinMFutures => self.config.coinm_futures_rest_api_endpoint,
        };
        let url = format!("{base}{path}?{params}");

        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-async-api"));
        if !body.is_empty() {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(custom_headers)
            .body(body)
            .send()
            .await?;

        handle_response(resp).await
    }

    pub async fn keyed_request<R: KeyedRequest>(
        &self,
        req: &R,
        api_key: &str,
    ) -> Result<BinanceResponse<R::Response>, BinanceError> {
        let params = build_params(req);
        let body = build_body(req);
        let path = R::ENDPOINT;
        let base = match R::PRODUCT {
            Product::Spot => self.config.rest_api_endpoint,
            Product::UsdMFutures => self.config.usdm_futures_rest_api_endpoint,
            Product::CoinMFutures => self.config.coinm_futures_rest_api_endpoint,
        };
        let url = format!("{base}{path}?{params}");

        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-async-api"));
        if !body.is_empty() {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(api_key).map_err(|_| BinanceError::InvalidApiKey)?,
        );

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(custom_headers)
            .body(body)
            .send()
            .await?;

        handle_response(resp).await
    }

    pub async fn signed_request<R: Request>(
        &self,
        req: &R,
        api_key: &str,
        api_secret: &str,
    ) -> Result<BinanceResponse<R::Response>, BinanceError> {
        let mut params = build_params(req);
        let body = build_body(req);

        if !params.is_empty() {
            params.push('&');
        }
        params.push_str(&format!("timestamp={}", Utc::now().timestamp_millis()));

        let signature = signature(&params, &body, api_secret);
        params.push_str(&format!("&signature={}", signature));

        let path = R::ENDPOINT;

        let base = match R::PRODUCT {
            Product::Spot => self.config.rest_api_endpoint,
            Product::UsdMFutures => self.config.usdm_futures_rest_api_endpoint,
            Product::CoinMFutures => self.config.coinm_futures_rest_api_endpoint,
        };
        let url = format!("{base}{path}?{params}");

        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-async-api"));
        if !body.is_empty() {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(api_key).map_err(|_| BinanceError::InvalidApiKey)?,
        );

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(custom_headers)
            .body(body)
            .send()
            .await?;

        handle_response(resp).await
    }
}

fn build_params<R: SignedRequest>(request: &R) -> String {
    if matches!(R::METHOD, Method::GET) {
        serde_qs::to_string(request).unwrap()
    } else {
        String::new()
    }
}

fn build_body<R: SignedRequest>(request: &R) -> String {
    if !matches!(R::METHOD, Method::GET) {
        serde_qs::to_string(request).unwrap()
    } else {
        String::new()
    }
}

fn signature(params: &str, body: &str, secret: &str) -> String {
    // Signature: hex(HMAC_SHA256(queries + data))
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    let sign_message = format!("{}{}", params, body);
    mac.update(sign_message.as_bytes());
    hexify(mac.finalize().into_bytes())
}

async fn handle_response<O: DeserializeOwned>(
    resp: Response,
) -> Result<BinanceResponse<O>, BinanceError> {
    let status_code = resp.status();
    let headers = resp.headers().clone();
    let resp_text = resp.text().await?;
    match from_str(&resp_text) {
        Ok(BinanceResponseContent::Success(content)) => Ok(BinanceResponse {
            status_code,
            headers,
            content,
        }),
        Ok(BinanceResponseContent::Error(content)) => Err(BinanceError::BinanceResponse {
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
}
