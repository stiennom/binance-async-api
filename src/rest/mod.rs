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

pub trait Request: Serialize {
    const PRODUCT: Product;
    const ENDPOINT: &'static str;
    const METHOD: Method;
    const KEYED: bool = false; // SIGNED imples KEYED no matter KEYED is true or false
    const SIGNED: bool = false;
    type Response: DeserializeOwned;
}

impl BinanceClient {
    pub async fn request<R>(
        &self,
        req: R,
        api_key: Option<&str>,
        api_secret: Option<&str>,
    ) -> Result<BinanceResponse<R::Response>, BinanceError>
    where
        R: Request,
    {
        let mut params = if matches!(R::METHOD, Method::GET) {
            serde_qs::to_string(&req).unwrap()
        } else {
            String::new()
        };

        let body = if !matches!(R::METHOD, Method::GET) {
            serde_qs::to_string(&req).unwrap()
        } else {
            String::new()
        };

        if R::SIGNED {
            let secret = match api_secret {
                Some(s) => s,
                None => return Err(BinanceError::MissingApiSecret),
            };
            if !params.is_empty() {
                params.push('&');
            }
            params.push_str(&format!("timestamp={}", Utc::now().timestamp_millis()));

            let signature = signature(&params, &body, secret);
            params.push_str(&format!("&signature={}", signature));
        }

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
        if R::SIGNED || R::KEYED {
            let key = match api_key {
                Some(key) => key,
                None => return Err(BinanceError::MissingApiKey),
            };
            custom_headers.insert(
                HeaderName::from_static("x-mbx-apikey"),
                HeaderValue::from_str(key).map_err(|_| BinanceError::InvalidApiKey)?,
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
}

fn signature(params: &str, body: &str, secret: &str) -> String {
    // Signature: hex(HMAC_SHA256(queries + data))
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    let sign_message = format!("{}{}", params, body);
    mac.update(sign_message.as_bytes());
    hexify(mac.finalize().into_bytes())
}

async fn handle_response<O: DeserializeOwned>(resp: Response) -> Result<BinanceResponse<O>, BinanceError> {
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
        },
    }
}
