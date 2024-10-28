use biome_css_parser::CssParse;
use biome_css_syntax::{CssLanguage, CssSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode};
use csscolorparser::{parse as parse_color, NAMED_COLORS};
use lsp_types::{Color, ColorInformation, ColorPresentation, Position, Range, TextDocumentItem};

use crate::service::LanguageService;

pub fn extract_colors_information(node: &SyntaxNode<CssLanguage>) -> Vec<ColorInformation> {
    let mut results = Vec::new();

    // PERF: This should probably only check CSS identifiers in relevant contexts (e.g. property that expects a color)
    // Good enough for now
    match node.kind() {
        CssSyntaxKind::CSS_IDENTIFIER => {
            if let Some(color) = NAMED_COLORS
                .get(&node.text().to_string())
                .map(|color| csscolorparser::Color::from_rgba8(color[0], color[1], color[2], 255))
            {
                results.push(ColorInformation {
                    color: Color {
                        red: color.r,
                        green: color.g,
                        blue: color.b,
                        alpha: color.a,
                    },
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
        }
        CssSyntaxKind::CSS_COLOR => {
            if let Ok(color) = parse_color(&node.text().to_string()) {
                results.push(ColorInformation {
                    color: Color {
                        red: color.r,
                        green: color.g,
                        blue: color.b,
                        alpha: color.a,
                    },
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
        }
        _ => {}
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
        todo!("Implement get_color_presentations in colors.rs");
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use crate::parser::parse_css;

    use super::find_document_colors;
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
    pub fn get_color_presentations(
        _document: JsValue,
        _color: JsValue,
        _range: JsValue,
    ) -> JsValue {
        todo!("Implement get_color_presentations in colors/wasm_bindings.rs");
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
            text: "h1 { color: #fff; }".to_string(),
        };

        let css = parse_css(&document.text);
        let document_colors = find_document_colors(&css);

        assert!(
            document_colors.len() == 1,
            "Expected 1 color, found {}",
            document_colors.len()
        );
        // assert_eq!(
        //     document_colors[0],
        //     ColorInformation {
        //         color: Color {
        //             red: 1.0,
        //             green: 1.0,
        //             blue: 1.0,
        //             alpha: 1.0,
        //         },
        //         range: Range {
        //             start: Position {
        //                 line: 0,
        //                 character: 12,
        //             },
        //             end: Position {
        //                 line: 0,
        //                 character: 16,
        //             },
        //         },
        //     },
        //     "Unexpected color information"
        // );
    }
}
