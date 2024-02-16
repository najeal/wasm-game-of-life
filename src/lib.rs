mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(message: &str) {
    alert(format!("Hello, wasm-game-of-life! {}", message).as_str());
}
