use biome_css_parser::CssParse;
use biome_css_syntax::{CssLanguage, CssSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode};
use lsp_types::{Color, ColorInformation, ColorPresentation, Position, Range, TextDocumentItem};

use crate::service::LanguageService;

pub fn extract_colors_information(node: &SyntaxNode<CssLanguage>) -> Vec<ColorInformation> {
    let mut results = Vec::new();

    // TODO: Also support normal CSS identifiers (e.g. "red", "blue", etc.)
    if node.kind() == CssSyntaxKind::CSS_COLOR {
        results.push(ColorInformation {
            // TODO: Extract color information
            color: Color {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            },
            // TODO: Extract range information
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
        });
    }

    for child in node.children() {
        results.extend(extract_colors_information(&child));
    }

    results
}

fn find_document_colors(css: &CssParse) -> Vec<ColorInformation> {
    let binding = css.tree().rules();
    extract_colors_information(binding.syntax())
}

impl LanguageService {
    pub fn get_document_colors(&mut self, document: TextDocumentItem) -> Vec<ColorInformation> {
        let store_entry = self.store.get_or_update_document(document);
        find_document_colors(&store_entry.css_tree)
    }

    pub fn get_color_presentations(
        self,
        _document: TextDocumentItem,
        _color: ColorInformation,
        _range: Range,
    ) -> Vec<ColorPresentation> {
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
    use super::*;
    use crate::parser::parse_css;
    use lsp_types::Uri;
    use std::str::FromStr;

    #[test]
    fn test_find_document_colors_basic() {
        let document = TextDocumentItem {
            uri: Uri::from_str("file:///test.css").unwrap(),
            language_id: "css".to_string(),
            version: 1,
            text: "h1 { color: #000; }".to_string(),
        };

        let css = parse_css(&document.text);
        let document_colors = find_document_colors(&css);

        assert!(
            document_colors.len() == 1,
            "Expected 1 color, found {}",
            document_colors.len()
        );
    }
}
