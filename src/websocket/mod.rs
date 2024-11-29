pub mod coinm;
pub mod spot;
pub mod usdm;

use crate::{client::BinanceClient, errors::WsConnectionError, response::Response};
use futures_util::stream::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use serde_json::{from_str, Value};
use std::{
    marker::PhantomData,
    pin::Pin,
    str::FromStr,
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub trait StreamTopic<T>: Clone + Copy {
    fn endpoint(&self) -> String;
    type Event: DeserializeOwned + Clone;
}

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
pub struct BinanceWebsocket<E> {
    stream: WSStream,
    _marker: PhantomData<E>,
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
    ) -> Result<Response<BinanceWebsocket<S::Event>>, WsConnectionError> {
        let base = &self.config.websocket_base_url;
        let endpoint = topic.endpoint();
        let url = format!("{}{}", base, endpoint);
        match connect_async(url).await {
            Ok((stream, response)) => {
                let status_code = response.status();
                let headers = Box::new(response.headers().clone());
                let ws_api = BinanceWebsocket {
                    stream,
                    _marker: PhantomData,
                };
                Ok(Response {
                    status: status_code,
                    headers,
                    content: ws_api,
                })
            }
            Err(e) => Err(Box::new(e).into()),
        }
    }
}
