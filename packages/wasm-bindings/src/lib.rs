use wasm_bindgen::prelude::*;
use engine::Engine;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub struct WasmEngine {
    inner: Engine,
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEngine {
        WasmEngine { inner: Engine::new() }
    }

    pub fn tick(&mut self) {
        self.inner.tick();
    }

    #[wasm_bindgen(js_name = getState)]
    pub fn get_state(&self) -> JsValue {
        to_value(self.inner.state()).unwrap()
    }
}
