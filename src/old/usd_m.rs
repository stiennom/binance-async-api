use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};
use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};
use url::Url;
use reqwest::Method;

use crate::{
    streams::BinanceStream,
    requests::{
        Request,
        ApiKeyHeaderRequest,
        SignedRequest,
        BinanceErrorMsg,
    },
    model::{
        Side,
        BookLevel,
        SelfTradePreventionMode,
        utils::{
            treat_error_as_none,
            serialize_bool_as_str,
            serialize_opt_bool_as_str,
        },
    },
};


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
pub struct MarkPriceEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub mark_price: String,
    #[serde(rename = "i")]
    pub index_price: String,
    #[serde(rename = "P")]
    pub estimated_settelment_price: String,
    #[serde(rename = "r")]
    pub funding_fee_rate: String,
    #[serde(rename = "T")]
    pub next_funding_time: usize,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum MarkPriceStreamUpdateSpeed {
    ms1000,
    ms3000,
}

#[derive(Debug, Clone, Copy)]
pub struct MarkPriceStream<'a> {
    pub symbol: &'a str,
    pub update_speed: MarkPriceStreamUpdateSpeed,
}

impl<'a> BinanceStream for MarkPriceStream<'a> {
    type Event = MarkPriceEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        let update_speed_str = match self.update_speed {
            MarkPriceStreamUpdateSpeed::ms1000 => "@1s",
            MarkPriceStreamUpdateSpeed::ms3000 => "",
        };
        Url::from_str(&format!("{}/ws/{}@markPrice{}", ws_base_url, self.symbol.to_lowercase(), update_speed_str))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AllMarkPricesStream {
    pub update_speed: MarkPriceStreamUpdateSpeed,
}

impl BinanceStream for AllMarkPricesStream {
    type Event = Vec<MarkPriceEvent>;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        let update_speed_str = match self.update_speed {
            MarkPriceStreamUpdateSpeed::ms1000 => "@1s",
            MarkPriceStreamUpdateSpeed::ms3000 => "",
        };
        Url::from_str(&format!("{}/ws/!markPrice@arr{}", ws_base_url, update_speed_str))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum KlineInterval {
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
    PerpetualDelivering,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContinuousContractKlineUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "ps")]
    pub pair: String,
    #[serde(rename = "ct")]
    pub contract_type: ContractType,
    #[serde(rename = "k")]
    pub kline_update: ContinuousContractKlineUpdate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContinuousContractKlineUpdate {
    #[serde(rename = "t")]
    pub open_time: usize,
    #[serde(rename = "T")]
    pub close_time: usize,
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
pub struct ContinuousContractKlineUpdateStream<'a> {
    pub pair: &'a str,
    pub contract_type: ContractType,
    pub interval: KlineInterval,
}

impl<'a> BinanceStream for ContinuousContractKlineUpdateStream<'a> {
    type Event = ContinuousContractKlineUpdateEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        let interval_str = match self.interval {
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
        let contract_type_str = match self.contract_type {
            ContractType::Perpetual => "perpetual",
            ContractType::CurrentMonth => "current_month",
            ContractType::NextMonth => "next_month",
            ContractType::CurrentQuarter => "current_quarter",
            ContractType::NextQuarter => "next_quarter",
            ContractType::PerpetualDelivering => "perpetual_delivering"
        };
        Url::from_str(&format!("{}/ws/{}_{}@continuousKline_{}", ws_base_url, self.pair.to_lowercase(), contract_type_str, interval_str))
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
    #[serde(rename = "c")]
    pub last_price: String,
    #[serde(rename = "Q")]
    pub last_qty: String,
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
pub struct BookTickerEvent {
    #[serde(rename = "u")]
    pub order_book_update_id: usize,
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "T")]
    pub transaction_time: usize,
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

#[derive(Debug, Clone, Copy)]
pub struct AllBookTickersStream;

impl<'a> BinanceStream for BookTickerStream<'a> {
    type Event = BookTickerEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/{}@bookTicker", ws_base_url, self.symbol.to_lowercase()))
    }
}

impl BinanceStream for AllBookTickersStream {
    type Event = BookTickerEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/!bookTicker", ws_base_url))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    GoodTillCancel,
    #[serde(rename = "IOC")]
    ImmediateOrCancel,
    #[serde(rename = "FOK")]
    FillOrKill,
    #[serde(rename = "GTX")]
    GoodTillCrossing,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LiquidationOrderEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "o")]
    pub liquidation_order: LiquidationOrder,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LiquidationOrder {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: Side,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q")]
    pub orig_qty: String,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "ap")]
    pub avg_price: String,
    #[serde(rename = "X")]
    pub status: OrderStatus,
    #[serde(rename = "l")]
    pub last_filled_qty: String,
    #[serde(rename = "z")]
    pub cumulative_filled_qty: String,
    #[serde(rename = "T")]
    pub order_trade_time: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct LiquidationOrderStream<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct AllLiquidationOrdersStream;

impl<'a> BinanceStream for LiquidationOrderStream<'a> {
    type Event = LiquidationOrderEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/{}@forceOrder", ws_base_url, self.symbol.to_lowercase()))
    }
}

