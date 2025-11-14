use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub fn hello() -> String {
    "Hola desde Rust WASM!".to_string()
}
