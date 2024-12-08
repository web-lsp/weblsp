use lsp_types::{TextDocumentItem, Uri};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

/// Convert a JS object to a TextDocumentItem.
pub fn create_text_document(js_value: JsValue) -> TextDocumentItem {
    let js_text_document: VSCodeTextDocument = serde_wasm_bindgen::from_value(js_value).unwrap();

    js_text_document.into()
}

impl From<VSCodeTextDocument> for TextDocumentItem {
    fn from(val: VSCodeTextDocument) -> Self {
        TextDocumentItem {
            uri: val.uri,
            language_id: val.language_id,
            version: val.version,
            text: val.content,
        }
    }
}

/// VS Code's `vscode-languageserver-textdocument` has a slightly different representation of a text document than lsp-types, as such for the serialization
/// and deserialization of text documents we need to use a custom struct in between. Bit annoying but it works.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct VSCodeTextDocument {
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
