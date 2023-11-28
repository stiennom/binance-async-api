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
    pub(crate) key: Option<String>,
    pub(crate) secret: Option<String>,
    pub(crate) client: Client,
    pub(crate) config: Config,
}

impl BinanceClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_key(mut self, api_key: &str) -> Self {
        self.key = Some(api_key.to_owned());
        self
    }

    pub fn with_key_and_secret(mut self, api_key: &str, api_secret: &str) -> Self {
        self.key = Some(api_key.to_owned());
        self.secret = Some(api_secret.to_owned());
        self
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }
}
