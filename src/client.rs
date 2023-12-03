use crate::config::Config;
use reqwest::Client;

#[derive(Debug, Clone, Copy)]
pub enum Product {
    Spot,
    UsdMFutures,
    CoinMFutures,
}

#[derive(Clone, Default)]
pub struct BinanceClient {
    pub(crate) client: Client,
    pub(crate) config: Config,
}

impl BinanceClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }
}
