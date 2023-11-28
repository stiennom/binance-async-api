use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};
use url::Url;
use reqwest::Method;

use crate::{
    streams::BinanceStream,
    requests::{
        Request,
        ApiKeyHeaderRequest,
        SignedRequest,
    },
    model::{
        Side,
        BookLevel,
        SelfTradePreventionMode,
        utils::{
            treat_error_as_none,
            serialize_array_as_str,
            serialize_opt_array_as_str,
        },
    },
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
    ExpiredInMatch,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    GoodTilCanceled,
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    #[serde(rename = "FOK")]
    FillOrKill,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum KlineInterval {
    #[serde(rename = "1s")]
    s1,
    #[serde(rename = "1s")]
    m1,
    #[serde(rename = "1s")]
    m3,
    #[serde(rename = "1s")]
    m5,
    #[serde(rename = "1s")]
    m15,
    #[serde(rename = "1s")]
    m30,
    #[serde(rename = "1s")]
    h1,
    #[serde(rename = "1s")]
    h2,
    #[serde(rename = "1s")]
    h4,
    #[serde(rename = "1s")]
    h6,
    #[serde(rename = "1s")]
    h8,
    #[serde(rename = "1s")]
    h12,
    #[serde(rename = "1s")]
    d1,
    #[serde(rename = "1s")]
    d3,
    #[serde(rename = "1s")]
    w1,
    #[serde(rename = "1s")]
    M1,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContingencyType {
    Oco,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ListStatusType {
    Response,
    ExecStarted,
    AllDone,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ListOrderStatus {
    Executing,
    ExecStarted,
    AllDone,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(alias = "s")]
    pub symbol: String,
    #[serde(rename = "orderId", alias = "i")]
    pub id: usize,
    #[serde(alias = "c")]
    pub client_order_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    #[serde(alias = "a")]
    pub asset: String,
    #[serde(alias = "f")]
    pub free: String,
    #[serde(alias = "l")]
    pub locked: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AggTradeEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub id: usize,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub qty: String,
    #[serde(rename = "f")]
    pub first_trade_id: usize,
    #[serde(rename = "l")]
    pub last_trade_id: usize,
    #[serde(rename = "T")]
    pub trade_time: usize,
    #[serde(rename = "m")]
    pub buyer_is_maker: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct AggTradeStream<'a> {
    pub symbol: &'a str,
}

impl<'a> BinanceStream for AggTradeStream<'a> {
    type Event = AggTradeEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/{}@aggTrade", ws_base_url, self.symbol.to_lowercase()))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TradeEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub id: usize,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub qty: String,
    #[serde(rename = "b")]
    pub buyer_order_id: usize,
    #[serde(rename = "a")]
    pub seller_order_id: usize,
    #[serde(rename = "T")]
    pub trade_time: usize,
    #[serde(rename = "m")]
    pub buyer_is_maker: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct TradeStream<'a> {
    pub symbol: &'a str,
}

impl<'a> BinanceStream for TradeStream<'a> {
    type Event = TradeEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/{}@trade", ws_base_url, self.symbol.to_lowercase()))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct KlineUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline_update: KlineUpdate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KlineUpdate {
    #[serde(rename = "t")]
    pub open_time: usize,
    #[serde(rename = "T")]
    pub close_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: KlineInterval,
    #[serde(rename = "f")]
    pub first_trade_id: usize,
    #[serde(rename = "L")]
    pub last_trade_id: usize,
    #[serde(rename = "o")]
    pub open_price: String,
    #[serde(rename = "c")]
    pub close_price: String,
    #[serde(rename = "h")]
    pub high_price: String,
    #[serde(rename = "l")]
    pub low_price: String,
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    #[serde(rename = "n")]
    pub trade_count: usize,
    #[serde(rename = "x")]
    pub is_closed: bool,
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: String,
    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: String,
}

#[derive(Debug, Clone, Copy)]
pub struct KlineUpdateStream<'a> {
    pub symbol: &'a str,
    pub interval: KlineInterval,
}

impl<'a> BinanceStream for KlineUpdateStream<'a> {
    type Event = KlineUpdateEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        let interval_str = match self.interval {
            KlineInterval::s1 => "1s",
            KlineInterval::m1 => "1m",
            KlineInterval::m3 => "3m",
            KlineInterval::m5 => "5m",
            KlineInterval::m15 => "15m",
            KlineInterval::m30 => "30m",
            KlineInterval::h1 => "1h",
            KlineInterval::h2 => "2h",
            KlineInterval::h4 => "4h",
            KlineInterval::h6 => "6h",
            KlineInterval::h8 => "8h",
            KlineInterval::h12 => "12h",
            KlineInterval::d1 => "1d",
            KlineInterval::d3 => "3d",
            KlineInterval::w1 => "1w",
            KlineInterval::M1 => "1M",
        };
        Url::from_str(&format!("{}/ws/{}@kline_{}", ws_base_url, self.symbol.to_lowercase(), interval_str))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MiniTickerEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub close_price: String,
    #[serde(rename = "o")]
    pub open_price: String,
    #[serde(rename = "h")]
    pub high_price: String,
    #[serde(rename = "l")]
    pub low_price: String,
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
}

#[derive(Debug, Clone, Copy)]
pub struct MiniTickerStream<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct AllMiniTickersStream;

impl<'a> BinanceStream for MiniTickerStream<'a> {
    type Event = MiniTickerEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/{}@miniTicker", ws_base_url, self.symbol.to_lowercase()))
    }
}

impl BinanceStream for AllMiniTickersStream {
    type Event = Vec<MiniTickerEvent>;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/!miniTicker@arr", ws_base_url))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TickerEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price_change: String,
    #[serde(rename = "P")]
    pub price_change_percent: String,
    #[serde(rename = "w")]
    pub volume_weighted_avg_price: String,
    #[serde(rename = "x")]
    pub previous_trade_price: String,
    #[serde(rename = "c")]
    pub last_price: String,
    #[serde(rename = "Q")]
    pub last_qty: String,
    #[serde(rename = "b")]
    pub best_bid_price: String,
    #[serde(rename = "B")]
    pub best_bid_qty: String,
    #[serde(rename = "a")]
    pub best_ask_price: String,
    #[serde(rename = "A")]
    pub best_ask_qty: String,
    #[serde(rename = "o")]
    pub open_price: String,
    #[serde(rename = "h")]
    pub high_price: String,
    #[serde(rename = "l")]
    pub low_price: String,
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
    #[serde(rename = "O")]
    pub open_time: usize,
    #[serde(rename = "C")]
    pub close_time: usize,
    #[serde(rename = "F")]
    pub first_trade_id: usize,
    #[serde(rename = "L")]
    pub last_trade_id: usize,
    #[serde(rename = "n")]
    pub trade_count: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct TickerStream<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct AllTickersStream;

impl<'a> BinanceStream for TickerStream<'a> {
    type Event = TickerEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/{}@ticker", ws_base_url, self.symbol.to_lowercase()))
    }
}

impl BinanceStream for AllTickersStream {
    type Event = Vec<TickerEvent>;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/!ticker@arr", ws_base_url))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WindowSizedTickerEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price_change: String,
    #[serde(rename = "P")]
    pub price_change_percent: String,
    #[serde(rename = "o")]
    pub open_price: String,
    #[serde(rename = "h")]
    pub high_price: String,
    #[serde(rename = "l")]
    pub low_price: String,
    #[serde(rename = "c")]
    pub close_price: String,
    #[serde(rename = "w")]
    pub volume_weighted_avg_price: String,
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
    #[serde(rename = "O")]
    pub open_time: usize,
    #[serde(rename = "C")]
    pub close_time: usize,
    #[serde(rename = "F")]
    pub first_trade_id: usize,
    #[serde(rename = "L")]
    pub last_trade_id: usize,
    #[serde(rename = "n")]
    pub trade_count: usize,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum WindowSizedTickerWindowSize {
    h1,
    h4,
    d1,
}

#[derive(Debug, Clone, Copy)]
pub struct WindowSizedTickerStream<'a> {
    pub symbol: &'a str,
    pub window_size: WindowSizedTickerWindowSize,
}

#[derive(Debug, Clone, Copy)]
pub struct AllWindowSizedTickersStream {
    pub window_size: WindowSizedTickerWindowSize,
}

impl<'a> BinanceStream for WindowSizedTickerStream<'a> {
    type Event = WindowSizedTickerEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        let window_size_str = match self.window_size {
            WindowSizedTickerWindowSize::h1 => "1h",
            WindowSizedTickerWindowSize::h4 => "4h",
            WindowSizedTickerWindowSize::d1 => "1d",
        };
        Url::from_str(&format!("{}/ws/{}@ticker_{}", ws_base_url, self.symbol.to_lowercase(), window_size_str))
    }
}

