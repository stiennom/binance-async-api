pub mod spot;
pub mod usdm;

use crate::{
    client::BinanceClient,
    errors::{BinanceError, BinanceResponseError},
};
use futures_util::{
    stream::{Stream, StreamExt},
    Sink, SinkExt,
};
use hex::encode as hexify;
use hmac::{Hmac, Mac};
use reqwest::{header::HeaderMap, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::{from_str, Value};
use sha2::Sha256;
use std::{
    marker::PhantomData,
    pin::Pin,
    str::FromStr,
    task::{Context, Poll},
};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, Message},
    MaybeTlsStream, WebSocketStream,
};

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct KeyedParams<R: Serialize> {
    #[serde(flatten)]
    params: R,
    api_key: String,
}

#[derive(Debug, Serialize)]
struct SignedParams<R: Serialize> {
    #[serde(flatten)]
    params: R,
    api_key: String,
    signature: String,
}

#[derive(Debug, Serialize)]
struct FullPublicRequest<R: Serialize> {
    id: u64,
    method: &'static str,
    params: R,
}

#[derive(Debug, Serialize)]
struct FullKeyedRequest<R: Serialize> {
    id: u64,
    method: &'static str,
    params: KeyedParams<R>,
}

#[derive(Debug, Serialize)]
struct FullSignedRequest<R: Serialize> {
    id: u64,
    method: &'static str,
    params: SignedParams<R>,
}

pub struct WsApiRequest<T> {
    raw: String,
    _marker: PhantomData<T>,
}

pub trait WsApiPublicRequest<T>: Serialize {
    fn method(&self) -> &'static str;

    fn build(self, id: u64) -> WsApiRequest<T>
    where
        Self: Sized,
    {
        let raw = public_req_into_message(id, self);
        WsApiRequest {
            raw,
            _marker: PhantomData,
        }
    }
}

pub trait WsApiKeyedRequest<T>: Serialize {
    fn method(&self) -> &'static str;

    fn build(self, id: u64, api_key: String) -> WsApiRequest<T>
    where
        Self: Sized,
    {
        let raw = keyed_req_into_message(id, self, api_key);
        WsApiRequest {
            raw,
            _marker: PhantomData,
        }
    }
}
pub trait WsApiSignedRequest<T>: Serialize {
    fn method(&self) -> &'static str;

    fn timestamp(&self) -> u64;
    fn recv_window(&self) -> u64;

    fn build(self, id: u64, api_key: String, api_secret: String) -> WsApiRequest<T>
    where
        Self: Sized,
    {
        let raw = signed_req_into_message(id, self, api_key, api_secret);
        WsApiRequest {
            raw,
            _marker: PhantomData,
        }
    }
}

fn public_req_into_message<T, R: WsApiPublicRequest<T>>(id: u64, req: R) -> String {
    let method = req.method();
    let full_req = FullPublicRequest {
        id,
        method,
        params: req,
    };
    serde_json::to_string(&full_req).unwrap()
}

fn keyed_req_into_message<T, R: WsApiKeyedRequest<T>>(id: u64, req: R, api_key: String) -> String {
    let method = req.method();
    let req_params = KeyedParams {
        params: req,
        api_key,
    };
    let full_req = FullKeyedRequest {
        id,
        method,
        params: req_params,
    };
    serde_json::to_string(&full_req).unwrap()
}

fn signed_req_into_message<T, R: WsApiSignedRequest<T>>(
    id: u64,
    req: R,
    api_key: String,
    api_secret: String,
) -> String {
    let method = req.method();
    let signature = signature(&req, &api_secret);
    let req_params = SignedParams {
        params: req,
        api_key,
        signature,
    };
    let full_req = FullSignedRequest {
        id,
        method,
        params: req_params,
    };
    serde_json::to_string(&full_req).unwrap()
}

