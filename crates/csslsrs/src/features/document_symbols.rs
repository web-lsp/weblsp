use crate::{
    converters::{
        line_index::LineIndex,
        to_proto::{position, range},
        PositionEncoding,
    },
    css_data::{CssCustomData, Status},
    service::LanguageService,
};
use biome_css_syntax::{CssLanguage, CssSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode, TextSize};
use lsp_types::{DocumentSymbol, Range, SymbolKind, SymbolTag, TextDocumentItem};

// From a given CSS node, recursively extract document symbols based on their kind in the CSS Syntax Tree.
fn extract_document_symbols(
    node: &SyntaxNode<CssLanguage>,
    line_index: &LineIndex,
    encoding: PositionEncoding,
    custom_data: &Vec<&CssCustomData>,
) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();
    for child in node.children() {
        let symbol: Option<DocumentSymbol> = match child.kind() {
            // Handle CSS at-rules, e.g. `@media`, `@keyframes`, etc.
            CssSyntaxKind::CSS_AT_RULE => child.first_child().and_then(|at_rule| {
                at_rule.first_token().map(|token| {
                    create_symbol(
                        String::from("@") + token.text_trimmed(),
                        // For some at-rules, we want to include more details, e.g. `@media` should include the media query list.
                        at_rule
                            .first_child()
                            .filter(|child| child.kind() == CssSyntaxKind::CSS_MEDIA_QUERY_LIST)
                            .map(|child| child.text_trimmed().to_string()),
                        SymbolKind::NAMESPACE,
                        range(line_index, child.text_trimmed_range(), encoding).unwrap(),
                        Range::new(
                            position(
                                line_index,
                                token.text_trimmed_range().start() - TextSize::from(1),
                                encoding,
                            )
                            .unwrap(),
                            position(line_index, token.text_trimmed_range().end(), encoding)
                                .unwrap(),
                        ),
                        false,
                    )
                })
            }),
            CssSyntaxKind::CSS_GENERIC_PROPERTY => child.children().find_map(|c| {
                match c.kind() {
                    // Handle CSS variables, e.g. `--foo`, `--bar`, etc.
                    CssSyntaxKind::CSS_DASHED_IDENTIFIER => c.first_token().map(|property_node| {
                        create_symbol(
                            property_node.text_trimmed().to_string(),
                            None,
                            SymbolKind::VARIABLE,
                            range(line_index, child.text_trimmed_range(), encoding).unwrap(),
                            range(line_index, property_node.text_trimmed_range(), encoding)
                                .unwrap(),
                            false,
                        )
                    }),
                    // Handle CSS properties, e.g. `color`, `font-size`, etc.
                    CssSyntaxKind::CSS_IDENTIFIER => c.first_token().map(|property_node| {
                        create_symbol(
                            property_node.text_trimmed().to_string(),
                            None,
                            SymbolKind::PROPERTY,
                            range(line_index, child.text_trimmed_range(), encoding).unwrap(),
                            range(line_index, property_node.text_trimmed_range(), encoding)
                                .unwrap(),
                            is_property_deprecated(property_node.text_trimmed(), custom_data),
                        )
                    }),
                    _ => None,
                }
            }),
            // Handle CSS selectors, e.g. `.foo`, `#bar`, `div > span`, etc. Even when nested.
            CssSyntaxKind::CSS_QUALIFIED_RULE | CssSyntaxKind::CSS_NESTED_QUALIFIED_RULE => child
                .children()
                .find(|c| {
                    c.kind() == CssSyntaxKind::CSS_SELECTOR_LIST
                        || c.kind() == CssSyntaxKind::CSS_SUB_SELECTOR_LIST
                        || c.kind() == CssSyntaxKind::CSS_RELATIVE_SELECTOR_LIST
                })
                .map(|selector| {
                    create_symbol(
                        selector.text_trimmed().to_string(),
                        None,
                        SymbolKind::CLASS,
                        range(line_index, child.text_trimmed_range(), encoding).unwrap(),
                        range(line_index, selector.text_trimmed_range(), encoding).unwrap(),
                        false,
                    )
                }),
            _ => None,
        };

        // If we have a symbol, search for nested children symbols.
        if let Some(mut sym) = symbol {
            let children_symbols =
                extract_document_symbols(&child, line_index, encoding, custom_data);
            if !children_symbols.is_empty() {
                sym.children = Some(children_symbols);
            }
            symbols.push(sym);
        } else {
            // Even if we don't have a symbol, we still want to search for nested symbols.
            let nested_symbols =
                extract_document_symbols(&child, line_index, encoding, custom_data);
            symbols.extend(nested_symbols);
        }
    }

    symbols
}

// Create a LSP `DocumentSymbol` based on the given parameters.
fn create_symbol(
    name: String,
    detail: Option<String>,
    kind: SymbolKind,
    range: lsp_types::Range,
    selection_range: lsp_types::Range,
    is_deprecated: bool,
) -> DocumentSymbol {
    // TMP: deprecated is deprecated, but—for now—we still need to intialize it to None, and hide the warning.
    #[allow(deprecated)]
    DocumentSymbol {
        name,
        detail,
        kind,
        tags: is_deprecated.then(|| vec![SymbolTag::DEPRECATED]),
        deprecated: None,
        range,
        selection_range,
        children: None,
    }
}

// Use the custom CSS data to determine if a given CSS property is deprecated.
fn is_property_deprecated(property: &str, custom_data: &[&CssCustomData]) -> bool {
    custom_data.iter().any(|data| {
        data.properties.as_ref().map_or(false, |properties| {
            properties
                .iter()
                .any(|prop| prop.name == property && matches!(prop.status, Some(Status::Obsolete)))
        })
    })
}

impl LanguageService {
    /// Extracts document symbols from the given CSS document.
    pub fn get_document_symbols(
        &mut self,
        document: TextDocumentItem,
    ) -> Option<Vec<DocumentSymbol>> {
        let store_entry = self.store.get(&document.uri);

        match store_entry {
            Some(store_entry) => Some(extract_document_symbols(
                store_entry.css_tree.tree().syntax(),
                &store_entry.line_index,
                self.options.encoding,
                &self.get_css_custom_data(),
            )),
            None => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use std::str::FromStr;

    use super::extract_document_symbols;
    use crate::service::wasm_bindings::WASMLanguageService;
    use biome_rowan::AstNode;
    use lsp_types::Uri;
    use serde_wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"
    export async function get_document_symbols(documentUri: string): import("vscode-languageserver-types").DocumentSymbol[];"#;

    #[wasm_bindgen]
    impl WASMLanguageService {
        #[wasm_bindgen(skip_typescript, js_name = getDocumentSymbols)]
        pub fn get_document_symbols(&self, document_uri: String) -> JsValue {
            let store_document = self.store.get(&Uri::from_str(&document_uri).unwrap());

            let symbols = match store_document {
                Some(store_document) => extract_document_symbols(
                    store_document.css_tree.tree().syntax(),
                    &store_document.line_index,
                    self.options.encoding,
                    &self.get_css_custom_data(),
                ),
                None => Vec::new(),
            };

            serde_wasm_bindgen::to_value(&symbols).unwrap()
        }
    }
}