impl BinanceStream for AllLiquidationOrdersStream {
    type Event = LiquidationOrderEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/!forceOrder@arr", ws_base_url))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PartialDepthEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "T")]
    pub transaction_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: usize,
    #[serde(rename = "u")]
    pub final_update_id: usize,
    #[serde(rename = "pu")]
    pub last_event_final_update_id: usize,
    #[serde(rename = "b")]
    pub bids: Vec<BookLevel>,
    #[serde(rename = "a")]
    pub asks: Vec<BookLevel>,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum PartialDepthStreamUpdateSpeed {
    ms100,
    ms250,
    ms500,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum PartialDepthStreamLevels {
    l5,
    l10,
    l20,
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
        let levels_str = match self.levels {
            PartialDepthStreamLevels::l5 => "5",
            PartialDepthStreamLevels::l10 => "10",
            PartialDepthStreamLevels::l20 => "20",
        };
        let update_speed_str = match self.update_speed {
            PartialDepthStreamUpdateSpeed::ms100 => "@100ms",
            PartialDepthStreamUpdateSpeed::ms250 => "",
            PartialDepthStreamUpdateSpeed::ms500 => "@500ms",
        };
        Url::from_str(&format!("{}/ws/{}@depth{}{}", ws_base_url, self.symbol.to_lowercase(), levels_str, update_speed_str))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiffDepthEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "T")]
    pub transaction_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: usize,
    #[serde(rename = "u")]
    pub final_update_id: usize,
    #[serde(rename = "pu")]
    pub last_event_final_update_id: usize,
    #[serde(rename = "b")]
    pub bid_updates: Vec<BookLevel>,
    #[serde(rename = "a")]
    pub ask_updates: Vec<BookLevel>,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum DiffDepthStreamUpdateSpeed {
    ms100,
    ms250,
    ms500,
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
            DiffDepthStreamUpdateSpeed::ms250 => "",
            DiffDepthStreamUpdateSpeed::ms500 => "@500ms",
        };
        Url::from_str(&format!("{}/ws/{}@depth{}", ws_base_url, self.symbol.to_lowercase(), update_speed_str))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompositeIndexEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "C")]
    pub c: String,
    #[serde(rename = "c", default = "Vec::new")]
    pub composition: Vec<Composite>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Composite {
    #[serde(rename = "b")]
    pub base_asset: String,
    #[serde(rename = "q")]
    pub quote_asset: String,
    #[serde(rename = "w")]
    pub qty_weight: String,
    #[serde(rename = "W")]
    pub percentage_weight: String,
    #[serde(rename = "i")]
    pub index_price: String,
}

#[derive(Debug, Clone, Copy)]
pub struct CompositeIndexStream<'a> {
    pub symbol: &'a str,
}

impl<'a> BinanceStream for CompositeIndexStream<'a> {
    type Event = CompositeIndexEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/{}@compositeIndex", ws_base_url, self.symbol.to_lowercase()))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractStatus {
    PendingTrading,
    Trading,
    PreDelivering,
    Delivering,
    Delivered,
    PreSettle,
    Settling,
    Close,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContractInfoEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "ps")]
    pub pair: String,
    #[serde(rename = "ct")]
    pub contract_type: ContractType,
    #[serde(rename = "dt")]
    pub delivery_time: usize,
    #[serde(rename = "ot")]
    pub onboard_time: usize,
    #[serde(rename = "cs")]
    pub contract_status: ContractStatus,
    #[serde(rename = "bks")]
    pub notional_brackets: Vec<NotionalBracket>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotionalBracket {
    #[serde(rename = "bs")]
    pub id: usize,
    #[serde(rename = "bnf")]
    pub floor: String,
    #[serde(rename = "bnc")]
    pub cap: String,
    #[serde(rename = "mmr")]
    pub maintenance_ratio: String,
    #[serde(rename = "cf")]
    pub auxiliary_number: String,
    #[serde(rename = "mi")]
    pub min_leverage: usize,
    #[serde(rename = "ma")]
    pub max_leverage: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct ContractInfoStream;

impl BinanceStream for ContractInfoStream {
    type Event = ContractInfoEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/!contractInfo", ws_base_url))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssetIndexUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub index_price: String,
    #[serde(rename = "b")]
    pub bid_buffer: String,
    #[serde(rename = "a")]
    pub ask_buffer: String,
    #[serde(rename = "B")]
    pub bid_rate: String,
    #[serde(rename = "A")]
    pub ask_rate: String,
    #[serde(rename = "q")]
    pub auto_exchange_bid_buffer: String,
    #[serde(rename = "g")]
    pub auto_exchange_ask_buffer: String,
    #[serde(rename = "Q")]
    pub auto_exchange_bid_rate: String,
    #[serde(rename = "G")]
    pub auto_exchange_ask_rate: String,
}

#[derive(Debug, Clone, Copy)]
pub struct AssetIndexUpdateStream<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct AllAssetIndexUpdatesStream;

impl<'a> BinanceStream for AssetIndexUpdateStream<'a> {
    type Event = AssetIndexUpdateEvent;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/{}@assetIndex", ws_base_url, self.symbol.to_lowercase()))
    }
}

impl BinanceStream for AllAssetIndexUpdatesStream {
    type Event = Vec<AssetIndexUpdateEvent>;

