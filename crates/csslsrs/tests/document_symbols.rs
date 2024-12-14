#![allow(deprecated)]
// TMP: deprecated is deprecated in Document Symbol, but we still need to intialize it to None, and hide the warning.
use csslsrs::service::LanguageService;
use lsp_types::{DocumentSymbol, Position, Range, SymbolKind, SymbolTag, TextDocumentItem, Uri};
use std::str::FromStr;

fn assert_document_symbols(
    ls: &mut LanguageService,
    text: &str,
    expected_symbols: Vec<DocumentSymbol>,
) {
    let document = TextDocumentItem {
        uri: Uri::from_str("file:///test.css").unwrap(),
        language_id: "css".to_string(),
        version: 1,
        text: text.to_string(),
    };

    ls.upsert_document(document.clone());

    let symbols = ls.get_document_symbols(document).unwrap_or_default();

    assert_eq!(
        symbols.len(),
        expected_symbols.len(),
        "Unexpected number of document symbols"
    );

    for (symbol, expected) in symbols.iter().zip(expected_symbols.iter()) {
        assert_eq!(symbol, expected, "Unexpected document symbol");
    }
}

#[test]
fn test_universal_selector() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        "* {}",
        vec![DocumentSymbol {
            name: "*".to_string(),
            detail: None,
            kind: SymbolKind::CLASS,
            tags: None,
            deprecated: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 4,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 1,
                },
            },
            children: None,
        }],
    );
}

#[test]
fn test_element_selector() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        ".foo {}",
        vec![DocumentSymbol {
            name: ".foo".to_string(),
            detail: None,
            kind: SymbolKind::CLASS,
            tags: None,
            deprecated: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 7,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 4,
                },
            },
            children: None,
        }],
    );
}

#[test]
fn test_compound_selector() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        "h1.foo {}",
        vec![DocumentSymbol {
            name: "h1.foo".to_string(),
            detail: None,
            kind: SymbolKind::CLASS,
            tags: None,
            deprecated: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 9,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 6,
                },
            },
            children: None,
        }],
    );
}

#[test]
fn test_descendant_selector() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        "main h1.foo {}",
        vec![DocumentSymbol {
            name: "main h1.foo".to_string(),
            detail: None,
            kind: SymbolKind::CLASS,
            tags: None,
            deprecated: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 14,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 11,
                },
            },
            children: None,
        }],
    );
}

#[test]
fn test_complex_selector_in_list() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        "main > h1.foo, main#bar > button:hover {}",
        vec![DocumentSymbol {
            name: "main > h1.foo, main#bar > button:hover".to_string(),
            detail: None,
            kind: SymbolKind::CLASS,
            tags: None,
            deprecated: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 41,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 38,
                },
            },
            children: None,
        }],
    );
}

#[test]
fn test_nested_selectors() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        ".foo { .bar {} }",
        vec![DocumentSymbol {
            name: ".foo".to_string(),
            detail: None,
            kind: SymbolKind::CLASS,
            tags: None,
            deprecated: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 16,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 4,
                },
            },
            children: Some(vec![DocumentSymbol {
                name: ".bar".to_string(),
                detail: None,
                kind: SymbolKind::CLASS,
                tags: None,
                deprecated: None,
                range: Range {
                    start: Position {
                        line: 0,
                        character: 7,
                    },
                    end: Position {
                        line: 0,
                        character: 14,
                    },
                },
                selection_range: Range {
                    start: Position {
                        line: 0,
                        character: 7,
                    },
                    end: Position {
                        line: 0,
                        character: 11,
                    },
                },
                children: None,
            }]),
        }],
    );
}

#[test]
fn test_at_rule() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        "@media (max-width: 768px) {}",
        vec![DocumentSymbol {
            name: "@media".to_string(),
            detail: None,
            kind: SymbolKind::NAMESPACE,
            tags: None,
            deprecated: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 28,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 6,
                },
            },
            children: None,
        }],
    );
}

#[test]
fn test_selector_with_properties() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        ".foo { color: red; text-align: center; }",
        vec![DocumentSymbol {
            name: ".foo".to_string(),
            detail: None,
            kind: SymbolKind::CLASS,
            tags: None,
            deprecated: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 40,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 4,
                },
            },
            children: Some(vec![
                DocumentSymbol {
                    name: "color".to_string(),
                    detail: None,
                    kind: SymbolKind::PROPERTY,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 7,
                        },
                        end: Position {
                            line: 0,
                            character: 17,
                        },
                    },
                    selection_range: Range {
                        start: Position {
                            line: 0,
                            character: 7,
                        },
                        end: Position {
                            line: 0,
                            character: 12,
                        },
                    },
                    children: None,
                },
                DocumentSymbol {
                    name: "text-align".to_string(),
                    detail: None,
                    kind: SymbolKind::PROPERTY,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 19,
                        },
                        end: Position {
                            line: 0,
                            character: 37,
                        },
                    },
                    selection_range: Range {
                        start: Position {
                            line: 0,
                            character: 19,
                        },
                        end: Position {
                            line: 0,
                            character: 29,
                        },
                    },
                    children: None,
                },
            ]),
        }],
    );
}

#[test]
fn test_deprecated_property() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        ".foo { box-align: center; }",
        vec![DocumentSymbol {
            name: ".foo".to_string(),
            detail: None,
            kind: SymbolKind::CLASS,
            tags: None,
            deprecated: None,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 27,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 4,
                },
            },
            children: Some(vec![DocumentSymbol {
                name: "box-align".to_string(),
                detail: None,
                kind: SymbolKind::PROPERTY,
                tags: Some(vec![SymbolTag::DEPRECATED]),
                deprecated: None,
                range: Range {
                    start: Position {
                        line: 0,
                        character: 7,
                    },
                    end: Position {
                        line: 0,
                        character: 24,
                    },
                },
                selection_range: Range {
                    start: Position {
                        line: 0,
                        character: 7,
                    },
                    end: Position {
                        line: 0,
                        character: 16,
                    },
                },
                children: None,
            }]),
        }],
    );
}