fn signature<T>(req: &impl WsApiSignedRequest<T>, api_secret: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).unwrap();

    // Serialize the struct to a JSON object and sort the keys
    let mut json_value = serde_json::to_value(req).unwrap();
    let map = json_value.as_object_mut().unwrap();
    map.sort_keys();

    // Create the message to sign
    let mut sign_message = String::new();
    for (key, value) in map.iter() {
        sign_message.push_str(&format!("{}={}&", key, value));
    }

    mac.update(sign_message.as_bytes());
    hexify(mac.finalize().into_bytes())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u64,
    pub limit: u64,
    pub count: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsApiEvent<R: DeserializeOwned> {
    pub id: u64,
    pub status: u16,
    #[serde(flatten, deserialize_with = "deserialize_result_field")]
    pub result: Result<R, BinanceResponseError>,
    pub rate_limits: Vec<RateLimit>,
}

fn deserialize_result_field<'de, R, D>(
    deserializer: D,
) -> Result<Result<R, BinanceResponseError>, D::Error>
where
    R: Deserialize<'de>,
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RawResult<R> {
        Ok { result: R },
        Err { error: BinanceResponseError },
    }

    let raw_result = RawResult::deserialize(deserializer)?;
    match raw_result {
        RawResult::Ok { result } => Ok(Ok(result)),
        RawResult::Err { error } => Ok(Err(error)),
    }
}

pub trait WsApiResponse<T>: DeserializeOwned {}

pub struct BinanceWsApi<R> {
    stream: WSStream,
    _marker: PhantomData<R>,
}

#[derive(Debug, Clone, Error)]
#[error(
    "Failed to start ws API (status: {:?})\ncontent: {}",
    status_code,
    body
)]
pub struct StartWsApiError {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

impl BinanceError for StartWsApiError {
    fn is_server_error(&self) -> bool {
        self.status_code.is_server_error()
    }
}

impl<R: DeserializeOwned + Unpin> Stream for BinanceWsApi<R> {
    type Item = WsApiEvent<R>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let msg = match self.stream.poll_next_unpin(cx) {
            Poll::Ready(Some(Ok(c))) => c,
            Poll::Ready(Some(Err(_))) | Poll::Ready(None) => return Poll::Ready(None),
            Poll::Pending => return Poll::Pending,
        };
        let text = match msg {
            Message::Text(msg) => msg,
            Message::Binary(_) | Message::Frame(_) | Message::Pong(_) | Message::Ping(_) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
            Message::Close(_) => return Poll::Ready(None),
        };

        let event = match from_str(&text) {
            Ok(r) => r,
            Err(e) => {
                let val = Value::from_str(&text).unwrap();
                eprintln!("Failed to parse event:");
                eprintln!("{:#?}", val.as_object().unwrap());
                panic!("parsing error: {}", e);
            }
        };

        Poll::Ready(Some(event))
    }
}

impl<T, R: WsApiResponse<T> + Unpin> Sink<WsApiRequest<T>> for BinanceWsApi<R> {
    type Error = ();

    fn start_send(mut self: Pin<&mut Self>, req: WsApiRequest<T>) -> Result<(), Self::Error> {
        self.stream
            .start_send_unpin(Message::Text(req.raw))
            .map_err(|_| ())
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.stream.poll_close_unpin(cx).map_err(|_| ())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.stream.poll_flush_unpin(cx).map_err(|_| ())
    }

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.stream.poll_ready_unpin(cx).map_err(|_| ())
    }
}

impl<T> BinanceClient<T> {
    pub async fn connect_ws_api<R: WsApiResponse<T>>(
        &self,
    ) -> Result<BinanceWsApi<R>, StartWsApiError> {
        let base = &self.config.ws_api_base_url;
        let stream = match connect_async(base).await {
            Ok((s, _)) => s,
            Err(tungstenite::Error::Http(http)) => {
                return Err(StartWsApiError {
                    status_code: http.status(),
                    headers: http.headers().clone(),
                    body: String::from_utf8_lossy(http.body().as_deref().unwrap_or_default())
                        .to_string(),
                })
            }
            Err(e) => panic!("Failed to connect WS API: {}", e),
        };
        Ok(BinanceWsApi {
            stream,
            _marker: PhantomData,
        })
    }
}
