#![allow(deprecated)]
// TMP: deprecated is deprecated in Document Symbol, but we still need to intialize it to None, and hide the warning.
use csslsrs::service::LanguageService;
use lsp_types::{DocumentSymbol, Position, Range, SymbolKind, SymbolTag, TextDocumentItem, Uri};
use pretty_assertions::assert_eq;
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
            detail: Some("(max-width: 768px)".to_string()),
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

#[test]
fn test_css_variables() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        ":root {--color-primary: #333;}h1 {color: var(--color-primary);}",
        vec![
            DocumentSymbol {
                name: ":root".to_string(),
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
                        character: 30,
                    },
                },
                selection_range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 5,
                    },
                },
                children: Some(vec![DocumentSymbol {
                    name: "--color-primary".to_string(),
                    detail: None,
                    kind: SymbolKind::VARIABLE,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 7,
                        },
                        end: Position {
                            line: 0,
                            character: 28,
                        },
                    },
                    selection_range: Range {
                        start: Position {
                            line: 0,
                            character: 7,
                        },
                        end: Position {
                            line: 0,
                            character: 22,
                        },
                    },
                    children: None,
                }]),
            },
            DocumentSymbol {
                name: "h1".to_string(),
                detail: None,
                kind: SymbolKind::CLASS,
                tags: None,
                deprecated: None,
                range: Range {
                    start: Position {
                        line: 0,
                        character: 30,
                    },
                    end: Position {
                        line: 0,
                        character: 63,
                    },
                },
                selection_range: Range {
                    start: Position {
                        line: 0,
                        character: 30,
                    },
                    end: Position {
                        line: 0,
                        character: 32,
                    },
                },
                children: Some(vec![DocumentSymbol {
                    name: "color".to_string(),
                    detail: None,
                    kind: SymbolKind::PROPERTY,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 34,
                        },
                        end: Position {
                            line: 0,
                            character: 61,
                        },
                    },
                    selection_range: Range {
                        start: Position {
                            line: 0,
                            character: 34,
                        },
                        end: Position {
                            line: 0,
                            character: 39,
                        },
                    },
                    children: None,
                }]),
            },
        ],
    );
}

#[test]
fn test_at_rule_with_nested_properties() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        "@font-palette-values --my-palette {\n  font-family: Bixa;\n  base-palette: 1;\n  override-colors: 0 #ff0000;\n}",
        vec![DocumentSymbol {
            name: "@font-palette-values".to_string(),
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
                    line: 4,
                    character: 1,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 20,
                },
            },
            children: Some(vec![
                DocumentSymbol {
                    name: "font-family".to_string(),
                    detail: None,
                    kind: SymbolKind::PROPERTY,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position {
                            line: 1,
                            character: 2,
                        },
                        end: Position {
                            line: 1,
                            character: 19,
                        },
                    },
                    selection_range: Range {
                        start: Position {
                            line: 1,
                            character: 2,
                        },
                        end: Position {
                            line: 1,
                            character: 13,
                        },
                    },
                    children: None,
                },
                DocumentSymbol {
                    name: "base-palette".to_string(),
                    detail: None,
                    kind: SymbolKind::PROPERTY,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position {
                            line: 2,
                            character: 2,
                        },
                        end: Position {
                            line: 2,
                            character: 17,
                        },
                    },
                    selection_range: Range {
                        start: Position {
                            line: 2,
                            character: 2,
                        },
                        end: Position {
                            line: 2,
                            character: 14,
                        },
                    },
                    children: None,
                },
                DocumentSymbol {
                    name: "override-colors".to_string(),
                    detail: None,
                    kind: SymbolKind::PROPERTY,
                    tags: None,
                    deprecated: None,
                    range: Range {
                        start: Position {
                            line: 3,
                            character: 2,
                        },
                        end: Position {
                            line: 3,
                            character: 28,
                        },
                    },
                    selection_range: Range {
                        start: Position {
                            line: 3,
                            character: 2,
                        },
                        end: Position {
                            line: 3,
                            character: 17,
                        },
                    },
                    children: None,
                },
            ]),
            }],
    );
}
