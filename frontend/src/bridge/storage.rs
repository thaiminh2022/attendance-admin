use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = r"/src/bridge/js/storage.js")]
extern "C" {
    pub fn clear_local_storage();

    #[wasm_bindgen(catch)]
    pub fn get_local_storage(key: &str) -> Result<String, JsValue>;

    pub fn set_local_storage(key: &str, json: String);
}
