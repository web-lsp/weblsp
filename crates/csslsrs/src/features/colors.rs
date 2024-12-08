use biome_css_parser::CssParse;
use biome_css_syntax::{CssLanguage, CssSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode};
use lsp_types::{ColorInformation, ColorPresentation, TextDocumentItem, TextEdit};
use palette::{named, Hsla, Hwba, Laba, Lcha, Srgba, WithAlpha};

use crate::{
    converters::{line_index::LineIndex, to_proto::range, PositionEncoding},
    features::color_parser::OPAQUE,
    service::LanguageService,
};

use super::color_parser::{parse_css_color, CSSColor};

fn extract_colors_information(
    node: &SyntaxNode<CssLanguage>,
    line_index: &LineIndex,
    encoding: PositionEncoding,
) -> Vec<ColorInformation> {
    let mut colors = Vec::new();

    // PERF: This implementation will traverse the entire tree of the CSS file, matching many unnecessary nodes.
    // A more efficient implementation would instead only look for colors in relevant contexts (e.g. CSS values, function parameters etc.)

    node.children().for_each(|child| {
        match child.kind() {
            CssSyntaxKind::CSS_FUNCTION => {
                // CSS functions come in many forms and shapes, and are sometimes colors themselves or container of other colors
                // In our case, we only care about functions that are colors (rgb, hsl, etc.) as the other branches will cover
                // functions that contain colors (e.g. linear-gradient, light-dark, etc.)
                if let Some(function_name) = child.first_child().map(|n| n.text().to_string()) {
                    if matches!(
                        function_name.as_str(),
                        "rgb" | "rgba" | "hsl" | "hsla" | "hwb" | "hwba" | "lab" | "lch"
                    ) {
                        if let Some(color) = parse_css_color(&node.text().to_string()) {
                            colors.push(ColorInformation {
                                color: color.to_lsp_color(),
                                range: range(line_index, node.text_range(), encoding).unwrap(),
                            });
                        }
                    }
                }
            }
            // Any CSS identifier, such as a property name or basic value (ex: `color: red;` contains two identifiers)
            CssSyntaxKind::CSS_IDENTIFIER => {
                if let Some(color) = named::from_str(&node.text().to_string().to_lowercase()) {
                    let color: Srgba = color.into_format().into();
                    colors.push(ColorInformation {
                        color: color.to_lsp_color(),
                        range: range(line_index, node.text_range(), encoding).unwrap(),
                    });
                }
            }
            // HEX colors
            CssSyntaxKind::CSS_COLOR => {
                if let Some(color) = parse_css_color(&node.text().to_string()) {
                    colors.push(ColorInformation {
                        color: color.to_lsp_color(),
                        range: range(line_index, node.text_range(), encoding).unwrap(),
                    });
                }
            }
            _ => {}
        }
    });

    // TODO: Handle CSS variables
    // If a reference to a CSS variable that contains a color is found, we can provide a document color for the variable where it is used

    for child in node.children() {
        colors.extend(extract_colors_information(&child, line_index, encoding));
    }

    colors
}

fn find_document_colors(
    css: &CssParse,
    line_index: &LineIndex,
    encoding: PositionEncoding,
) -> Vec<ColorInformation> {
    let binding = css.tree().rules();
    extract_colors_information(binding.syntax(), line_index, encoding)
}

fn compute_color_presentations(color: ColorInformation) -> Vec<ColorPresentation> {
    let rgb_color = Srgba::from_lsp_color(color.color);

    let mut color_texts: Vec<String> = vec![];

    // Check for a named color with the same values
    let named_color = named::colors()
        .position(|color| color == rgb_color.into_format::<u8, u8>().without_alpha());
    let named_color_string = named_color.map(|i| named::names().nth(i).unwrap());

    if let Some(named_color_string) = named_color_string {
        color_texts.push(named_color_string.to_string());
    }

    color_texts.extend(vec![
        rgb_color.to_css(),
        // Hex representation
        {
            let u8_color: Srgba<u8> = rgb_color.into_format();
            if rgb_color.alpha == OPAQUE {
                format!("#{:02x}", u8_color.without_alpha())
            } else {
                format!("#{:02x}", u8_color)
            }
        },
        Hsla::from_lsp_color(color.color).to_css(),
        Hwba::from_lsp_color(color.color).to_css(),
        Laba::from_lsp_color(color.color).to_css(),
        Lcha::from_lsp_color(color.color).to_css(),
    ]);

    color_texts
        .into_iter()
        .map(|text| ColorPresentation {
            label: text.clone(),
            text_edit: Some(TextEdit {
                range: color.range,
                new_text: text,
            }),
            additional_text_edits: None,
        })
        .collect()
}

impl LanguageService {
    pub fn get_document_colors(&self, document: TextDocumentItem) -> Vec<ColorInformation> {
        let store_entry = self.store.get(&document.uri);

        match store_entry {
            Some(store_entry) => find_document_colors(
                &store_entry.css_tree,
                &store_entry.line_index,
                self.options.encoding,
            ),
            None => {
                vec![]
            }
        }
    }

    pub fn get_color_presentations(&self, color: ColorInformation) -> Vec<ColorPresentation> {
        compute_color_presentations(color)
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use std::str::FromStr;

    use crate::service::wasm_bindings::WASMLanguageService;

    use super::{compute_color_presentations, find_document_colors};
    use lsp_types::Uri;
    use serde_wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"
        declare function get_document_colors(documentUri: string): import("vscode-languageserver-types").ColorInformation[];
        declare function get_color_presentations(color: import("vscode-languageserver-types").ColorInformation): import("vscode-languageserver-types").ColorPresentation[];
    "#;

    #[wasm_bindgen]
    impl WASMLanguageService {
        #[wasm_bindgen(skip_typescript, js_name = getDocumentColors)]
        pub fn get_document_colors(&self, document_uri: String) -> JsValue {
            let store_document = self.store.get(&Uri::from_str(&document_uri).unwrap());

            let document_colors = match store_document {
                Some(store_document) => find_document_colors(
                    &store_document.css_tree,
                    &store_document.line_index,
                    self.options.encoding,
                ),
                None => vec![],
            };

            serde_wasm_bindgen::to_value(&document_colors).unwrap()
        }

        #[wasm_bindgen(skip_typescript, js_name = getColorPresentations)]
        pub fn get_color_presentations(&self, color: JsValue) -> JsValue {
            let color = serde_wasm_bindgen::from_value(color).unwrap();
            let color_presentations = compute_color_presentations(color);

            serde_wasm_bindgen::to_value(&color_presentations).unwrap()
        }
    }
}