impl BinanceStream for AllWindowSizedTickersStream {
    type Event = Vec<WindowSizedTickerEvent>;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        let window_size_str = match self.window_size {
            WindowSizedTickerWindowSize::h1 => "1h",
            WindowSizedTickerWindowSize::h4 => "4h",
            WindowSizedTickerWindowSize::d1 => "1d",
        };
        Url::from_str(&format!("{}/ws/!ticker_{}@arr", ws_base_url, window_size_str))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookTickerEvent {
    #[serde(rename = "u")]
    pub order_book_update_id: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b")]
    pub best_bid_price: String,
    #[serde(rename = "B")]
    pub best_bid_qty: String,
    #[serde(rename = "a")]
    pub best_ask_price: String,
    #[serde(rename = "A")]
    pub best_ask_qty: String,
}

#[derive(Debug, Clone, Copy)]
pub struct BookTickerStream<'a> {
    pub symbol: &'a str,
}

impl<'a> BinanceStream for BookTickerStream<'a> {
    type Event = BookTickerEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/{}@bookTicker", ws_base_url, self.symbol.to_lowercase()))
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialDepthEvent {
    pub last_update_id: usize,
    pub bids: Vec<BookLevel>,
    pub asks: Vec<BookLevel>,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum PartialDepthStreamUpdateSpeed {
    ms100,
    ms1000,
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
#[allow(non_camel_case_types)]
pub enum PartialDepthStreamLevels {
    l5 = 5,
    l10 = 10,
    l20 = 20,
}

#[derive(Debug, Clone, Copy)]
pub struct PartialDepthStream<'a> {
    pub symbol: &'a str,
    pub update_speed: PartialDepthStreamUpdateSpeed,
    pub levels: PartialDepthStreamLevels,
}

impl<'a> BinanceStream for PartialDepthStream<'a> {
    type Event = PartialDepthEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        let update_speed_str = match self.update_speed {
            PartialDepthStreamUpdateSpeed::ms100 => "@100ms",
            PartialDepthStreamUpdateSpeed::ms1000 => "",
        };
        Url::from_str(&format!("{}/ws/{}@depth{}{}", ws_base_url, self.symbol.to_lowercase(), self.levels as usize, update_speed_str))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiffDepthEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: usize,
    #[serde(rename = "u")]
    pub final_update_id: usize,
    #[serde(rename = "b")]
    pub bid_updates: Vec<BookLevel>,
    #[serde(rename = "a")]
    pub ask_updates: Vec<BookLevel>,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum DiffDepthStreamUpdateSpeed {
    ms100,
    ms1000,
}

#[derive(Debug, Clone, Copy)]
pub struct DiffDepthStream<'a> {
    pub symbol: &'a str,
    pub update_speed: DiffDepthStreamUpdateSpeed,
}

impl<'a> BinanceStream for DiffDepthStream<'a> {
    type Event = DiffDepthEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        let update_speed_str = match self.update_speed {
            DiffDepthStreamUpdateSpeed::ms100 => "@100ms",
            DiffDepthStreamUpdateSpeed::ms1000 => "",
        };
        Url::from_str(&format!("{}/ws/{}@depth{}", ws_base_url, self.symbol.to_lowercase(), update_speed_str))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "u")]
    pub last_account_update_time: usize,
    #[serde(rename = "B")]
    pub balance_updates: Vec<Balance>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "d")]
    pub balance_delta: String,
    #[serde(rename = "T")]
    pub clear_time: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub enum OrderExecutionType {
    New,
    Canceled,
    Replaced, // Currently unused
    Rejected,
    Trade,
    Expired,
    TradePrevention,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: Side,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q")]
    pub order_qty: String,
    #[serde(rename = "p")]
    pub order_price: String,
    #[serde(rename = "P")]
    pub stop_price: String,
    #[serde(rename = "F")]
    pub iceberg_qty: String,
    #[serde(rename = "g", deserialize_with = "treat_error_as_none")]
    pub order_list_id: Option<usize>,
    #[serde(rename = "C")]
    pub original_client_order_id: String,
    #[serde(rename = "x")]
    pub current_order_execution_type: OrderExecutionType,
    #[serde(rename = "X")]
    pub current_order_status: OrderStatus,
    #[serde(rename = "i")]
    pub order_id: usize,
    #[serde(rename = "l")]
    pub last_filled_qty: String,
    #[serde(rename = "z")]
    pub cumulative_filled_qty: String,
    #[serde(rename = "L")]
    pub last_filled_price: String,
    #[serde(rename = "n")]
    pub commission_amount: Option<String>,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "T")]
    pub transaction_time: usize,
    #[serde(rename = "t", deserialize_with = "treat_error_as_none")]
    pub trade_id: Option<usize>,
    #[serde(rename = "w")]
    pub is_order_on_book: bool,
    #[serde(rename = "m")]
    pub is_trade_maker: bool,
    #[serde(rename = "O")]
    pub order_creation_time: usize,
    #[serde(rename = "Z")]
    pub cumulative_quote_asset_transacted_qty: String,
    #[serde(rename = "Y")]
    pub last_quote_asset_transacted_qty: String,
    #[serde(rename = "Q")]
    pub order_quote_qty: String,
    #[serde(rename = "W")]
    pub order_working_time: Option<usize>,
    #[serde(rename = "V")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    #[serde(rename = "d")]
    pub trailing_delta: Option<usize>,
    #[serde(rename = "D")]
    pub trailing_time: Option<usize>,
    #[serde(rename = "j")]
    pub strategy_id: Option<usize>,
    #[serde(rename = "J")]
    pub strategy_type: Option<usize>,
    #[serde(rename = "v")]
    pub prevented_match_id: Option<usize>,
    #[serde(rename = "A")]
    pub prevented_qty: Option<String>,
    #[serde(rename = "B")]
    pub last_prevented_qty: Option<String>,
    #[serde(rename = "u")]
    pub trade_group_id: Option<usize>,
    #[serde(rename = "U")]
    pub counter_order_id: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListOrderUpdate {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "g")]
    pub order_list_id: usize,
    #[serde(rename = "c")]
    pub contingency_type: ContingencyType,
    #[serde(rename = "l")]
    pub list_status_type: ListStatusType,
    #[serde(rename = "L")]
    pub list_order_status: ListOrderStatus,
    #[serde(rename = "C")]
    pub list_client_order_id: String,
    #[serde(rename = "T")]
    pub transaction_time: usize,
    #[serde(rename = "O")]
    pub orders: Vec<Order>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "e")]