    fn build_url(&self, ws_base_url: &str) -> Result<Url, url::ParseError> {
        Url::from_str(&format!("{}/ws/!assetIndex@arr", ws_base_url))
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PingRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct PingResponse { }

impl Request for PingRequest {
    type Response = PingResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ping";
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
    const ENDPOINT: &'static str = "/fapi/v1/time";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ExchangeInfoRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoResponse {
    pub exchange_filters: Vec<ExchangeFilter>,
    pub rate_limits: Vec<RateLimit>,
    pub server_time: usize,
    #[serde(rename = "symbols")]
    pub markets: Vec<Market>,
}

impl Request for ExchangeInfoRequest {
    type Response = ExchangeInfoResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/exchangeInfo";
}

#[derive(Debug, Clone, Deserialize)]
pub enum ExchangeFilter {
    // No info about this on binance api docs
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
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Second,
    Minute,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub symbol: String,
    pub pair: String,
    #[serde(deserialize_with = "treat_error_as_none")]
    pub contract_type: Option<ContractType>,
    pub delivery_date: usize,
    pub onboard_date: usize,
    pub status: ContractStatus,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub margin_asset: String,
    pub price_precision: usize,
    #[serde(rename = "quantityPrecision")]
    pub qty_precision: usize,
    pub base_asset_precision: usize,
    #[serde(rename = "quotePrecision")]
    pub quote_asset_precision: usize,
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
    #[serde(rename = "PRICE_FILTER", rename_all = "camelCase")]
    Price {
        #[serde(rename = "minPrice")]
        min: String,
        #[serde(rename = "maxPrice")]
        max: String,
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
        #[serde(rename = "limit")]
        max: usize,
    },
    MaxNumAlgoOrders {
        #[serde(rename = "limit")]
        max: usize,
    },
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: String,
        multiplier_down: String,
        #[serde(rename = "multiplierDecimal")]
        multiplier_precision: String,
    },
    #[serde(rename_all = "camelCase")]
    MinNotional {
        #[serde(rename = "notional")]
        min: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct OrderBookRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<OrderBookRequestLevels>,
}

#[derive(Debug, Clone, Copy, Serialize_repr)]
#[repr(usize)]
#[allow(non_camel_case_types)]
pub enum OrderBookRequestLevels {
    l5 = 5,
    l10 = 10,
    l20 = 20,
    l50 = 50,
    l100 = 100,
    l500 = 500,
    l1000 = 1000,
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

impl<'a> Request for OrderBookRequest<'a> {
    type Response = OrderBookResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/depth";
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
    const ENDPOINT: &'static str = "/fapi/v1/trades";
}

impl<'a> ApiKeyHeaderRequest for OldTradesRequest<'a> {
    type Response = Vec<TradeResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/historicalTrades";
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
    const ENDPOINT: &'static str = "/fapi/v1/aggTrades";
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
    pub limit: Option<usize>, // <= 1500
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousContractKlinesRequest<'a> {
    pub pair: &'a str,
    pub contract_type: ContractType,
    pub interval: KlineInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1500
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
    const ENDPOINT: &'static str = "/fapi/v1/klines";
}

impl<'a> Request for ContinuousContractKlinesRequest<'a> {
    type Response = Vec<KlineResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/continuousKlines";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlinesRequest<'a> {
    pub pair: &'a str,
    pub interval: KlineInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1500
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKlinesRequest<'a> {
    pub symbol: &'a str,
    pub interval: KlineInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1500
}

#[derive(Debug, Clone, Deserialize)]
pub struct IndexKlineResponse {
    pub open_time: usize,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub close_time: usize,
}

impl<'a> Request for IndexPriceKlinesRequest<'a> {
    type Response = Vec<IndexKlineResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/indexPriceKlines";
}

impl<'a> Request for MarkPriceKlinesRequest<'a> {
    type Response = Vec<IndexKlineResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/markPriceKlines";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct MarkPriceRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AllMarkPricesRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceResponse {
    pub symbol: String,
    pub mark_price: String,
    pub index_price: String,
    pub estimated_settle_price: String,
    pub last_funding_rate: String,
    pub next_funding_time: usize,
    pub interest_rate: String,
    pub time: usize,
}

impl<'a> Request for MarkPriceRequest<'a> {
    type Response = MarkPriceResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/premiumIndex";
}

impl Request for AllMarkPricesRequest {
    type Response = Vec<MarkPriceResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/premiumIndex";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistoryRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistoryResponse {
    pub symbol: String,
    pub funding_rate: String,
    pub funding_time: usize,
}

impl<'a> Request for FundingRateHistoryRequest<'a> {
    type Response = Vec<FundingRateHistoryResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/fundingRate";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TickerRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AllTickersRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerResponse {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    #[serde(rename = "weightedAvgPrice")]
    pub volume_weighted_avg_price: String,
    pub last_price: String,
    pub last_qty: String,
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

impl<'a> Request for TickerRequest<'a> {
    type Response = TickerResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ticker/24hr";
}

impl Request for AllTickersRequest {
    type Response = Vec<TickerResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ticker/24hr";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PriceTickerRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AllPriceTickersRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct PriceTickerResponse {
    pub symbol: String,
    pub price: String,
    pub time: usize,
}

impl<'a> Request for PriceTickerRequest<'a> {
    type Response = PriceTickerResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ticker/price";
}

impl Request for AllPriceTickersRequest {
    type Response = Vec<PriceTickerResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ticker/price";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct BookTickerRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AllBookTickersRequest;

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
    pub time: usize,
}

impl<'a> Request for BookTickerRequest<'a> {
    type Response = BookTickerResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ticker/bookTicker";
}

impl Request for AllBookTickersRequest {
    type Response = Vec<BookTickerResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/ticker/bookTicker";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct OpenInterestRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestResponse {
    pub open_interest: String,
    pub symbol: String,
    pub time: usize,
}

impl<'a> Request for OpenInterestRequest<'a> {
    type Response = OpenInterestResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/openInterest";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct OpenInterestStatisticsRequest<'a> {
    pub symbol: &'a str,
    pub period: FuturesStatisticsPeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[allow(non_camel_case_types)]
pub enum FuturesStatisticsPeriod {
    #[serde(rename = "5m")]
    m5,
    #[serde(rename = "15m")]
    m15,
    #[serde(rename = "30m")]
    m30,
    #[serde(rename = "1h")]
    h1,
    #[serde(rename = "2h")]
    h2,
    #[serde(rename = "4h")]
    h4,
    #[serde(rename = "6h")]
    h6,
    #[serde(rename = "12h")]
    h12,
    #[serde(rename = "1d")]
    d1,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestStatisticsResponse {
    pub symbol: String,
    pub sum_open_interest: String,
    pub sum_open_interest_value: String,
    pub timestamp: usize,
}

impl<'a> Request for OpenInterestStatisticsRequest<'a> {
    type Response = Vec<OpenInterestStatisticsResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/futures/data/openInterestHist";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TopLongShortAccountRatioRequest<'a> {
    pub symbol: &'a str,
    pub period: FuturesStatisticsPeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortAccountRatioResponse {
    pub symbol: String,
    pub long_short_ratio: String,
    pub long_account: String,
    pub short_account: String,
    pub timestamp: usize,
}

impl<'a> Request for TopLongShortAccountRatioRequest<'a> {
    type Response = Vec<TopLongShortAccountRatioResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/futures/data/topLongShortAccountRatio";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TopLongShortPositionRatioRequest<'a> {
    pub symbol: &'a str,
    pub period: FuturesStatisticsPeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLongShortPositionRatioResponse {
    pub symbol: String,
    pub long_short_ratio: String,
    pub long_account: String,
    pub short_account: String,
    pub timestamp: usize,
}

impl<'a> Request for TopLongShortPositionRatioRequest<'a> {
    type Response = Vec<TopLongShortPositionRatioResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/futures/data/topLongShortPositionRatio";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct GlobalLongShortAccountRatioRequest<'a> {
    pub symbol: &'a str,
    pub period: FuturesStatisticsPeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLongShortAccountRatioResponse {
    pub symbol: String,
    pub long_short_ratio: String,
    pub long_account: String,
    pub short_account: String,
    pub timestamp: usize,
}

impl<'a> Request for GlobalLongShortAccountRatioRequest<'a> {
    type Response = Vec<GlobalLongShortAccountRatioResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/futures/data/globalLongShortAccountRatio";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TakerBuySellVolumeRequest<'a> {
    pub symbol: &'a str,
    pub period: FuturesStatisticsPeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TakerBuySellVolumeResponse {
    pub buy_sell_ratio: String,
    pub buy_vol: String,
    pub sell_vol: String,
    pub timestamp: usize,
}

impl<'a> Request for TakerBuySellVolumeRequest<'a> {
    type Response = Vec<TakerBuySellVolumeResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/futures/data/globalLongShortAccountRatio";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalBlvtKlinesRequest<'a> {
    pub symbol: &'a str,
    pub interval: KlineInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 1500
}

#[derive(Debug, Clone, Deserialize)]
pub struct HistoricalBlvtKlineResponse {
    pub open_time: usize,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub real_leverage: String,
    pub close_time: usize,
    pub nav_update_count: usize,
}

impl<'a> Request for HistoricalBlvtKlinesRequest<'a> {
    type Response = Vec<HistoricalBlvtKlineResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/lvtKlines";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexRequest<'a> {
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllAssetIndexesRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndexResponse {
    pub symbol: String,
    pub time: usize,
    pub index: String,
    pub bid_buffer: String,
    pub ask_buffer: String,
    pub bid_rate: String,
    pub ask_rate: String,
    pub auto_exchange_bid_buffer: String,
    pub auto_exchange_ask_buffer: String,
    pub auto_exchange_bid_rate: String,
    pub auto_exchange_ask_rate: String,
}

impl<'a> Request for AssetIndexRequest<'a> {
    type Response = AssetIndexResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/assetIndex";
}

impl Request for AllAssetIndexesRequest {
    type Response = Vec<AssetIndexResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/assetIndex";
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CreateListenKeyRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateListenKeyResponse {
    pub listen_key: String,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct KeepAliveListenKeyRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct KeepAliveListenKeyResponse { }

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CloseListenKeyRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct CloseListenKeyResponse { }

impl ApiKeyHeaderRequest for CreateListenKeyRequest {
    type Response = CreateListenKeyResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/listenKey";
}

impl ApiKeyHeaderRequest for KeepAliveListenKeyRequest {
    type Response = KeepAliveListenKeyResponse;

    const METHOD: Method = Method::PUT;
    const ENDPOINT: &'static str = "/fapi/v1/listenKey";
}

impl ApiKeyHeaderRequest for CloseListenKeyRequest {
    type Response = CloseListenKeyResponse;

    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/fapi/v1/listenKey";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePositionModeRequest {
    #[serde(serialize_with = "serialize_bool_as_str")]
    pub dual_side_position: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePositionModeResponse { }

impl SignedRequest for ChangePositionModeRequest {
    type Response = ChangePositionModeResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/positionSide/dual";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionModeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionModeResponse {
    pub dual_side_position: bool,
}

impl SignedRequest for GetPositionModeRequest {
    type Response = GetPositionModeResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/positionSide/dual";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMultiAssetModeRequest {
    #[serde(serialize_with = "serialize_bool_as_str")]
    pub multi_asset_margin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeMultiAssetModeResponse { }

impl SignedRequest for ChangeMultiAssetModeRequest {
    type Response = ChangeMultiAssetModeResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/multiAssetsMargin";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMultiAssetModeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMultiAssetModeResponse {
    pub multi_asset_margin: bool,
}

impl SignedRequest for GetMultiAssetModeRequest {
    type Response = GetMultiAssetModeResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/multiAssetsMargin";
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestNewOrderRequest<'a> {
    pub symbol: &'a str,
    pub side: Side,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "quantity")]
    pub qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_bool_as_str")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "newClientOrderId")]
    pub client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_bool_as_str")]
    pub close_position: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_bool_as_str")]
    pub price_protect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_till_date: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestNewOrderResponse { }

impl<'a> SignedRequest for TestNewOrderRequest<'a> {
    type Response = TestNewOrderResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/order/test";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest<'a> {
    pub symbol: &'a str,
    pub side: Side,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "quantity")]
    pub qty: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_bool_as_str")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "newClientOrderId")]
    pub client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_bool_as_str")]
    pub close_position: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_opt_bool_as_str")]
    pub price_protect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_till_date: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResponse {
    pub client_order_id: String,
    #[serde(rename = "cumQty")]
    pub cummulative_qty: String,
    #[serde(rename = "cumQuote")]
    pub cummulative_quote_qty: String,
    pub executed_qty: String,
    pub order_id: usize,
    #[serde(rename = "avgPrice")]
    pub average_price: String,
    pub orig_qty: String,
    pub price: String,
    pub reduce_only: bool,
    pub side: Side,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    pub stop_price: String,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(rename = "origType")]
    pub orig_order_type: OrderType,
    pub activate_price: Option<String>,
    pub price_rate: Option<String>,
    pub update_time: usize,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub good_till_date: usize,
}

impl SignedRequest for NewOrderRequest<'_> {
    type Response = NewOrderResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/order";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<usize>,
    pub symbol: &'a str,
    pub side: Side,
    #[serde(rename = "quantity")]
    pub qty: &'a str,
    pub price: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderResponse {
    pub order_id: usize,
    pub symbol: String,
    pub pair: String,
    pub status: OrderStatus,
    pub client_order_id: String,
    pub price: String,
    #[serde(rename = "avgPrice")]
    pub average_price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    #[serde(rename = "cumQty")]
    pub cummulative_qty: String,
    #[serde(rename = "cumBase")]
    pub cummulative_base_qty: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: Side,
    pub position_side: PositionSide,
    pub stop_price: String,
    pub working_type: WorkingType,
    pub price_protect: bool,
    #[serde(rename = "origType")]
    pub orig_order_type: OrderType,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub good_till_date: usize,
    pub update_time: usize,
}

impl SignedRequest for ModifyOrderRequest<'_> {
    type Response = ModifyOrderResponse;

    const METHOD: Method = Method::PUT;
    const ENDPOINT: &'static str = "/fapi/v1/order";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrdersRequest<'a> {
    pub batch_orders: &'a [NewOrderRequest<'a>],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

impl SignedRequest for NewOrdersRequest<'_> {
    type Response = Vec<Result<NewOrderResponse, BinanceErrorMsg>>;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/batchOrders";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrdersRequest<'a> {
    pub batch_orders: &'a [ModifyOrderRequest<'a>],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

impl SignedRequest for ModifyOrdersRequest<'_> {
    type Response = Vec<Result<ModifyOrderResponse, BinanceErrorMsg>>;

    const METHOD: Method = Method::PUT;
    const ENDPOINT: &'static str = "/fapi/v1/batchOrders";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderModifyHistoryRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modification {
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmendment {
    pub price: Modification,
    pub orig_qty: Modification,
    pub count: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderModifyHistoryResponse {
    pub amendment_id: usize,
    pub symbol: String,
    pub pair: String,
    pub order_id: usize,
    pub client_order_id: String,
    pub time: usize,
    pub amendment: OrderAmendment,
}

impl SignedRequest for GetOrderModifyHistoryRequest<'_> {
    type Response = GetOrderModifyHistoryResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/orderAmendment";
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
    #[serde(rename = "avgPrice")]
    pub average_price: String,
    pub client_order_id: String,
    #[serde(rename = "cumQuote")]
    pub cummulative_quote_qty: String,
    pub executed_qty: String,
    pub order_id: usize,
    pub orig_qty: String,
    #[serde(rename = "origType")]
    pub orig_order_type: OrderType,
    pub price: String,
    pub reduce_only: bool,
    pub side: Side,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    pub stop_price: String,
    pub close_position: bool,
    pub symbol: String,
    pub time: usize,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub activate_price: Option<String>,
    pub price_rate: Option<String>,
    pub update_time: usize,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub good_till_date: usize,
}

impl SignedRequest for QueryOrderRequest<'_> {
    type Response = QueryOrderResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/order";
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
    #[serde(rename = "cumQty")]
    pub cummulative_qty: String,
    #[serde(rename = "cumQuote")]
    pub cummulative_quote_qty: String,
    pub executed_qty: String,
    pub order_id: usize,
    pub orig_qty: String,
    #[serde(rename = "origType")]
    pub orig_order_type: OrderType,
    pub price: String,
    pub reduce_only: bool,
    pub side: Side,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    pub stop_price: String,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub activate_price: Option<String>,
    pub price_rate: Option<String>,
    pub update_time: usize,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub good_till_date: usize,
}

impl SignedRequest for CancelOrderRequest<'_> {
    type Response = CancelOrderResponse;

    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/fapi/v1/order";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersResponse { }

impl SignedRequest for CancelAllOrdersRequest<'_> {
    type Response = CancelAllOrdersResponse;

    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/fapi/v1/allOpenOrders";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrdersRequest<'a> {
    pub symbol: &'a str,
    pub order_id_list: Option<&'a [usize]>, // len <= 10
    pub orig_client_order_id_list: Option<&'a [String]>, // len <= 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

impl SignedRequest for CancelOrdersRequest<'_> {
    type Response = Vec<Result<CancelOrderResponse, BinanceErrorMsg>>;

    const METHOD: Method = Method::DELETE;
    const ENDPOINT: &'static str = "/fapi/v1/batchOrders";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersWithCountdownRequest<'a> {
    pub symbol: &'a str,
    pub countdown_time: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersWithCountdownResponse {
    pub symbol: String,
    pub countdown_time: String,
}

impl SignedRequest for CancelAllOrdersWithCountdownRequest<'_> {
    type Response = CancelAllOrdersWithCountdownResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/countdownCancelAll";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOpenOrderRequest<'a> {
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
pub struct QueryOpenOrderResponse {
    #[serde(rename = "avgPrice")]
    pub average_price: String,
    pub client_order_id: String,
    #[serde(rename = "cumQuote")]
    pub cummulative_quote_qty: String,
    pub executed_qty: String,
    pub order_id: usize,
    pub orig_qty: String,
    #[serde(rename = "origType")]
    pub orig_order_type: OrderType,
    pub price: String,
    pub reduce_only: bool,
    pub side: Side,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    pub stop_price: String,
    pub close_position: bool,
    pub symbol: String,
    pub time: usize,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub activate_price: Option<String>,
    pub price_rate: Option<String>,
    pub update_time: usize,
    pub working_type: WorkingType,
    pub price_protect: bool,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    pub good_till_date: usize,
}

impl SignedRequest for QueryOpenOrderRequest<'_> {
    type Response = QueryOpenOrderResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/openOrder";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryAllOpenOrdersRequest<'a> {
    pub symbol: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

impl SignedRequest for QueryAllOpenOrdersRequest<'_> {
    type Response = Vec<QueryOpenOrderResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/openOrders";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryAllOrdersRequest<'a> {
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

impl SignedRequest for QueryAllOrdersRequest<'_> {
    type Response = Vec<QueryOrderResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/allOrders";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BalancesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    pub account_alias: String,
    pub asset: String,
    pub balance: String,
    pub cross_wallet_balance: String,
    #[serde(rename = "crossUnPnl")]
    pub cross_unrealized_pnl: String,
    pub available_balance: String,
    pub max_withdraw_amount: String,
    pub margin_availble: bool,
    pub update_time: usize,
}

impl SignedRequest for BalancesRequest {
    type Response = BalanceResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v2/balance";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetInformation {
    pub asset: String,
    pub wallet_balance: String,
    pub unrealized_profit: String,
    pub margin_balance: String,
    #[serde(rename = "maintMargin")]
    pub maintenance_margin: String,
    pub initial_margin: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub cross_wallet_balance: String,
    #[serde(rename = "crossUnPnl")]
    pub cross_unrealized_pnl: String,
    pub available_balance: String,
    pub max_withdraw_amount: String,
    pub margin_available: bool,
    pub update_time: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInformation {
    pub symbol: String,
    pub initial_margin: String,
    #[serde(rename = "maintMargin")]
    pub maintenance_margin: String,
    pub unrealized_profit: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub leverage: String,
    pub isolated: bool,
    pub entry_price: String,
    pub max_notional: String,
    pub bid_notional: String,
    pub ask_notional: String,
    pub position_side: PositionSide,
    #[serde(rename = "positionAmt")]
    pub position_amount: String,
    pub update_time: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformationResponse {
    pub fee_tier: usize,
    pub can_trade: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub update_time: usize,
    pub multi_asset_margin: bool,
    #[serde(deserialize_with = "treat_error_as_none")]
    pub trade_group_id: Option<usize>,
    pub total_initial_margin: String,
    #[serde(rename = "totalMaintMargin")]
    pub total_maintenance_margin: String,
    pub total_wallet_balance: String,
    pub total_unrealized_profit: String,
    pub total_margin_balance: String,
    pub total_position_initial_margin: String,
    pub total_open_order_initial_margin: String,
    pub total_cross_wallet_balance: String,
    #[serde(rename = "totalCrossUnPnl")]
    pub total_cross_unrealized_pnl: String,
    pub available_balance: String,
    pub max_withdraw_amount: String,
    pub assets: Vec<AssetInformation>,
    pub positions: Vec<PositionInformation>,
}

impl SignedRequest for AccountInformationRequest {
    type Response = AccountInformationResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v2/account";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageRequest<'a> {
    pub symbol: &'a str,
    pub leverage: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageResponse {
    pub leverage: usize,
    pub max_notional_value: String,
    pub symbol: String,
}

impl SignedRequest for ChangeLeverageRequest<'_> {
    type Response = ChangeLeverageResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/leverage";
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginType {
    #[serde(alias = "isolated")]
    Isolated,
    #[serde(alias = "crossed")]
    Crossed,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginTypeRequest<'a> {
    pub symbol: &'a str,
    pub margin_type: MarginType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeMarginTypeResponse { }

impl SignedRequest for ChangeMarginTypeRequest<'_> {
    type Response = ChangeMarginTypeResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/marginType";
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(usize)]
pub enum ModifiyPositionMarginType {
    Add = 1,
    Reduce = 2,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyIsolatedPositionMarginRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,
    pub amount: &'a str,
    #[serde(rename = "type")]
    pub modifiy_position_margin_type: ModifiyPositionMarginType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyIsolatedPositionMarginResponse { }

impl SignedRequest for ModifyIsolatedPositionMarginRequest<'_> {
    type Response = ModifyIsolatedPositionMarginResponse;

    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/fapi/v1/positionMargin";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionMarginChangeHistoryRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub modifiy_position_margin_type: Option<ModifiyPositionMarginType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionMarginChange {
    pub symbol: String,
    #[serde(rename = "type")]
    pub modifiy_position_margin_type: ModifiyPositionMarginType,
    pub amount: String,
    pub asset: String,
    pub time: usize,
    pub position_side: PositionSide,
}

impl SignedRequest for PositionMarginChangeHistoryRequest<'_> {
    type Response = Vec<PositionMarginChange>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/positionMargin/history";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionResponse {
    pub entry_price: String,
    pub break_even_price: String,
    pub margin_type: MarginType,
    pub is_auto_add_margin: String,
    pub isolated_margin: String,
    pub leverage: String,
    pub liquidation_price: String,
    pub mark_price: String,
    pub max_notional_value: String,
    #[serde(rename = "positionAmt")]
    pub position_amount: String,
    pub notional: String,
    pub isolated_wallet: String,
    pub symbol: String,
    #[serde(rename = "unRealizedProfit")]
    pub unrealized_profit: String,
    pub position_side: PositionSide,
    pub update_time: usize,
}

impl SignedRequest for PositionRequest<'_> {
    type Response = Vec<PositionResponse>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v2/positionRisk";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradesRequest<'a> {
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
pub struct AccountTrade {
    pub buyer: bool,
    pub commission: String,
    pub commission_asset: String,
    pub id: usize,
    pub maker: bool,
    pub order_id: usize,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub realized_pnl: String,
    pub side: Side,
    pub position_side: PositionSide,
    pub symbol: String,
    pub time: usize,
}

impl SignedRequest for AccountTradesRequest<'_> {
    type Response = Vec<AccountTrade>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/userTrades";
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IncomeType {
    Transfer,
    WelcomeBonus,
    RealizedPNL,
    FundingFee,
    Commission,
    InsuranceClear,
    ReferralKickback,
    CommissionRebate,
    ApiRebate,
    ContestReward,
    CrossCollateralTransfer,
    OptionsPremiumFee,
    OptionsSettleProfit,
    InternalTransfer,
    AutoExchange,
    DeliveredSettlement,
    CoinSwapDeposit,
    CoinSwapWithdraw,
    PositionLimitIncreaseFee,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomeHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_type: Option<IncomeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Income {
    pub symbol: String,
    pub income_type: IncomeType,
    pub income: String,
    pub asset: String,
    pub time: usize,
    #[serde(rename = "tranId")]
    pub traansaction_id: String,
    pub trade_id: String,
}

impl SignedRequest for IncomeHistoryRequest<'_> {
    type Response = Vec<Income>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/income";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracketsRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
    pub bracket: usize,
    pub initial_leverage: usize,
    pub notional_cap: usize,
    pub notional_floor: usize,
    #[serde(rename = "maintMarginRatio")]
    pub maintenance_margin_ration: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolLeverageBrackets {
    pub symbol: String,
    pub brackets: Vec<LeverageBracket>,
}

impl SignedRequest for LeverageBracketsRequest<'_> {
    type Response = Vec<SymbolLeverageBrackets>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/leverageBracket";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AutoCloseType {
    Liquidation,
    Adl,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserForceOrdersRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_close_type: Option<AutoCloseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>, // <= 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserForceOrder {
    pub order_id: usize,
    pub symbol: String,
    pub status: OrderStatus,
    pub client_order_id: String,
    pub price: String,
    #[serde(rename = "avgPrice")]
    pub average_price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    #[serde(rename = "cumQuote")]
    pub cummulative_quote_qty: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: Side,
    pub position_side: PositionSide,
    pub stop_price: String,
    pub working_type: WorkingType,
    #[serde(rename = "origType")]
    pub orig_order_type: OrderType,
    pub time: usize,
    pub update_time: usize,
}

impl SignedRequest for UserForceOrdersRequest<'_> {
    type Response = Vec<UserForceOrder>;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/forceOrders";
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCommissionRateRequest<'a> {
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<usize>, // <= 60_000
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCommissionRateResponse {
    pub symbol: String,
    pub maker_commission_rate: String,
    pub taker_commission_rate: String,
}

impl SignedRequest for UserCommissionRateRequest<'_> {
    type Response = UserCommissionRateResponse;

    const METHOD: Method = Method::GET;
    const ENDPOINT: &'static str = "/fapi/v1/commissionRate";
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionMarginCall {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
    #[serde(rename = "pa")]
    pub position_amount: String,
    #[serde(rename = "mt")]
    pub margin_type: MarginType,
    #[serde(rename = "iw")]
    pub isolated_wallet: Option<String>,
    #[serde(rename = "mp")]
    pub mark_price: String,
    #[serde(rename = "up")]
    pub unrealized_pnl: String,
    #[serde(rename = "mm")]
    pub required_maintenance_margin: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarginCallEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "cw")]
    pub cross_wallet_balance: Option<String>,
    #[serde(rename = "p")]
    pub positions: Vec<PositionMarginCall>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BalancePositionEventReason {
    Deposit,
    Withdraw,
    Order,
    FundingFee,
    WithdrawReject,
    Adjustment,
    InsuranceClear,
    AdminDeposit,
    AdminWithdraw,
    MarginTransfer,
    MarginTypeChange,
    AssetTransfer,
    OptionsPremiumFee,
    OptionsSettleProfit,
    AutoExchange,
    CoinSwapDeposit,
    CoinSwapWithdraw,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalancePositionUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "T")]
    pub transaction_time: usize,
    #[serde(rename = "a")]
    pub balance_position_update: BalancePositionUpdate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalancePositionUpdate {
    #[serde(rename = "m")]
    pub reason: BalancePositionEventReason,
    #[serde(rename = "B")]
    pub balance_updates: Vec<BalanceUpdate>,
    #[serde(rename = "P")]
    pub position_updates: Vec<PositionUpdate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceUpdate {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb")]
    pub wallet_balance: String,
    #[serde(rename = "cw")]
    pub cross_wallet_balance: String,
    #[serde(rename = "bc")]
    pub balance_change: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionUpdate {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa")]
    pub position_amount: String,
    #[serde(rename = "ep")]
    pub entry_price: String,
    #[serde(rename = "bep")]
    pub breakeven_price: String,
    #[serde(rename = "cr")]
    pub realized_pnl: String,
    #[serde(rename = "up")]
    pub unrealized_pnl: String,
    #[serde(rename = "mt")]
    pub margin_type: MarginType,
    #[serde(rename = "iw")]
    pub isolated_wallet: Option<String>,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderExecutionType {
    New,
    Canceled,
    Calculated, // Liquidation Execution
    Expired,
    Trade,
    Amendment,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "T")]
    pub transaction_time: usize,
    #[serde(rename = "o")]
    pub order_update: OrderUpdate,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderUpdate {
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
    pub orig_qty: String,
    #[serde(rename = "p")]
    pub orig_price: String,
    #[serde(rename = "ap")]
    pub average_price: String,
    #[serde(rename = "sp")]
    pub stop_price: String,
    #[serde(rename = "x")]
    pub current_order_execution_type: OrderExecutionType,
    #[serde(rename = "X")]
    pub current_order_status: OrderStatus,
    #[serde(rename = "i")]
    pub order_id: usize,
    #[serde(rename = "l")]
    pub last_filled_qty: String,
    #[serde(rename = "z")]
    pub cummulative_filled_qty: String,
    #[serde(rename = "L")]
    pub last_fill_price: String,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "n")]
    pub commission_amount: Option<String>,
    #[serde(rename = "T")]
    pub order_trade_time: usize,
    #[serde(rename = "t")]
    pub order_trade_id: usize,
    #[serde(rename = "b")]
    pub bid_notional: String,
    #[serde(rename = "a")]
    pub ask_notional: String,
    #[serde(rename = "m")]
    pub is_trade_maker: bool,
    #[serde(rename = "R")]
    pub is_reduce_only: bool,
    #[serde(rename = "wt")]
    pub stop_price_working_type: WorkingType,
    #[serde(rename = "ot")]
    pub orig_order_type: OrderType,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
    #[serde(rename = "cp")]
    pub close_position: bool,
    #[serde(rename = "AP")]
    pub activation_price: Option<String>,
    #[serde(rename = "cr")]
    pub callback_rate: Option<String>,
    #[serde(rename = "pP")]
    pub price_protection: bool,
    #[serde(rename = "rp")]
    pub trade_realized_profit: String,
    #[serde(rename = "V")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    #[serde(rename = "gtd")]
    pub good_till_date: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountConfigurationUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "T")]
    pub transaction_time: usize,
    #[serde(rename = "ac")]
    pub leverage: Option<LeverageUpdate>,
    #[serde(rename = "ai")]
    pub multi_asset_mode: Option<MultiAssetModeUpdate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LeverageUpdate {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l")]
    pub leverage: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MultiAssetModeUpdate {
    #[serde(rename = "j")]
    pub multi_asset_mode: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConditionalOrderTriggerRejectEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "T")]
    pub transaction_time: usize,
    #[serde(rename = "or")]
    pub conditional_order_trigger_reject: ConditionalOrderTriggerReject,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConditionalOrderTriggerReject {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub order_id: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListenKeyExpiredEvent {
    #[serde(rename = "E")]
    pub event_time: usize,
    #[serde(rename = "listenKey")]
    pub listen_key: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "e")]
pub enum UserStreamEvent {
    #[serde(rename = "MARGIN_CALL")]
    MarginCall(MarginCallEvent),
    #[serde(rename = "ACCOUNT_UPDATE")]
    BalancePositionUpdate(BalancePositionUpdateEvent),
    #[serde(rename = "ORDER_TRADE_UPDATE")]
    OrderUpdate(OrderUpdateEvent),
    #[serde(rename = "ACCOUNT_CONFIG_UPDATE")]
    AccountConfigurationUpdate(AccountConfigurationUpdateEvent),
    #[serde(rename = "CONDITIONAL_ORDER_TRIGGER_REJECT")]
    ConditionalOrderTriggerReject(ConditionalOrderTriggerRejectEvent),
    #[serde(rename = "listenKeyExpired")]
    ListenKeyExpired(ListenKeyExpiredEvent),
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
