use crate::client::Product;
use super::Request;
use serde::{Deserialize, Serialize};
use reqwest::Method;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ExchangeInfoRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoResponse {
    pub exchange_filters: Vec<ExchangeFilter>,
    pub rate_limits: Vec<RateLimit>,
    pub server_time: usize,
    pub symbols: Vec<Market>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ExchangeFilter {
    // No info about this on binance api docs
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: usize,
    pub limit: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub symbol: String,
    pub pair: String,
    pub contract_type: Option<String>,
    pub delivery_date: usize,
    pub onboard_date: usize,
    pub status: String,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub margin_asset: String,
    pub price_precision: usize,
    pub quantity_precision: usize,
    pub base_asset_precision: usize,
    pub quote_precision: usize,
    pub underlying_type: String, // No info on this
    pub underlying_sub_type: Vec<String>, // No info on this
    pub settle_plan: usize, // No info on this
    pub trigger_protect: String,
    pub filters: Vec<SymbolFilter>,
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
        limit: usize,
    },
    MaxNumAlgoOrders {
        limit: usize,
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

impl Request for ExchangeInfoRequest {
    const PRODUCT: Product = Product::UsdMFutures;
    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/exchangeInfo";
    type Response = ExchangeInfoResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct OrderBookRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub last_update_id: usize,
    #[serde(rename = "E")]
    pub message_output_time: usize,
    #[serde(rename = "T")]
    pub transaction_time: usize,
    pub bids: Vec<BookLevel>,
    pub asks: Vec<BookLevel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookLevel {
    pub price: String,
    pub qty: String,
}

impl<'a> Request for OrderBookRequest<'a> {
    const PRODUCT: Product = Product::UsdMFutures;
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
    pub time: usize,
}

impl<'a> Request for PriceTickerRequest<'a> {
    const PRODUCT: Product = Product::UsdMFutures;
    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ticker/price";
    type Response = PriceTickerResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct BookTickerRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookTickerResponse {
    pub symbol: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
    pub time: usize,
}

impl<'a> Request for BookTickerRequest<'a> {
    const PRODUCT: Product = Product::UsdMFutures;
    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ticker/bookTicker";
    type Response = BookTickerResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CreateListenKeyRequest { }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateListenKeyResponse {
    pub listen_key: String,
}

impl Request for CreateListenKeyRequest {
    const PRODUCT: Product = Product::UsdMFutures;
    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/listenKey";
    const KEYED: bool = true;
    type Response = CreateListenKeyResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct KeepAliveListenKeyRequest { }

#[derive(Debug, Clone, Deserialize)]
pub struct KeepAliveListenKeyResponse { }

impl Request for KeepAliveListenKeyRequest {
    const PRODUCT: Product = Product::UsdMFutures;
    const METHOD: Method = Method::PUT;
    const ENDPOINT: &'static str = "/fapi/v1/listenKey";
    const KEYED: bool = true;
    type Response = KeepAliveListenKeyResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePositionModeRequest<'a> {
    pub dual_side_position: &'a str, // true or false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePositionModeResponse { }

impl<'a> Request for ChangePositionModeRequest<'a> {
    const PRODUCT: Product = Product::UsdMFutures;
    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/positionSide/dual";
    const KEYED: bool = true;
    const SIGNED: bool = true;
    type Response = ChangePositionModeResponse;
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
    pub good_till_date: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResponse {
    pub client_order_id: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub executed_qty: String,
    pub order_id: usize,
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
    pub update_time: usize,
    pub working_type: String,
    pub price_protect: bool,
    pub self_trade_prevention_mode: String,
    pub good_till_date: usize,
}

impl<'a> Request for NewOrderRequest<'a> {
    const PRODUCT: Product = Product::UsdMFutures;
    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/order";
    const KEYED: bool = true;
    const SIGNED: bool = true;
    type Response = NewOrderResponse;
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub client_order_id: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub executed_qty: String,
    pub order_id: usize,
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
    pub update_time: usize,
    pub working_type: String,
    pub price_protect: bool,
    pub self_trade_prevention_mode: String,
    pub good_till_date: usize,
}

impl<'a> Request for CancelOrderRequest<'a> {
    const PRODUCT: Product = Product::UsdMFutures;
    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/fapi/v1/order";
    const KEYED: bool = true;
    const SIGNED: bool = true;
    type Response = CancelOrderResponse;
}