pub enum UserStreamEvent {
    #[serde(rename = "outboundAccountPosition")]
    AccountUpdate(AccountUpdateEvent),
    #[serde(rename = "balanceUpdate")]
    BalanceUpdate(BalanceUpdateEvent),
    #[serde(rename = "executionReport")]
    OrderUpdate(OrderUpdateEvent),
    #[serde(rename = "listStatus")]
    ListOrderUpdate(ListOrderUpdate),
}

#[derive(Debug, Clone, Copy)]
pub struct UserStream<'a> {
    pub listen_key: &'a str,
}

impl<'a> BinanceStream for UserStream<'a> {
    type Event = UserStreamEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/{}", ws_base_url, self.listen_key))
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PingRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct PingResponse { }

impl Request for PingRequest {
    type Response = PingResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ping";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ServerTimeRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    pub server_time: usize,
}

impl Request for ServerTimeRequest {
    type Response = ServerTimeResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/time";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ExchangeInfoRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_array_as_str")]
    pub symbols: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_array_as_str")]
    pub permissions: Option<&'a [Permission]>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoResponse {
    pub server_time: usize,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<ExchangeFilter>,
    #[serde(rename = "symbols")]
    pub markets: Vec<Market>,
}

impl<'a> Request for ExchangeInfoRequest<'a> {
    type Response = ExchangeInfoResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/exchangeInfo";
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: RateLimitType,
    pub interval: RateLimitInterval,
    pub interval_num: usize,
    pub limit: usize,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
    RawRequests,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Second,
    Minute,
    Day,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "filterType")]
