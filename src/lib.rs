use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Statsig, js_name = StatsigClient)]
    pub type StatsigClient;

    #[wasm_bindgen(constructor, js_namespace = Statsig)]
    pub fn _new(api_key: &str, user: &JsValue) -> StatsigClient;

    #[wasm_bindgen(method, js_namespace = Statsig, js_name = initializeAsync)]
    pub async fn initialize(this: &StatsigClient);

    #[wasm_bindgen(method, js_namespace = Statsig, js_name = initializeSync)]
    pub fn initialize_sync(this: &StatsigClient);

    #[wasm_bindgen(js_namespace = Statsig, js_name = runStatsigSessionReplay)]
    pub fn run_statsig_session_replay(client: &StatsigClient);

    #[wasm_bindgen(js_namespace = Statsig, js_name = runStatsigAutoCapture)]
    pub fn run_statsig_auto_capture(client: &StatsigClient);
}

#[derive(Serialize)]
pub struct StatsigUser {
    #[serde(rename = "userID")]
    pub user_id: String,
}

impl StatsigClient {
    pub fn new(api_key: &str, user: StatsigUser) -> Result<StatsigClient, String> {
        let js_user = serde_wasm_bindgen::to_value(&user)
            .map_err(|e| format!("Failed to serialize user: {}", e))?;

        Ok(StatsigClient::_new(api_key, &js_user))
    }
}
