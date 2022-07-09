use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Coodinate {
    pub x: u8,
    pub y: u8,
}

pub enum MinoColors {
    Purple,
    Yellow,
    Lightblue,
    Red,
    Green,
    Orange,
    Blue,
}
