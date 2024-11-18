use super::StreamTopic;
use crate::client::Product;
use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
pub struct AggTradeStream<'a> {
    pub symbol: &'a str,
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

impl StreamTopic for AggTradeStream<'_> {
    const PRODUCT: Product = Product::UsdMFutures;
    fn endpoint(&self) -> String {
        format!("/ws/{}@aggTrade", self.symbol.to_lowercase())
    }
    type Event = AggTradeEvent;
}

#[derive(Debug, Clone, Copy)]
pub struct BookTickerStream<'a> {
    pub symbol: &'a str,
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

impl StreamTopic for BookTickerStream<'_> {
    const PRODUCT: Product = Product::UsdMFutures;
    fn endpoint(&self) -> String {
        format!("/ws/{}@bookTicker", self.symbol.to_lowercase())
    }
    type Event = BookTickerEvent;
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
    pub bid_updates: Vec<BookLevelUpdate>,
    #[serde(rename = "a")]
    pub ask_updates: Vec<BookLevelUpdate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookLevelUpdate {
    pub price: String,
    pub qty: String,
}

#[derive(Debug, Clone, Copy)]
pub struct DiffDepthStream<'a> {
    pub symbol: &'a str,
}

impl StreamTopic for DiffDepthStream<'_> {
    const PRODUCT: Product = Product::UsdMFutures;
    fn endpoint(&self) -> String {
        format!("/ws/{}@depth@100ms", self.symbol.to_lowercase())
    }
    type Event = DiffDepthEvent;
}

#[derive(Debug, Clone, Copy)]
pub struct UserStream<'a> {
    pub listen_key: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionMarginCall {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "ps")]
    pub position_side: String,
    #[serde(rename = "pa")]
    pub position_amount: String,
    #[serde(rename = "mt")]
    pub margin_type: String,
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
    pub reason: String,
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
    pub margin_type: String,
    #[serde(rename = "iw")]
    pub isolated_wallet: Option<String>,
    #[serde(rename = "ps")]
    pub position_side: String,
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
    pub side: String,
    #[serde(rename = "o")]
    pub order_type: String,
    #[serde(rename = "f")]
    pub time_in_force: String,
    #[serde(rename = "q")]
    pub orig_qty: String,
    #[serde(rename = "p")]
    pub orig_price: String,
    #[serde(rename = "ap")]
    pub average_price: String,
    #[serde(rename = "sp")]
    pub stop_price: String,
    #[serde(rename = "x")]
    pub current_order_execution_type: String,
    #[serde(rename = "X")]
    pub current_order_status: String,
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
    pub stop_price_working_type: String,
    #[serde(rename = "ot")]
    pub orig_order_type: String,
    #[serde(rename = "ps")]
    pub position_side: String,
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
    pub self_trade_prevention_mode: String,
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
#[allow(clippy::large_enum_variant)]
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

impl StreamTopic for UserStream<'_> {
    const PRODUCT: Product = Product::UsdMFutures;
    fn endpoint(&self) -> String {
        format!("/ws/{}", self.listen_key)
    }
    type Event = UserStreamEvent;
}
