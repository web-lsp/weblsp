use biome_css_parser::CssParse;
use lsp_types::{ColorInformation, ColorPresentation, Range, TextDocumentItem};

use crate::{service::LanguageService, store::StoreEntry};

fn find_document_colors(css: &CssParse) -> Vec<ColorInformation> {
    vec![]
}

fn find_color_presentations(
    _document: &TextDocumentItem,
    _color: ColorInformation,
    _range: Range,
) -> Vec<ColorPresentation> {
    todo!()
}

impl LanguageService {
    pub fn get_document_colors(&mut self, document: TextDocumentItem) -> Vec<ColorInformation> {
        let store_entry: &StoreEntry = self.store.update_document(document);

        find_document_colors(store_entry.css_tree.as_ref().unwrap())
    }

    pub fn get_color_presentations(
        self,
        _document: TextDocumentItem,
        _color: ColorInformation,
        _range: Range,
    ) -> Vec<ColorPresentation> {
        //let store_entry = self.store.insert_or_get(document);
        // find_color_presentations(&store_entry, color, range)

        todo!();
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use crate::parser::parse_css;

    use super::{find_color_presentations, find_document_colors};
    use serde_wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"export async function get_document_colors(source: import("vscode-languageserver-textdocument").TextDocument): Promise<import("vscode-languageserver-types").ColorInformation[]>;"#;

    #[wasm_bindgen(skip_typescript)]
    pub fn get_document_colors(document: JsValue) -> JsValue {
        let parsed_text_document = crate::wasm_text_document::create_text_document(document);
        let document_colors = find_document_colors(&parse_css(&parsed_text_document.text));

        serde_wasm_bindgen::to_value(&document_colors).unwrap()
    }

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"export async function get_color_presentations(source: import("vscode-languageserver-textdocument").TextDocument, color: import("vscode-languageserver-types").ColorInformation, range: import("vscode-languageserver-types").Range): Promise<import("vscode-languageserver-types").ColorPresentation[]>;"#;

    #[wasm_bindgen(skip_typescript)]
    pub fn get_color_presentations(document: JsValue, color: JsValue, range: JsValue) -> JsValue {
        let parsed_text_document = crate::wasm_text_document::create_text_document(document);
        let parsed_color = serde_wasm_bindgen::from_value(color).unwrap();
        let parsed_range = serde_wasm_bindgen::from_value(range).unwrap();

        let color_presentations =
            find_color_presentations(&parsed_text_document, parsed_color, parsed_range);

        serde_wasm_bindgen::to_value(&color_presentations).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use lsp_types::Uri;

    use crate::parser::parse_css;

    use super::*;

    #[test]
    fn test_find_document_colors_empty() {
        let document = TextDocumentItem {
            uri: Uri::from_str("file:///test.css").unwrap(),
            language_id: "css".to_string(),
            version: 1,
            text: "".to_string(),
        };

        let css = parse_css(&document.text);
        let document_colors = find_document_colors(&css);

        assert!(
            document_colors.is_empty(),
            "No document colors should be returned for empty input"
        );
    }
}
