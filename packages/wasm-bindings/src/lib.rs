use engine::Engine;
use game::board::GameState;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmEngine {
    inner: Engine,
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEngine {
        WasmEngine {
            inner: Engine::new(),
        }
    }

    #[wasm_bindgen(js_name = getState)]
    pub fn get_state(&self) -> JsValue {
        to_value(self.inner.state()).unwrap()
    }
}

#[wasm_bindgen]
pub fn random_start(seed: u64) -> JsValue {
    to_value(&GameState::random_start(seed)).unwrap()
}
