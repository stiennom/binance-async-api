use serde::Deserialize;

pub trait BinanceError {
    fn is_server_error(&self) -> bool;
}

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceResponseError {
    pub code: i64,
    pub msg: String,
}
