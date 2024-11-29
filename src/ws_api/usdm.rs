use serde::Deserialize;

use super::{WsApiPublicRequest, WsApiResponse, WsApiSignedRequest};

pub use crate::rest::usdm::*;

impl<T> WsApiPublicRequest<T> for OrderBookRequest<'_> {
    fn method(&self) -> &'static str {
        "depth"
    }
}

impl<T> WsApiPublicRequest<T> for PriceTickerRequest<'_> {
    fn method(&self) -> &'static str {
        "ticker.price"
    }
}

impl<T> WsApiPublicRequest<T> for BookTickerRequest<'_> {
    fn method(&self) -> &'static str {
        "ticker.book"
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerResponse {
    pub last_update_id: u64,
    pub symbol: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
    pub time: u64,
}

impl<T> WsApiSignedRequest<T> for NewOrderRequest<'_> {
    fn method(&self) -> &'static str {
        "order.place"
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }
    fn recv_window(&self) -> u64 {
        self.recv_window.unwrap_or(5000)
    }
}

impl<T> WsApiSignedRequest<T> for CancelOrderRequest<'_> {
    fn method(&self) -> &'static str {
        "order.cancel"
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }
    fn recv_window(&self) -> u64 {
        self.recv_window.unwrap_or(5000)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum WsApiResult {
    OrderBook(OrderBookResponse),
    PriceTicker(PriceTickerResponse),
    BookTicker(BookTickerResponse),
    NewOrder(NewOrderResponse),
    CancelOrder(CancelOrderResponse),
}

impl<T> WsApiResponse<T> for WsApiResult {}

#[cfg(test)]
mod tests {
    use super::{super::WsApiRequest, *};
    use crate::client::{BinanceClient, Usdm};
    use futures_util::{SinkExt, StreamExt};
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_order_book_ws_api_request() {
        let client = BinanceClient::usdm();
        let mut ws_api = client
            .connect_ws_api::<WsApiResult>()
            .await
            .unwrap()
            .content;

        eprintln!("connected");

        let req: WsApiRequest<Usdm> = OrderBookRequest {
            symbol: "BTCUSDT",
            limit: Some(5),
        }
        .build(0);
        ws_api.send(req).await.unwrap();
        eprintln!("sent req");

        let resp = ws_api.next().await.unwrap();

        eprintln!("{:#?}", resp);

        assert!(resp.id == Some(0));
        assert!(StatusCode::from_u16(resp.status).unwrap().is_success());
        assert!(matches!(resp.result, Ok(WsApiResult::OrderBook(_))));
    }

    #[tokio::test]
    async fn test_price_ticker_ws_api_request() {
        let client = BinanceClient::usdm();
        let mut ws_api = client
            .connect_ws_api::<WsApiResult>()
            .await
            .unwrap()
            .content;

        let req: WsApiRequest<Usdm> = PriceTickerRequest { symbol: "BTCUSDT" }.build(0);
        ws_api.send(req).await.unwrap();
        let resp = ws_api.next().await.unwrap();

        eprintln!("{:#?}", resp);

        assert!(resp.id == Some(0));
        assert!(StatusCode::from_u16(resp.status).unwrap().is_success());
        assert!(matches!(resp.result, Ok(WsApiResult::PriceTicker(_))));
    }

    #[tokio::test]
    async fn test_book_ticker_ws_api_request() {
        let client = BinanceClient::usdm();
        let mut ws_api = client
            .connect_ws_api::<WsApiResult>()
            .await
            .unwrap()
            .content;

        let req: WsApiRequest<Usdm> = BookTickerRequest { symbol: "BTCUSDT" }.build(0);
        ws_api.send(req).await.unwrap();
        let resp = ws_api.next().await.unwrap();

        eprintln!("{:#?}", resp);

        assert!(resp.id == Some(0));
        assert!(StatusCode::from_u16(resp.status).unwrap().is_success());
        assert!(matches!(resp.result, Ok(WsApiResult::BookTicker(_))));
    }
}
