use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = r"/src/bridge/js/db.js")]
extern "C" {
    pub async fn get_data_all() -> JsValue;
}
