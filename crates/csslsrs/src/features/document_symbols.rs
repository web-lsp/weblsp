use crate::{
    converters::{
        line_index::LineIndex,
        to_proto::{position, range},
        PositionEncoding,
    },
    service::LanguageService,
};
use biome_css_syntax::{CssLanguage, CssSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode, TextSize};
use lsp_types::{DocumentSymbol, Range, SymbolKind, SymbolTag, TextDocumentItem};

// Recursively extracts document symbols from the given CSS node.
fn extract_document_symbols(
    node: &SyntaxNode<CssLanguage>,
    line_index: &LineIndex,
    encoding: PositionEncoding,
) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();
    for child in node.children() {
        let symbol: Option<DocumentSymbol> = match child.kind() {
            // Handle CSS at-rules, e.g. `@media`, `@keyframes`, etc.
            CssSyntaxKind::CSS_AT_RULE => child.first_child().and_then(|at_rule| {
                at_rule.first_token().map(|token| {
                    create_symbol(
                        String::from("@") + token.text_trimmed(),
                        SymbolKind::NAMESPACE,
                        range(line_index, child.text_range(), encoding).unwrap(),
                        Range::new(
                            position(
                                line_index,
                                token.text_range().start() - TextSize::from(1),
                                encoding,
                            )
                            .unwrap(),
                            position(line_index, token.text_range().end(), encoding).unwrap(),
                        ),
                        false,
                    )
                })
            }),
            // Handle CSS properties, e.g. `color`, `font-size`, etc.
            CssSyntaxKind::CSS_GENERIC_PROPERTY => child
                .children()
                .find(|c| c.kind() == CssSyntaxKind::CSS_IDENTIFIER)
                .and_then(|c| c.first_token())
                .map(|token| {
                    create_symbol(
                        token.text_trimmed().to_string(),
                        SymbolKind::PROPERTY,
                        range(line_index, child.text_range(), encoding).unwrap(),
                        range(line_index, token.text_range(), encoding).unwrap(),
                        false,
                    )
                }),
            // Handle CSS selectors, e.g. `.foo`, `#bar`, `div > span`, etc.
            CssSyntaxKind::CSS_QUALIFIED_RULE => child
                .children()
                .find(|c| c.kind() == CssSyntaxKind::CSS_SELECTOR_LIST)
                .map(|selector| {
                    create_symbol(
                        selector.text_trimmed().to_string(),
                        SymbolKind::CLASS,
                        range(line_index, child.text_range(), encoding).unwrap(),
                        range(line_index, selector.text_range(), encoding).unwrap(),
                        false,
                    )
                }),
            _ => None,
        };

        // If we have a symbol, search for nested children symbols.
        if let Some(mut sym) = symbol {
            let children_symbols = extract_document_symbols(&child, line_index, encoding);
            if !children_symbols.is_empty() {
                sym.children = Some(children_symbols);
            }
            symbols.push(sym);
        } else {
            // Even if we don't have a symbol, we still want to search for nested symbols.
            let nested_symbols = extract_document_symbols(&child, line_index, encoding);
            symbols.extend(nested_symbols);
        }
    }

    symbols
}

fn create_symbol(
    name: String,
    kind: SymbolKind,
    range: lsp_types::Range,
    selection_range: lsp_types::Range,
    is_deprecated: bool,
) -> DocumentSymbol {
    // TMP: deprecated is deprecated, but—for now—we still need to intialize it to None, and hide the warning.
    #[allow(deprecated)]
    DocumentSymbol {
        name,
        detail: None,
        kind,
        tags: is_deprecated.then(|| vec![SymbolTag::DEPRECATED]),
        deprecated: None,
        range,
        selection_range,
        children: None,
    }
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
            )),
            None => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use super::extract_document_symbols;
    use crate::{
        converters::{line_index::LineIndex, PositionEncoding},
        parser::parse_css,
        wasm_text_document::create_text_document,
    };
    use biome_rowan::AstNode;
    use serde_wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"
    export async function get_document_symbols(document: import("vscode-languageserver-textdocument").TextDocument): Promise<import("vscode-languageserver-types").DocumentSymbol[] | null>;"#;

    #[wasm_bindgen(skip_typescript)]
    pub fn get_document_symbols(document: JsValue) -> JsValue {
        let parsed_text_document = create_text_document(document);
        let css_parse = parse_css(&parsed_text_document.text);
        let line_index = LineIndex::new(&parsed_text_document.text);
        let encoding = PositionEncoding::Utf16;

        let symbols = extract_document_symbols(css_parse.tree().syntax(), &line_index, encoding);

        if symbols.is_empty() {
            JsValue::NULL
        } else {
            serde_wasm_bindgen::to_value(&symbols).unwrap()
        }
    }
}