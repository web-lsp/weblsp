#[cfg(feature = "wasm")]
pub mod wasm_bindings {
    use lsp_types::{TextDocumentItem, Uri};
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen;
    use wasm_bindgen::prelude::*;

    pub fn create_text_document(js_value: JsValue) -> TextDocumentItem {
        let js_text_document: JSTextDocument = serde_wasm_bindgen::from_value(js_value).unwrap();

        TextDocumentItem {
            uri: js_text_document.uri,
            language_id: js_text_document.language_id,
            version: js_text_document.version,
            text: js_text_document.content,
        }
    }

    /// JS has a slightly different representation of a text document than lsp-types, as such for the serialization
    /// and deserialization of text documents we need to use a custom struct in between.
    #[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct JSTextDocument {
        /// The text document's URI.
        pub uri: Uri,

        /// The text document's language identifier.
        pub language_id: String,

        /// The version number of this document (it will strictly increase after each
        /// change, including undo/redo).
        pub version: i32,

        /// The content of the opened text document.
        #[serde(rename = "_content")]
        pub content: String,
    }
}
