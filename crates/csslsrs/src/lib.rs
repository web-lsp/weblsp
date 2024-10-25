#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

pub mod analyzer;
pub mod formatter;
pub mod parser;
pub mod text_document;
pub mod features {
    pub mod folding;
}
