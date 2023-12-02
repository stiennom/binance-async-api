pub mod coinm;
pub mod spot;
pub mod usdm;

use crate::{
    client::{BinanceClient, Product},
    errors::{BinanceError, BinanceResponse},
};
use chrono::Utc;
use hex::encode as hexify;
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT},
    Method, Response,
};
use serde::{de::DeserializeOwned, Serialize};
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
    pub async fn request<R>(&self, req: R) -> Result<R::Response, BinanceError>
    where
        R: Request,
    {
        let mut params = if matches!(R::METHOD, Method::GET) {
            serde_qs::to_string(&req)?
        } else {
            String::new()
        };

        let body = if !matches!(R::METHOD, Method::GET) {
            serde_qs::to_string(&req)?
        } else {
            String::new()
        };

        if R::SIGNED {
            if !params.is_empty() {
                params.push('&');
            }
            params.push_str(&format!("timestamp={}", Utc::now().timestamp_millis()));

            let signature = self.signature(&params, &body)?;
            params.push_str(&format!("&signature={}", signature));
        }

        let path = R::ENDPOINT.to_string();

        let base = match R::PRODUCT {
            Product::Spot => &self.config.rest_api_endpoint,
            Product::UsdMFutures => &self.config.usdm_futures_rest_api_endpoint,
            Product::CoinMFutures => &self.config.coinm_futures_rest_api_endpoint,
        };
        let url = format!("{base}{path}?{params}");

        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-async-rs"));
        if !body.is_empty() {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        if R::SIGNED || R::KEYED {
            let key = match &self.key {
                Some(key) => key,
                None => return Err(BinanceError::MissingApiKey),
            };
            custom_headers.insert(
                HeaderName::from_static("x-mbx-apikey"),
                HeaderValue::from_str(key)?,
            );
        }

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(custom_headers)
            .body(body)
            .send()
            .await?;

        self.handle_response(resp).await
    }

    fn signature(&self, params: &str, body: &str) -> Result<String, BinanceError> {
        let secret = match &self.secret {
            Some(s) => s,
            None => return Err(BinanceError::MissingApiSecret),
        };
        // Signature: hex(HMAC_SHA256(queries + data))
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        let sign_message = format!("{}{}", params, body);
        mac.update(sign_message.as_bytes());
        let signature = hexify(mac.finalize().into_bytes());
        Ok(signature)
    }

    async fn handle_response<O: DeserializeOwned>(
        &self,
        resp: Response,
    ) -> Result<O, BinanceError> {
        let resp: BinanceResponse<O> = resp.json().await?;
        resp.to_result()
    }
}
