#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub rest_api_endpoint: &'static str,
    pub ws_endpoint: &'static str,

    pub usdm_futures_rest_api_endpoint: &'static str,
    pub usdm_futures_ws_endpoint: &'static str,

    pub coinm_futures_rest_api_endpoint: &'static str,
    pub coinm_futures_ws_endpoint: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rest_api_endpoint: "https://api.binance.com",
            ws_endpoint: "wss://stream.binance.com:9443",

            usdm_futures_rest_api_endpoint: "https://fapi.binance.com",
            usdm_futures_ws_endpoint: "wss://fstream.binance.com",

            coinm_futures_rest_api_endpoint: "https://dapi.binance.com",
            coinm_futures_ws_endpoint: "wss://dstream.binance.com",
        }
    }
}

impl Config {
    pub fn testnet() -> Self {
        Self {
            rest_api_endpoint: "https://testnet.binance.vision",
            ws_endpoint: "wss://testnet.binance.vision",

            usdm_futures_rest_api_endpoint: "https://testnet.binancefuture.com",
            usdm_futures_ws_endpoint: "wss://stream.binancefuture.com",

            coinm_futures_rest_api_endpoint: "https://testnet.binancefuture.com",
            coinm_futures_ws_endpoint: "wss://dstream.binancefuture.com",
        }
    }
}
