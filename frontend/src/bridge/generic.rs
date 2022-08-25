use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = r"/src/bridge/js/generic.js")]
extern "C" {
    pub fn log(msg: &str);
}
