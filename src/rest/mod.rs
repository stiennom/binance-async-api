pub mod coinm;
pub mod spot;
pub mod usdm;

use crate::{
    client::BinanceClient,
    errors::{RequestError, ResponseError},
    response::Response,
};
use hex::encode as hexify;
use hmac::{digest::InvalidLength, Hmac, Mac};
use reqwest::{
    self,
    header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT},
    Method,
};
use serde::{de::DeserializeOwned, Serialize};
use sha2::Sha256;

pub trait PublicRequest<T>: Serialize + Clone + Copy {
    const ENDPOINT: &'static str;
    const METHOD: Method;
    type Response: DeserializeOwned + Clone;
}

pub trait KeyedRequest<T>: Serialize + Clone + Copy {
    const ENDPOINT: &'static str;
    const METHOD: Method;
    type Response: DeserializeOwned + Clone;
}

pub trait SignedRequest<T>: Serialize + Clone + Copy {
    const ENDPOINT: &'static str;
    const METHOD: Method;
    type Response: DeserializeOwned + Clone;

    fn timestamp(&self) -> u64;
    fn recv_window(&self) -> u64;
}

impl<T> BinanceClient<T> {
    pub async fn request<R: PublicRequest<T>>(
        &self,
        req: &R,
    ) -> Result<Response<R::Response>, RequestError> {
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
            .await?;

        Ok(handle_response(resp).await?)
    }

    pub async fn keyed_request<R: KeyedRequest<T>>(
        &self,
        req: &R,
        api_key: &str,
    ) -> Result<Response<R::Response>, RequestError> {
        let base = &self.config.rest_base_url;
        let endpoint = R::ENDPOINT;
        let params = serde_qs::to_string(req).unwrap();
        let url = format!("{base}{endpoint}?{params}");

        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-async-api"));
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(api_key)?,
        );

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(custom_headers)
            .send()
            .await?; // Redirect error should not happen with correct use of binance API

        Ok(handle_response(resp).await?)
    }

    pub async fn signed_request<R: PublicRequest<T>>(
        &self,
        req: &R,
        api_key: &str,
        api_secret: &str,
    ) -> Result<Response<R::Response>, RequestError> {
        let base = &self.config.rest_base_url;
        let endpoint = R::ENDPOINT;
        let mut params = serde_qs::to_string(req).unwrap();

        let signature = signature(&params, api_secret)?;
        params.push_str(&format!("&signature={}", signature));

        let url = format!("{base}{endpoint}?{params}");

        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-async-api"));
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(api_key)?,
        );

        let resp = self
            .client
            .request(R::METHOD, url.as_str())
            .headers(custom_headers)
            .send()
            .await?; // Redirect error should not happen with correct use of binance API

        Ok(handle_response(resp).await?)
    }
}

fn signature(params: &str, secret: &str) -> Result<String, InvalidLength> {
    // Signature: hex(HMAC_SHA256(queries + data))
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;
    mac.update(params.as_bytes());
    Ok(hexify(mac.finalize().into_bytes()))
}

async fn handle_response<O: DeserializeOwned>(
    resp: reqwest::Response,
) -> Result<Response<O>, ResponseError> {
    let status = resp.status();
    let headers = Box::new(resp.headers().clone());
    if status.is_success() {
        let content = resp.json().await.unwrap();
        Ok(Response {
            status,
            headers,
            content,
        })
    } else {
        let content = resp.json().await.unwrap();
        Err(ResponseError {
            status,
            headers,
            content,
        })
    }
}
