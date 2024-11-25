pub mod coinm;
pub mod spot;
pub mod usdm;

use crate::{client::BinanceClient, errors::BinanceError};
use futures_util::stream::{Stream, StreamExt};
use reqwest::{header::HeaderMap, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::{from_str, Value};
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

pub trait StreamTopic<T> {
    fn endpoint(&self) -> String;
    type Event: DeserializeOwned;
}

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct BinanceWebsocket<E> {
    stream: WSStream,
    _phantom: PhantomData<E>,
}

#[derive(Debug, Clone, Error)]
#[error(
    "Failed to start websocket (status: {:?})\ncontent: {}",
    status_code,
    body
)]
pub struct StartWebsocketError {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

impl BinanceError for StartWebsocketError {
    fn is_server_error(&self) -> bool {
        self.status_code.is_server_error()
    }
}

impl<E: DeserializeOwned + Unpin> Stream for BinanceWebsocket<E> {
    type Item = E;

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

impl<T> BinanceClient<T> {
    pub async fn connect_stream<S: StreamTopic<T>>(
        &self,
        topic: &S,
    ) -> Result<BinanceWebsocket<S::Event>, StartWebsocketError> {
        let base = &self.config.websocket_base_url;
        let endpoint = topic.endpoint();
        let url = format!("{}{}", base, endpoint);
        let stream = match connect_async(url).await {
            Ok((s, _)) => s,
            Err(tungstenite::Error::Http(http)) => {
                return Err(StartWebsocketError {
                    status_code: http.status(),
                    headers: http.headers().clone(),
                    body: String::from_utf8_lossy(http.body().as_deref().unwrap_or_default())
                        .to_string(),
                })
            }
            Err(e) => panic!("Failed to connect websocket: {}", e),
        };
        Ok(BinanceWebsocket {
            stream,
            _phantom: PhantomData,
        })
    }
}
