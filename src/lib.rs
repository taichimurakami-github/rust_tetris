mod utils;

use wasm_bindgen::prelude::*;
// use wasm_bindgen::{Clamped, JsCast};
// use web_sys::ImageData;

mod models;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-tetris!");
}

#[wasm_bindgen]
pub fn alert_test() {
    alert("Oh you've got an alert!");
}

#[wasm_bindgen]
pub fn get_t_mino() {
    alert("test!");
}