pub enum ExchangeFilter {
    #[serde(rename = "EXCHANGE_MAX_NUM_ORDERS")]
    MaxNumOrders {
        #[serde(rename = "maxNumOrders")]
        max: usize,
    },
    #[serde(rename = "EXCHANGE_MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders {
        #[serde(rename = "maxNumAlgoOrders")]
        max: usize,
    },
    #[serde(rename = "EXCHANGE_MAX_NUM_ICEBERG_ORDERS")]
    MaxNumIcebergOrders {
        #[serde(rename = "maxNumIcebergOrders")]
        max: usize,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub symbol: String,
    pub status: SymbolStatus,
    pub base_asset: String,
    pub base_asset_precision: usize,
    pub quote_asset: String,
    pub quote_asset_precision: usize,
    pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub allow_trailing_stop: bool,
    pub cancel_replace_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<SymbolFilter>,
    pub permissions: Vec<Permission>,
    pub default_self_trade_prevention_mode: SelfTradePreventionMode,
    pub allowed_self_trade_prevention_modes: Vec<SelfTradePreventionMode>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolStatus {
    PreTrading,
    Trading,
    PostTrading,
    EndOfDay,
    Halt,
    AuctionMatch,
    Break,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolFilter {
    #[serde(rename = "PRICE_FILTER", rename_all = "camelCase")]
    Price {
        #[serde(rename = "minPrice")]
        min: String,
        #[serde(rename = "maxPrice")]
        max: String,
        tick_size: String,
    },
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: String,
        multiplier_down: String,
        avg_price_mins: usize,
    },
    #[serde(rename_all = "camelCase")]
    PercentPriceBySide {
        bid_multiplier_up: String,
        bid_multiplier_down: String,
        ask_multiplier_up: String,
        ask_multiplier_down: String,
        avg_price_mins: usize,
    },
    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename_all = "camelCase")]
    MinNotional {
        #[serde(rename = "minNotional")]
        min: String,
        apply_to_market: bool,
        avg_price_mins: usize,
    },
    #[serde(rename_all = "camelCase")]
    Notional {
        #[serde(rename = "minNotional")]
        min: String,
        apply_min_to_market: bool,
        #[serde(rename = "maxNotional")]
        max: String,
        apply_max_to_market: bool,
        avg_price_mins: usize,
    },
    #[serde(rename_all = "camelCase")]
    IcebergParts {
        limit: usize,
    },
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    MaxNumOrders {
        #[serde(rename = "maxNumOrders")]
        max: usize,
    },
    MaxNumAlgoOrders {
        #[serde(rename = "maxNumAlgoOrders")]
        max: usize,
    },
    MaxNumIcebergOrders {
        #[serde(rename = "maxNumIcebergOrders")]
        max: usize,
    },
    MaxPosition {
        #[serde(rename = "maxPosition")]
        max: String
    },
    #[serde(rename_all = "camelCase")]
    TrailingDelta {
        min_trailing_above_delta: usize,
        max_trailing_above_delta: usize,
        min_trailing_below_delta: usize,
        max_trailing_below_delta: usize,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Permission {
    Spot,
    Margin,
    Leveraged,
    #[serde(rename = "TRD_GRP_002")]
    TrdGrp002,
    #[serde(rename = "TRD_GRP_003")]
    TrdGrp003,
    #[serde(rename = "TRD_GRP_004")]
    TrdGrp004,
    #[serde(rename = "TRD_GRP_005")]
    TrdGrp005,
    #[serde(rename = "TRD_GRP_006")]
    TrdGrp006,
    #[serde(rename = "TRD_GRP_007")]
    TrdGrp007,
    #[serde(rename = "TRD_GRP_008")]
    TrdGrp008,
    #[serde(rename = "TRD_GRP_009")]
    TrdGrp009,
    #[serde(rename = "TRD_GRP_010")]
    TrdGrp010,
    #[serde(rename = "TRD_GRP_011")]
    TrdGrp011,
    #[serde(rename = "TRD_GRP_012")]
    TrdGrp012,
    #[serde(rename = "TRD_GRP_013")]
    TrdGrp013,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct OrderBookRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // max 5000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub last_update_id: usize,
    pub bids: Vec<BookLevel>,
    pub asks: Vec<BookLevel>,
}

impl<'a> Request for OrderBookRequest<'a> {
    type Response = OrderBookResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/depth";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct RecentTradesRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1000
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OldTradesRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeResponse {
    pub id: usize,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub time: usize,
    pub is_buyer_maker: bool,
}

impl<'a> Request for RecentTradesRequest<'a> {
    type Response = Vec<TradeResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/trades";
}

impl<'a> Request for OldTradesRequest<'a> {
    type Response = Vec<TradeResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/historicalTrades";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggTradesRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1000
}

#[derive(Debug, Clone, Deserialize)]
pub struct AggTradeResponse {
    #[serde(rename = "a")]
    pub id: usize,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub qty: String,
    #[serde(rename = "f")]
    pub first_trade_id: usize,
    #[serde(rename = "l")]
    pub last_trade_id: usize,
    #[serde(rename = "T")]
    pub time: usize,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}

impl<'a> Request for AggTradesRequest<'a> {
    type Response = Vec<AggTradeResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/aggTrades";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KlinesRequest<'a> {
    pub symbol: &'a str,
    pub interval: KlineInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1000
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UiKlinesRequest<'a> {
    pub symbol: &'a str,
    pub interval: KlineInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1000
}

#[derive(Debug, Clone, Deserialize)]
pub struct KlineResponse {
    pub open_time: usize,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub base_asset_volume: String,
    pub close_time: usize,
    pub quote_asset_volmue: String,
    pub trade_count: usize,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
}

impl<'a> Request for KlinesRequest<'a> {
    type Response = Vec<KlineResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/klines";
}

impl<'a> Request for UiKlinesRequest<'a> {
    type Response = Vec<KlineResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/uiKlines";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AvgPriceRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AvgPriceResponse {
    pub mins: usize,
    pub price: String,
}

impl<'a> Request for AvgPriceRequest<'a> {
    type Response = AvgPriceResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/avgPrice";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TickerRequest<'a> {
    pub symbol: &'a str,
    #[serde(rename = "type")]
    pub ticker_type: Option<TickerType>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TickersRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_array_as_str")]
    pub symbols: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub ticker_type: Option<TickerType>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TickerType {
    Full,
    Mini,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTickerResponse {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    #[serde(rename = "weightedAvgPrice")]
    pub volume_weighted_avg_price: String,
    #[serde(rename = "prevClosePrice")]
    pub previous_trade_price: String,
    pub last_price: String,
    pub last_qty: String,
    #[serde(rename = "bidPrice")]
    pub best_bid_price: String,
    #[serde(rename = "bidQty")]
    pub best_bid_qty: String,
    #[serde(rename = "bidQty")]
    pub best_ask_price: String,
    #[serde(rename = "askQty")]
    pub best_ask_qty: String,
    pub open_price: String,
    pub hight_price: String,
    pub low_price: String,
    #[serde(rename = "volume")]
    pub base_asset_volume: String,
    #[serde(rename = "quoteVolume")]
    pub quote_asset_volume: String,
    pub open_time: usize,
    pub close_time: usize,
    #[serde(rename = "firstId")]
    pub first_trade_id: usize,
    #[serde(rename = "lastId")]
    pub last_trade_id: usize,
    #[serde(rename = "count")]
    pub trade_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniTickerResponse {
    pub symbol: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub last_price: String,
    #[serde(rename = "volume")]
    pub base_asset_volume: String,
    #[serde(rename = "quoteVolume")]
    pub quote_asset_volume: String,
    pub open_time: usize,
    pub close_time: usize,
    #[serde(rename = "firstId")]
    pub first_trade_id: usize,
    #[serde(rename = "lastId")]
    pub last_trade_id: usize,
    #[serde(rename = "count")]
    pub trade_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TickerResponse {
    Full(FullTickerResponse),
    Mini(MiniTickerResponse),
}

impl<'a> Request for TickerRequest<'a> {
    type Response = TickerResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ticker/24hr";
}

impl<'a> Request for TickersRequest<'a> {
    type Response = Vec<TickerResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ticker/24hr";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PriceTickerRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PriceTickersRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_array_as_str")]
    pub symbols: Option<&'a [&'a str]>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceTickerResponse {
    pub symbol: String,
    pub price: String,
}

impl<'a> Request for PriceTickerRequest<'a> {
    type Response = PriceTickerResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ticker/price";
}

impl<'a> Request for PriceTickersRequest<'a> {
    type Response = Vec<PriceTickerResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ticker/price";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct BookTickerRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct BookTickersRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_array_as_str")]
    pub symbols: Option<&'a [&'a str]>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookTickerResponse {
    pub symbol: String,
    #[serde(rename = "bidPrice")]
    pub best_bid_price: String,
    #[serde(rename = "bidQty")]
    pub best_bid_qty: String,
    #[serde(rename = "askPrice")]
    pub best_ask_price: String,
    #[serde(rename = "askQty")]
    pub best_ask_qty: String,
}

impl<'a> Request for BookTickerRequest<'a> {
    type Response = BookTickerResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ticker/bookTicker";
}

impl<'a> Request for BookTickersRequest<'a> {
    type Response = Vec<BookTickerResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ticker/bookTicker";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowSizedTickerRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_size: Option<&'a str>,
    pub window_sized_ticker_type: Option<WindowSizedTickerType>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowSizedTickersRequest<'a> {
    #[serde(serialize_with = "serialize_array_as_str")]
    pub symbols: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_size: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_sized_ticker_type: Option<WindowSizedTickerType>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WindowSizedTickerType {
    Full,
    Mini,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FullWindowSizedTickerResponse {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    #[serde(rename = "weightedAvgPrice")]
    pub volume_weighted_avg_price: String,
    pub open_price: String,
    pub hight_price: String,
    pub low_price: String,
    pub last_price: String,
    #[serde(rename = "volume")]
    pub base_asset_volume: String,
    #[serde(rename = "quoteVolume")]
    pub quote_asset_volume: String,
    pub open_time: usize,
    pub close_time: usize,
    #[serde(rename = "firstId")]
    pub first_trade_id: usize,
    #[serde(rename = "lastId")]
    pub last_trade_id: usize,
    #[serde(rename = "count")]
    pub trade_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MiniWindowSizedTickerResponse {
    pub symbol: String,
    pub open_price: String,
    pub hight_price: String,
    pub low_price: String,
    pub last_price: String,
    #[serde(rename = "volume")]
    pub base_asset_volume: String,
    #[serde(rename = "quoteVolume")]
    pub quote_asset_volume: String,
    pub open_time: usize,
    pub close_time: usize,
    #[serde(rename = "firstId")]
    pub first_trade_id: usize,
    #[serde(rename = "lastId")]
    pub last_trade_id: usize,
    #[serde(rename = "count")]
    pub trade_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum WindowSizedTickerResponse {
    Full(FullWindowSizedTickerResponse),
    Mini(MiniWindowSizedTickerResponse),
}

impl<'a> Request for WindowSizedTickerRequest<'a> {
    type Response = WindowSizedTickerResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ticker";
}

impl<'a> Request for WindowSizedTickersRequest<'a> {
    type Response = Vec<WindowSizedTickerResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/ticker";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CreateSpotListenKeyRequest;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CreateMarginListenKeyRequest;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CreateIsolatedMarginListenKeyRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateListenKeyResponse {
    pub listen_key: String,
}

impl ApiKeyHeaderRequest for CreateSpotListenKeyRequest {
    type Response = CreateListenKeyResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/api/v3/userDataStream";
}

impl ApiKeyHeaderRequest for CreateMarginListenKeyRequest {
    type Response = CreateListenKeyResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/sapi/v1/userDataStream";
}

impl<'a> ApiKeyHeaderRequest for CreateIsolatedMarginListenKeyRequest<'a> {
    type Response = CreateListenKeyResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/sapi/v1/userDataStream/isolated";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepAliveSpotListenKeyRequest<'a> {
    pub listen_key: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeepAliveSpotListenKeyResponse { }

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepAliveMarginListenKeyRequest<'a> {
    pub listen_key: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeepAliveMarginListenKeyResponse { }

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepAliveIsolatedMarginListenKeyRequest<'a> {
    pub symbol: &'a str,
    pub listen_key: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeepAliveIsolatedMarginListenKeyResponse { }

impl<'a> ApiKeyHeaderRequest for KeepAliveSpotListenKeyRequest<'a> {
    type Response = KeepAliveSpotListenKeyResponse;

    const METHOD: Method = Method::PUT;
    const ENDPOINT: &'static str = "/api/v3/userDataStream";
}

impl<'a> ApiKeyHeaderRequest for KeepAliveMarginListenKeyRequest<'a> {
    type Response = KeepAliveSpotListenKeyResponse;

    const METHOD: Method = Method::PUT;
    const ENDPOINT: &'static str = "/sapi/v1/userDataStream";
}

impl<'a> ApiKeyHeaderRequest for KeepAliveIsolatedMarginListenKeyRequest<'a> {
    type Response = KeepAliveIsolatedMarginListenKeyResponse;

    const METHOD: Method = Method::PUT;
    const ENDPOINT: &'static str = "/sapi/v1/userDataStream/isolated";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseSpotListenKeyRequest<'a> {
    pub listen_key: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloseSpotListenKeyResponse { }

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseMarginListenKeyRequest<'a> {
    pub listen_key: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloseMarginListenKeyResponse { }

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseIsolatedMarginListenKeyRequest<'a> {
    pub symbol: &'a str,
    pub listen_key: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloseIsolatedMarginListenKeyResponse { }

impl<'a> ApiKeyHeaderRequest for CloseSpotListenKeyRequest<'a> {
    type Response = CloseSpotListenKeyResponse;

    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/api/v3/userDataStream";
}

impl<'a> ApiKeyHeaderRequest for CloseMarginListenKeyRequest<'a> {
    type Response = CloseMarginListenKeyResponse;

    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/sapi/v1/userDataStream";
}

impl<'a> ApiKeyHeaderRequest for CloseIsolatedMarginListenKeyRequest<'a> {
    type Response = CloseIsolatedMarginListenKeyResponse;

    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/sapi/v1/userDataStream/isolated";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestNewOrderRequest<'a> {
    pub symbol: &'a str,
    pub side: Side,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "quantity")]
    pub qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "quoteOrderQty")]
    pub quote_qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "newClientOrderId")]
    pub client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<usize>, // >= 1_000_000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<NewOrderResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestNewOrderResponse { }

impl<'a> SignedRequest for TestNewOrderRequest<'a> {
    type Response = TestNewOrderResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/api/v3/order/test";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NewOrderResponseType {
    Full,
    Result,
    Ack,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest<'a> {
    pub symbol: &'a str,
    pub side: Side,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "quantity")]
    pub qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "quoteOrderQty")]
    pub quote_qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "newClientOrderId")]
    pub client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<usize>, // >= 1_000_000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<NewOrderResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderAckResponse {
    pub symbol: String,
    pub order_id: usize,
    #[serde(deserialize_with = "treat_error_as_none")]
    pub order_list_id: Option<usize>,
    pub client_order_id: String,
    #[serde(rename = "transactTime")]
    pub transaction_time: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResultResponse {
    pub symbol: String,
    pub order_id: usize,
    #[serde(deserialize_with = "treat_error_as_none")]
    pub order_list_id: Option<usize>,
    pub client_order_id: String,
    #[serde(rename = "transactTime")]
    pub transaction_time: usize,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: Side,
    #[serde(deserialize_with = "treat_error_as_none")]
    pub working_time: Option<usize>,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderFullResponse {
    pub symbol: String,
    pub order_id: usize,
    #[serde(deserialize_with = "treat_error_as_none")]
    pub order_list_id: Option<usize>,
    pub client_order_id: String,
    #[serde(rename = "transactTime")]
    pub transaction_time: usize,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: Side,
    #[serde(deserialize_with = "treat_error_as_none")]
    pub working_time: Option<usize>,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub fills: Vec<Fill>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum NewOrderResponse {
    Full(NewOrderFullResponse),
    Result(NewOrderResultResponse),
    Ack(NewOrderAckResponse),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub price: String,
    pub qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub trade_id: usize,
}

impl<'a> SignedRequest for NewOrderRequest<'a> {
    type Response = NewOrderResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/api/v3/order";
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
    pub new_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<CancelRestrictions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelRestrictions {
    OnlyNew,
    OnlyPartiallyFilled,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: usize,
    #[serde(deserialize_with = "treat_error_as_none")]
    pub order_list_id: Option<usize>,
    pub client_order_id: String,
    #[serde(rename = "transactTime")]
    pub transaction_time: usize,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: Side,
    pub stop_price: Option<String>,
    pub iceberg_qty: Option<String>,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub prevented_match_id: Option<usize>,
    #[serde(rename = "preventedQuantity")]
    pub prevented_qty: Option<String>,
    pub strategy_id: Option<usize>,
    pub strategy_type: Option<usize>,
    pub trailing_delta: Option<usize>,
    pub trailing_time: Option<usize>,
}

impl<'a> SignedRequest for CancelOrderRequest<'a> {
    type Response = CancelOrderResponse;

    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/api/v3/order";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CancelOrderOrListOrderResponse {
    CancelOrderResponse(CancelOrderResponse),
    CancelListOrderResponse(CancelListOrderResponse),
}

impl<'a> SignedRequest for CancelAllOrdersRequest<'a> {
    type Response = Vec<CancelOrderOrListOrderResponse>;

    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/api/v3/openOrders";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderRequest<'a> {
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
pub struct QueryOrderResponse {
    pub symbol: String,
    pub order_id: usize,
    #[serde(deserialize_with = "treat_error_as_none")]
    pub order_list_id: Option<usize>,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cummulative_quote_qty: String,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: Side,
    pub stop_price: Option<String>,
    pub iceberg_qty: Option<String>,
    pub time: usize,
    pub update_time: usize,
    pub is_working: bool,
    #[serde(deserialize_with = "treat_error_as_none")]
    pub working_time: Option<usize>,
    pub orig_quote_order_qty: String,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub prevented_match_id: Option<usize>,
    #[serde(rename = "preventedQuantity")]
    pub prevented_qty: Option<String>,
    pub strategy_id: Option<usize>,
    pub strategy_type: Option<usize>,
    pub trailing_delta: Option<usize>,
    pub trailing_time: Option<usize>,
}

impl<'a> SignedRequest for QueryOrderRequest<'a> {
    type Response = QueryOrderResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/order";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceOrderRequest<'a> {
    pub symbol: &'a str,
    pub side: Side,
    pub order_type: OrderType,
    pub cancel_replace_mode: CancelReplaceMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "quantity")]
    pub qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_new_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_orig_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_order_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<NewOrderResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_tarde_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<CancelRestrictions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelReplaceMode {
    StopOnFailure,
    AllowFailure,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceOrderResponse {
    pub cancel_response: CancelOrderResponse,
    pub new_order_response: NewOrderResponse,
}

impl<'a> SignedRequest for CancelReplaceOrderRequest<'a> {
    type Response = CancelReplaceOrderResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/api/v3/cancelReplace";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

impl<'a> SignedRequest for OpenOrdersRequest<'a> {
    type Response = Vec<QueryOrderResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/openOrders";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllOrdersRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none", rename = "orderId")]
    pub from_order_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

impl<'a> SignedRequest for AllOrdersRequest<'a> {
    type Response = Vec<QueryOrderResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/allOrders";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOcoOrderRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<&'a str>,
    pub side: Side,
    #[serde(rename = "quantity")]
    pub qty: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_strategy_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_strategy_type: Option<usize>, // >= 1_000_000
    pub price: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_iceberg_qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_client_order_id: Option<&'a str>,
    pub stop_price: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_strategy_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_strategy_type: Option<usize>, // >= 1_000_000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_limit_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_iceberg_qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_limit_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<NewOrderResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOcoOrderResponse {
    pub order_list_id: usize,
    pub list_status_type: ListStatusType,
    pub list_order_status: ListOrderStatus,
    pub list_client_order_id: String,
    pub transaction_time: usize,
    pub symbol: String,
    pub orders: [Order; 2], 
    pub order_reports: [NewOrderResponse; 2],
}

impl<'a> SignedRequest for NewOcoOrderRequest<'a> {
    type Response = NewOcoOrderResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/api/v3/order/oco";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelListOrderRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_list_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelListOrderResponse {
    pub order_list_id: usize,
    pub contingency_type: ContingencyType,
    pub list_status_type: ListStatusType,
    pub list_order_status: ListOrderStatus,
    pub list_client_order_id: String,
    pub transaction_time: usize,
    pub symbol: String,
    pub orders: Vec<Order>, 
    pub order_reports: Vec<CancelOrderResponse>,
}

impl<'a> SignedRequest for CancelListOrderRequest<'a> {
    type Response = CancelListOrderResponse;

    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/api/v3/orderList";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOrderRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_list_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "origClientOrderId")]
    pub list_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOrderResponse {
    pub order_list_id: usize,
    pub contingency_type: ContingencyType,
    pub list_status_type: ListStatusType,
    pub list_order_status: ListOrderStatus,
    pub list_client_order_id: String,
    pub transaction_time: usize,
    pub symbol: String,
    pub orders: Vec<Order>,
}

impl<'a> SignedRequest for ListOrderRequest<'a> {
    type Response = ListOrderResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/orderList";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllListOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

impl SignedRequest for AllListOrdersRequest {
    type Response = Vec<ListOrderResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/allOrderList";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenListOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

impl SignedRequest for OpenListOrdersRequest {
    type Response = Vec<ListOrderResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/openOrderList";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformationResponse {
    pub commission_rates: CommissionRates,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub brokered: bool,
    pub required_self_trade_prevention: bool,
    pub prevent_sor: bool,
    pub update_time: usize,
    pub balances: Vec<Balance>,
    pub permissions: Vec<Permission>,
    pub uid: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommissionRates {
    pub maker: String,
    pub taker: String,
    pub buyer: String,
    pub seller: String,
}

impl SignedRequest for AccountInformationRequest {
    type Response = AccountInformationResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/account";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTradesRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTradeResponse {
    pub symbol: String,
    pub id: usize,
    pub order_id: usize,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub time: usize,
    pub is_buyer: bool,
    pub is_maker: bool,
}

impl<'a> SignedRequest for MyTradesRequest<'a> {
    type Response = Vec<MyTradeResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/myTrades";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitsOrderUsageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitOrderUsageResponse {
    pub interval: RateLimitInterval,
    pub interval_num: usize,
    pub limit: usize,
    pub count: usize,
}

impl SignedRequest for RateLimitsOrderUsageRequest {
    type Response = Vec<RateLimitOrderUsageResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/rateLimit/order";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyPreventedMatchesRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevented_match_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_prevented_match_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyPreventedMatchResponse {
    pub symbol: String,
    pub prevented_match_id: usize,
    pub taker_order_id: usize,
    pub maker_order_id: usize,
    pub trade_group_id: usize,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub price: String,
    #[serde(rename = "makerPreventedQuantity")]
    pub maker_prevented_qty: String,
    #[serde(rename = "transactTime")]
    pub transaction_time: usize,
}

impl<'a> SignedRequest for MyPreventedMatchesRequest<'a> {
    type Response = Vec<MyPreventedMatchResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/api/v3/myPreventedMatches";
}
