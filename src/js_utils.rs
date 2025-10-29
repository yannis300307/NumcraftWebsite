use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/assets/libs/utils.js")]
extern "C" {
    pub fn is_usb_supported() -> bool;
    pub fn detect();
}