use csslsrs::service::LanguageService;
use lsp_types::{DocumentSymbol, Position, Range, SymbolKind, TextDocumentItem, Uri};
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
fn test_class_selector() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        ".foo { color: red; }",
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
                    character: 18,
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
fn test_id_selector() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        "#foo { color: blue; }",
        vec![DocumentSymbol {
            name: "#foo".to_string(),
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
                    character: 18,
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
fn test_nested_selectors() {
    let mut ls = LanguageService::default();

    assert_document_symbols(
        &mut ls,
        ".foo { .bar { color: green; } }",
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
                    character: 29,
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
                        character: 6,
                    },
                    end: Position {
                        line: 0,
                        character: 27,
                    },
                },
                selection_range: Range {
                    start: Position {
                        line: 0,
                        character: 6,
                    },
                    end: Position {
                        line: 0,
                        character: 10,
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
        "@media screen { .foo { color: black; } }",
        vec![DocumentSymbol {
            name: "@media screen".to_string(),
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
                    character: 38,
                },
            },
            selection_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 13,
                },
            },
            children: Some(vec![DocumentSymbol {
                name: ".foo".to_string(),
                detail: None,
                kind: SymbolKind::CLASS,
                tags: None,
                deprecated: None,
                range: Range {
                    start: Position {
                        line: 0,
                        character: 15,
                    },
                    end: Position {
                        line: 0,
                        character: 36,
                    },
                },
                selection_range: Range {
                    start: Position {
                        line: 0,
                        character: 15,
                    },
                    end: Position {
                        line: 0,
                        character: 19,
                    },
                },
                children: None,
            }]),
        }],
    );
}
