use crate::{
    converters::{line_index::LineIndex, to_proto::range, PositionEncoding},
    service::LanguageService,
};
use biome_css_syntax::{CssLanguage, CssSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode};
use lsp_types::{DocumentSymbol, SymbolKind, TextDocumentItem};

// Recursively extracts document symbols from the given CSS node.
fn extract_document_symbols(
    node: &SyntaxNode<CssLanguage>,
    line_index: &LineIndex,
    encoding: PositionEncoding,
) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();

    for child in node.children() {
        let symbol = match child.kind() {
            CssSyntaxKind::CSS_QUALIFIED_RULE => Some(create_symbol(
                child.text().to_string().trim(),
                SymbolKind::CLASS,
                &child,
                line_index,
                encoding,
            )),
            CssSyntaxKind::CSS_AT_RULE => Some(create_symbol(
                child.text().to_string().trim(),
                SymbolKind::NAMESPACE,
                &child,
                line_index,
                encoding,
            )),
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
    name: &str,
    kind: SymbolKind,
    node: &SyntaxNode<CssLanguage>,
    line_index: &LineIndex,
    encoding: PositionEncoding,
) -> DocumentSymbol {
    let node_range = range(line_index, node.text_range(), encoding).unwrap();

    // TMP: deprecated is deprecated, but—for now—we still need to intialize it to None, and hide the warning.
    #[allow(deprecated)]
    DocumentSymbol {
        name: name.to_string(),
        detail: None,
        kind,
        tags: None,
        deprecated: None,
        range: node_range,
        selection_range: node_range,
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
