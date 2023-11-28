pub mod utils;
pub mod spot;
pub mod usd_m;

use serde::{
    Deserialize,
    Serialize,
};


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookLevel {
    pub price: String,
    pub qty: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    None,
    ExpireTaker,
    ExpireMaker,
    ExpireBoth,
}
