use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = r"/src/bridge/js/auth.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn sign_in(email: &str, password: &str) -> Result<(), JsValue>;

    pub async fn sign_out() -> JsValue;

    pub async fn is_logged_in() -> JsValue;
}
