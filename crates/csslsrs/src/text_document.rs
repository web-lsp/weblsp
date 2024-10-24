use wasm_bindgen::prelude::*;

/// VSCode-like object that represents a text document.
#[wasm_bindgen]
pub struct TextDocument {
    #[wasm_bindgen(skip)]
    pub uri: String,
    #[wasm_bindgen(skip)]
    pub language_id: String,
    pub version: i64,
    #[wasm_bindgen(skip)]
    pub text: String,
}

#[wasm_bindgen]
impl TextDocument {
    /// Creates a new `TextDocument` object.
    ///
    /// # Arguments
    ///
    /// * `uri` - A string slice that holds the URI of the text document.
    /// * `language_id` - A string slice that holds the language ID of the text document.
    /// * `version` - An integer that holds the version of the text document.
    /// * `text` - A string slice that holds the text of the document.
    ///
    /// # Returns
    ///
    /// * A `TextDocument` object.
    #[wasm_bindgen(constructor)]
    pub fn new(uri: &str, language_id: &str, version: i64, text: &str) -> TextDocument {
        TextDocument {
            uri: uri.to_string(),
            language_id: language_id.to_string(),
            version,
            text: text.to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn uri(&self) -> String {
        self.uri.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn language_id(&self) -> String {
        self.language_id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }
}
