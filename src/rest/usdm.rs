use crate::client::Usdm;

use super::{KeyedRequest, PublicRequest, SignedRequest};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ExchangeInfoRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoResponse {
    pub exchange_filters: Vec<ExchangeFilter>,
    pub rate_limits: Vec<RateLimit>,
    pub server_time: u64,
    pub assets: Vec<Asset>,
    pub symbols: Vec<Market>,
    pub timezone: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExchangeFilter {
    // No info about this on binance api docs
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u64,
    pub limit: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset: String,
    pub margin_available: bool,
    pub auto_asset_exchange: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub symbol: String,
    pub pair: String,
    pub contract_type: String,
    pub delivery_date: u64,
    pub onboard_date: u64,
    pub status: String,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub margin_asset: String,
    pub price_precision: u64,
    pub quantity_precision: u64,
    pub base_asset_precision: u64,
    pub quote_precision: u64,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
    pub trigger_protect: String,
    pub filters: Vec<SymbolFilter>,
    pub order_types: Vec<String>,
    pub time_in_force: Vec<String>,
    pub liquidation_fee: String,
    pub market_take_bound: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolFilter {
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: String,
        max_price: String,
        tick_size: String,
    },
    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    MaxNumOrders {
        limit: u64,
    },
    MaxNumAlgoOrders {
        limit: u64,
    },
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: String,
        multiplier_down: String,
        multiplier_decimal: String,
    },
    MinNotional {
        notional: String,
    },
}

impl PublicRequest<Usdm> for ExchangeInfoRequest {
    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/exchangeInfo";
    type Response = ExchangeInfoResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct OrderBookRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub last_update_id: u64,
    #[serde(rename = "E")]
    pub message_output_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    pub bids: Vec<BookLevel>,
    pub asks: Vec<BookLevel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookLevel {
    pub price: String,
    pub qty: String,
}

impl PublicRequest<Usdm> for OrderBookRequest<'_> {
    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/depth";
    type Response = OrderBookResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PriceTickerRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceTickerResponse {
    pub symbol: String,
    pub price: String,
    pub time: u64,
}

impl PublicRequest<Usdm> for PriceTickerRequest<'_> {
    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ticker/price";
    type Response = PriceTickerResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct BookTickerRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerResponse {
    pub symbol: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
    pub time: u64,
}

impl PublicRequest<Usdm> for BookTickerRequest<'_> {
    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ticker/bookTicker";
    type Response = BookTickerResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentAggTradesRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AggTradeResponse {
    #[serde(rename = "a")]
    pub id: u64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub qty: String,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub timestamp: u64,
    #[serde(rename = "m")]
    pub buyer_is_maker: bool,
}

impl PublicRequest<Usdm> for RecentAggTradesRequest<'_> {
    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/aggTrades";
    type Response = Vec<AggTradeResponse>;
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CreateListenKeyRequest {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateListenKeyResponse {
    pub listen_key: String,
}

impl KeyedRequest<Usdm> for CreateListenKeyRequest {
    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/listenKey";
    type Response = CreateListenKeyResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct KeepAliveListenKeyRequest {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepAliveListenKeyResponse {
    pub listen_key: String,
}

impl KeyedRequest<Usdm> for KeepAliveListenKeyRequest {
    const METHOD: Method = Method::PUT;
    const ENDPOINT: &'static str = "/fapi/v1/listenKey";
    type Response = KeepAliveListenKeyResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CloseListenKeyRequest {}

#[derive(Debug, Clone, Deserialize)]
pub struct CloseListenKeyResponse {}

impl KeyedRequest<Usdm> for CloseListenKeyRequest {
    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/fapi/v1/listenKey";
    type Response = CloseListenKeyResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePositionModeRequest<'a> {
    pub dual_side_position: &'a str, // true or false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>, // <= 60_000
    pub timestamp: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePositionModeResponse {}

impl SignedRequest<Usdm> for ChangePositionModeRequest<'_> {
    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/positionSide/dual";
    type Response = ChangePositionModeResponse;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }
    fn recv_window(&self) -> u64 {
        self.recv_window.unwrap_or(5000)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest<'a> {
    pub symbol: &'a str,
    pub side: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<&'a str>,
    pub r#type: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_position: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_protect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_till_date: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>, // <= 60_000
    pub timestamp: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResponse {
    pub client_order_id: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub executed_qty: String,
    pub order_id: u64,
    pub avg_price: String,
    pub orig_qty: String,
    pub price: String,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    pub stop_price: String,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    pub r#type: String,
    pub orig_type: String,
    pub activate_price: Option<String>,
    pub price_rate: Option<String>,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
    pub self_trade_prevention_mode: String,
    pub good_till_date: u64,
}

impl SignedRequest<Usdm> for NewOrderRequest<'_> {
    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/order";
    type Response = NewOrderResponse;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }
    fn recv_window(&self) -> u64 {
        self.recv_window.unwrap_or(5000)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>, // <= 60_000
    pub timestamp: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub client_order_id: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub executed_qty: String,
    pub order_id: u64,
    pub orig_qty: String,
    pub orig_type: String,
    pub price: String,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    pub stop_price: String,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    pub r#type: String,
    pub activate_price: Option<String>,
    pub price_rate: Option<String>,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
    pub self_trade_prevention_mode: String,
    pub good_till_date: u64,
}

impl SignedRequest<Usdm> for CancelOrderRequest<'_> {
    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/fapi/v1/order";
    type Response = CancelOrderResponse;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }
    fn recv_window(&self) -> u64 {
        self.recv_window.unwrap_or(5000)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCommissionRateRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>, // <= 60_000
    pub timestamp: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCommissionRateResponse {
    pub symbol: String,
    pub maker_commission_rate: String,
    pub taker_commission_rate: String,
}

impl SignedRequest<Usdm> for UserCommissionRateRequest<'_> {
    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/commissionRate";
    type Response = UserCommissionRateResponse;

    fn timestamp(&self) -> u64 {
        self.timestamp
    }
    fn recv_window(&self) -> u64 {
        self.recv_window.unwrap_or(5000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::BinanceClient;

    #[tokio::test]
    async fn test_exchange_info_request() {
        let client = BinanceClient::usdm();
        let req = ExchangeInfoRequest;
        let res = client.request(&req).await.unwrap();
        assert!(res.status.is_success());
    }

    #[tokio::test]
    async fn test_order_book_request() {
        let client = BinanceClient::usdm();
        let req = OrderBookRequest {
            symbol: "BTCUSDT",
            limit: Some(5),
        };
        let res = client.request(&req).await.unwrap();
        assert!(res.status.is_success());
    }

    #[tokio::test]
    async fn test_price_ticker_request() {
        let client = BinanceClient::usdm();
        let req = PriceTickerRequest { symbol: "BTCUSDT" };
        let res = client.request(&req).await.unwrap();
        assert!(res.status.is_success());
    }

    #[tokio::test]
    async fn test_book_ticker_request() {
        let client = BinanceClient::usdm();
        let req = BookTickerRequest { symbol: "BTCUSDT" };
        let res = client.request(&req).await.unwrap();
        assert!(res.status.is_success());
    }
}
