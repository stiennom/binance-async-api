use reqwest::Client;

#[derive(Debug, Clone)]
pub struct Spot;
#[derive(Debug, Clone)]
pub struct Usdm;
#[derive(Debug, Clone)]
pub struct Coinm;

#[derive(Debug, Clone)]
pub struct ClientConfig<T> {
    pub rest_base_url: String,
    pub websocket_base_url: String,
    pub ws_api_base_url: String,
    _marker: std::marker::PhantomData<T>,
}

impl Default for ClientConfig<Spot> {
    fn default() -> Self {
        Self {
            rest_base_url: "https://api.binance.com".to_string(),
            websocket_base_url: "wss://stream.binance.com:9443".to_string(),
            ws_api_base_url: "wss://ws-api.binance.com:443/ws-api/v3".to_string(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl Default for ClientConfig<Usdm> {
    fn default() -> Self {
        Self {
            rest_base_url: "https://fapi.binance.com".to_string(),
            websocket_base_url: "wss://fstream.binance.com".to_string(),
            ws_api_base_url: "wss://ws-fapi.binance.com/ws-fapi/v1".to_string(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl Default for ClientConfig<Coinm> {
    fn default() -> Self {
        Self {
            rest_base_url: "https://dapi.binance.com".to_string(),
            websocket_base_url: "wss://dstream.binance.com".to_string(),
            ws_api_base_url: "".to_string(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> ClientConfig<T> {
    pub fn with_rest_base_url(mut self, rest_base_url: String) -> Self {
        self.rest_base_url = rest_base_url;
        self
    }
    pub fn with_websocket_base_url(mut self, websocket_base_url: String) -> Self {
        self.websocket_base_url = websocket_base_url;
        self
    }
    pub fn with_ws_api_base_url(mut self, ws_api_base_url: String) -> Self {
        self.ws_api_base_url = ws_api_base_url;
        self
    }
}

#[derive(Debug, Clone)]
pub struct BinanceClient<T> {
    pub(crate) client: Client,
    pub(crate) config: ClientConfig<T>,
}

impl Default for BinanceClient<Spot> {
    fn default() -> Self {
        Self {
            client: Client::default(),
            config: ClientConfig::default(),
        }
    }
}

impl Default for BinanceClient<Usdm> {
    fn default() -> Self {
        Self {
            client: Client::default(),
            config: ClientConfig::default(),
        }
    }
}

impl Default for BinanceClient<Coinm> {
    fn default() -> Self {
        Self {
            client: Client::default(),
            config: ClientConfig::default(),
        }
    }
}

impl BinanceClient<Spot> {
    pub fn spot() -> Self {
        Self::default()
    }
}

impl BinanceClient<Usdm> {
    pub fn usdm() -> Self {
        Self::default()
    }
}

impl BinanceClient<Coinm> {
    pub fn coinm() -> Self {
        Self::default()
    }
}

impl<T> BinanceClient<T> {
    pub fn with_config(mut self, config: ClientConfig<T>) -> Self {
        self.config = config;
        self
    }
}
