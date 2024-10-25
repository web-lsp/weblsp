use serde::{Deserialize, Serialize};

/// VSCode-like object that represents a text document.
#[derive(Serialize, Deserialize)]
pub struct TextDocument {
    pub uri: String,
    pub language_id: String,
    pub version: i64,
    pub text: String,
}

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
    pub fn new(uri: &str, language_id: &str, version: i64, text: &str) -> TextDocument {
        TextDocument {
            uri: uri.to_string(),
            language_id: language_id.to_string(),
            version,
            text: text.to_string(),
        }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn language_id(&self) -> &str {
        &self.language_id
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use super::TextDocument;
    use serde_wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn create_text_document(js_value: JsValue) -> JsValue {
        let doc: TextDocument = serde_wasm_bindgen::from_value(js_value).unwrap();
        serde_wasm_bindgen::to_value(&doc).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_text_document_uri(js_value: JsValue) -> JsValue {
        let doc: TextDocument = serde_wasm_bindgen::from_value(js_value).unwrap();
        serde_wasm_bindgen::to_value(&doc.uri).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_text_document_language_id(js_value: JsValue) -> JsValue {
        let doc: TextDocument = serde_wasm_bindgen::from_value(js_value).unwrap();
        serde_wasm_bindgen::to_value(&doc.language_id).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_text_document_text(js_value: JsValue) -> JsValue {
        let doc: TextDocument = serde_wasm_bindgen::from_value(js_value).unwrap();
        serde_wasm_bindgen::to_value(&doc.text).unwrap()
    }
}
