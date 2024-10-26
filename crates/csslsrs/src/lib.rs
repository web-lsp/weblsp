pub mod analyzer;
pub mod parser;

pub mod service;
pub mod store;

pub mod features {
    pub mod folding;
}

#[cfg(target_arch = "wasm32")]
pub mod wasm_text_document;
