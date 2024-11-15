pub mod coinm;
pub mod spot;
pub mod usdm;

use crate::{
    client::{BinanceClient, Product},
    errors::BinanceError,
};
use futures_util::{stream::Stream, StreamExt};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json::{from_str, Value};
use std::{
    marker::PhantomData, pin::Pin, str::FromStr, task::{Context, Poll}
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, Message},
    MaybeTlsStream, WebSocketStream,
};

pub trait StreamTopic {
    const PRODUCT: Product;
    fn endpoint(&self) -> String;
    type Event: DeserializeOwned + Unpin;
}

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct BinanceWebsocket<E> {
    stream: WSStream,
    _phantom: PhantomData<E>,
}

impl<E: DeserializeOwned + Unpin> Stream for BinanceWebsocket<E> {
    type Item = Result<E, BinanceError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let msg = match self.stream.poll_next_unpin(cx) {
            Poll::Ready(Some(Ok(c))) => c,
            Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e.into()))),
            Poll::Pending => return Poll::Pending,
            Poll::Ready(None) => return Poll::Ready(None),
        };
        let text = match msg {
            Message::Text(msg) => msg,
            Message::Binary(_) | Message::Frame(_) | Message::Pong(_) | Message::Ping(_) => {
                return Poll::Pending;
            }
            Message::Close(_) => return Poll::Ready(None),
        };

        let event: E = match from_str(&text) {
            Ok(r) => r,
            Err(e) => {
                let val = Value::from_str(&text).unwrap();
                eprintln!("Failed to parse event:");
                eprintln!("{:#?}", val.as_object().unwrap());
                panic!("parsing error: {}", e);
            },
        };

        Poll::Ready(Some(Ok(event)))
    }
}

impl BinanceClient {
    pub async fn connect_stream<T: StreamTopic>(
        &self,
        topic: T,
    ) -> Result<BinanceWebsocket<T::Event>, BinanceError> {
        let base = match T::PRODUCT {
            Product::Spot => &self.config.ws_endpoint,
            Product::UsdMFutures => &self.config.usdm_futures_ws_endpoint,
            Product::CoinMFutures => &self.config.coinm_futures_ws_endpoint,
        };
        let endpoint = topic.endpoint();
        let url = Url::parse(&format!("{}{}", base, endpoint)).unwrap();
        let (stream, _) = match connect_async(url).await {
            Ok(v) => v,
            Err(tungstenite::Error::Http(http)) => {
                return Err(BinanceError::StartWebsocketError {
                    status_code: http.status(),
                    headers: http.headers().clone(),
                    body: String::from_utf8_lossy(http.body().as_deref().unwrap_or_default()).to_string(),
                })
            }
            Err(e) => return Err(e.into()),
        };
        Ok(BinanceWebsocket {
            stream,
            _phantom: PhantomData,
        })
    }
}
