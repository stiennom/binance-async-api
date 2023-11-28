#[derive(Debug, Clone)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,

    pub usdm_futures_rest_api_endpoint: String,
    pub usdm_futures_ws_endpoint: String,

    pub coinm_futures_rest_api_endpoint: String,
    pub coinm_futures_ws_endpoint: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rest_api_endpoint: "https://api.binance.com".to_owned(),
            ws_endpoint: "wss://stream.binance.com:9443".to_owned(),

            usdm_futures_rest_api_endpoint: "https://fapi.binance.com".to_owned(),
            usdm_futures_ws_endpoint: "wss://fstream.binance.com".to_owned(),

            coinm_futures_rest_api_endpoint: "https://dapi.binance.com".to_owned(),
            coinm_futures_ws_endpoint: "wss://dstream.binance.com".to_owned(),
        }
    }
}

impl Config {
    pub fn testnet() -> Self {
        Self {
            rest_api_endpoint: "https://testnet.binance.vision".to_owned(),
            ws_endpoint: "wss://testnet.binance.vision".to_owned(),

            usdm_futures_rest_api_endpoint: "https://testnet.binancefuture.com".to_owned(),
            usdm_futures_ws_endpoint: "https://testnet.binancefuture.com".to_owned(),

            coinm_futures_rest_api_endpoint: "https://testnet.binancefuture.com".to_owned(),
            coinm_futures_ws_endpoint: "wss://dstream.binancefuture.com".to_owned(),
        }
    }
}
