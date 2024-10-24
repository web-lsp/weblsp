use wasm_bindgen::prelude::wasm_bindgen;

pub mod analyzer;
pub mod formatter;
pub mod parser;
pub mod text_document;
pub mod features {
    pub mod colors;
    pub mod folding;
}

pub mod types;

#[wasm_bindgen]
pub fn say_hello() {
    println!("Hello, world!");
}
