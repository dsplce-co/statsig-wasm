use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Statsig, js_name = StatsigClient)]
    pub type StatsigClient;

    #[wasm_bindgen(constructor, js_namespace = Statsig)]
    pub fn _new(api_key: &str, options: &JsValue) -> StatsigClient;

    #[wasm_bindgen(method, js_namespace = Statsig, js_name = initializeAsync)]
    pub async fn initialize_async(this: &StatsigClient);

    #[wasm_bindgen(method, js_namespace = Statsig, js_name = initializeSync)]
    pub fn initialize_sync(this: &StatsigClient);

    #[wasm_bindgen(js_namespace = Statsig, js_name = runStatsigAutoCapture)]
    pub fn run_statsig_auto_capture(client: &StatsigClient);
}

#[derive(Serialize)]
pub struct StatsigClientOptions {
    #[serde(rename = "userID")]
    pub user_id: String,
}

impl StatsigClient {
    pub fn new(api_key: &str, options: StatsigClientOptions) -> Result<StatsigClient, JsValue> {
        let js_options = serde_wasm_bindgen::to_value(&options)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize options: {}", e)))?;

        Ok(StatsigClient::_new(api_key, &js_options))
    }
}
