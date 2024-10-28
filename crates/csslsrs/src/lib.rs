pub mod analyzer;
pub mod parser;

pub mod service;
pub mod store;

pub mod features {
    pub mod colors;
    pub mod folding;
}

#[cfg(feature = "wasm")]
pub mod wasm_text_document;

mod converters;
