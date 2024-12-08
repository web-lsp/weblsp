use csslsrs::service::LanguageService;
use lsp_types::{FoldingRange, FoldingRangeKind, TextDocumentItem, Uri};
use std::str::FromStr;

#[test]
fn test_folding_ranges_empty() {
    assert_folding_ranges("", vec![]);
}

#[test]
fn test_folding_ranges_single_rule() {
    let css_text = "body {\n    margin: 0;\n    padding: 0;\n}\n";
    let expected_ranges = vec![FoldingRange {
        start_line: 0,
        end_line: 3,
        kind: None,
        ..Default::default()
    }];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_multiple_rules() {
    let css_text = "body {\n    margin: 0;\n}\n\nh1 {\n    color: red;\n}\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 2,
            kind: None,
            ..Default::default()
        },
        FoldingRange {
            start_line: 4,
            end_line: 6,
            kind: None,
            ..Default::default()
        },
    ];

    // Since the order might vary, sort expected ranges by start_line
    let mut expected_sorted = expected_ranges.clone();
    expected_sorted.sort_by_key(|fr| fr.start_line);

    assert_folding_ranges(css_text, expected_sorted);
}

#[test]
fn test_folding_ranges_nested_rules() {
    let css_text = "@media screen {\n    body {\n        margin: 0;\n    }\n}\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 4,
            kind: None,
            ..Default::default()
        },
        FoldingRange {
            start_line: 1,
            end_line: 3,
            kind: None,
            ..Default::default()
        },
    ];

    // Sort expected ranges by start_line
    let mut expected_sorted = expected_ranges.clone();
    expected_sorted.sort_by_key(|fr| fr.start_line);

    assert_folding_ranges(css_text, expected_sorted);
}

#[test]
fn test_folding_ranges_single_line_rule() {
    let css_text = "h1 { color: blue; }\n";
    let expected_ranges = vec![];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_with_comments() {
    let css_text = "/* Comment block\nspanning multiple lines\n*/\nbody {\n    margin: 0;\n}\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 2,
            kind: Some(FoldingRangeKind::Comment),
            ..Default::default()
        },
        FoldingRange {
            start_line: 3,
            end_line: 5,
            kind: None,
            ..Default::default()
        },
    ];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_complex() {
    let css_text = "@media screen {\n    @supports (display: grid) {\n        .container {\n            display: grid;\n        }\n    }\n}\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 6,
            kind: None,
            ..Default::default()
        },
        FoldingRange {
            start_line: 1,
            end_line: 5,
            kind: None,
            ..Default::default()
        },
        FoldingRange {
            start_line: 2,
            end_line: 4,
            kind: None,
            ..Default::default()
        },
    ];

    // Sort expected ranges by start_line
    let mut expected_sorted = expected_ranges.clone();
    expected_sorted.sort_by_key(|fr| fr.start_line);

    assert_folding_ranges(css_text, expected_sorted);
}

#[test]
fn test_folding_ranges_with_region_comments() {
    // Create a CSS document with #region and #endregion markers in comments
    let css_text = "/* #region Header */\n.header {\n    background: blue;\n}\n/* #endregion */\n\n/* #region Footer */\n.footer {\n    background: green;\n}\n/* #endregion */\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 4,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        },
        FoldingRange {
            start_line: 1,
            end_line: 3,
            kind: None,
            ..Default::default()
        },
        FoldingRange {
            start_line: 6,
            end_line: 10,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        },
        FoldingRange {
            start_line: 7,
            end_line: 9,
            kind: None,
            ..Default::default()
        },
    ];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_closing_brace_same_line() {
    let css_text = "body {\n    margin: 0;\n    padding: 0; }\n";
    let expected_ranges = vec![FoldingRange {
        start_line: 0,
        end_line: 2,
        kind: None,
        ..Default::default()
    }];

    assert_folding_ranges(css_text, expected_ranges);
}

// When the CSS ends abruptly (EOF) with unclosed comments or blocks, the folding ranges
// should still be extended to the last line of the document, without panicking.
#[test]
fn test_folding_ranges_handles_eof_correctly() {
    let css_text = "body {\n    margin: 0;\n    padding: 0;\n/* Unclosed comment\n continuation\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 4,
            kind: None,
            ..Default::default()
        },
        FoldingRange {
            start_line: 3,
            end_line: 4,
            kind: Some(FoldingRangeKind::Comment),
            ..Default::default()
        },
    ];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_mixed_delimiters() {
    let css_text = "/* #region Styles */\nbody {\n    /* Comment inside body */\n    margin: 0;\n}\n/* #endregion */\n\n/* #region Utilities */\n.utility {\n    padding: 10px;\n}\n/* #endregion */\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 5,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        },
        FoldingRange {
            start_line: 1,
            end_line: 4,
            kind: None,
            ..Default::default()
        },
        FoldingRange {
            start_line: 7,
            end_line: 11,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        },
        FoldingRange {
            start_line: 8,
            end_line: 10,
            kind: None,
            ..Default::default()
        },
    ];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_single_line_comments_without_region() {
    let css_text = "/* This is a single-line comment */\nbody {\n    margin: 0;\n}\n/* Another single-line comment */\n";
    let expected_ranges = vec![FoldingRange {
        start_line: 1,
        end_line: 3,
        kind: None,
        ..Default::default()
    }];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_nested_regions() {
    let css_text = "/* #region Outer Region */\nbody {\n    /* #region Inner Region */\n    margin: 0;\n    /* #endregion */\n}\n/* #endregion */\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 6,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        },
        FoldingRange {
            start_line: 1,
            end_line: 5,
            kind: None,
            ..Default::default()
        },
        FoldingRange {
            start_line: 2,
            end_line: 4,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        },
    ];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_comments_with_mixed_content() {
    let css_text = "/* #region Header */\n/* This is the header section */\n.header {\n    background: blue;\n}\n/* #endregion */\n\n/* Regular comment without region */\n.footer {\n    background: green;\n}\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 5,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        },
        FoldingRange {
            start_line: 2,
            end_line: 4,
            kind: None,
            ..Default::default()
        },
        FoldingRange {
            start_line: 8,
            end_line: 10,
            kind: None,
            ..Default::default()
        },
    ];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_without_opening_brace() {
    let css_text = "body\n    margin: 0;\n    padding: 0;\n}";
    let expected_ranges = vec![];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_declaration_intersecting_with_region() {
    let css_text = "/* #region Header */\n.header {\n    color: red;\n}\n/* #endregion */\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 4,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        },
        FoldingRange {
            start_line: 1,
            end_line: 3,
            kind: None,
            ..Default::default()
        },
    ];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_region_end_without_start() {
    let css_text = ".footer {\n    background: green;\n}\n/* #endregion */\n";
    let expected_ranges = vec![FoldingRange {
        start_line: 0,
        end_line: 2,
        kind: None,
        ..Default::default()
    }];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_no_indentation() {
    let css_text = "body {\nmargin: 0;\npadding: 0;\n}\nh1 {\ncolor: blue;\n}\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 3,
            kind: None,
            ..Default::default()
        },
        FoldingRange {
            start_line: 4,
            end_line: 6,
            kind: None,
            ..Default::default()
        },
    ];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_opening_brace_new_line() {
    let css_text = "body\n{\n    margin: 0;\n    padding: 0;\n}\n";
    let expected_ranges = vec![FoldingRange {
        start_line: 1,
        end_line: 4,
        kind: None,
        ..Default::default()
    }];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_comment_wrong_indentation_no_newline() {
    let css_text = "/*#region Comment */\nbody {\n    margin: 0;\n}\n/*#endregion */";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 4,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        },
        FoldingRange {
            start_line: 1,
            end_line: 3,
            kind: None,
            ..Default::default()
        },
    ];

    assert_folding_ranges(css_text, expected_ranges);
}

#[test]
fn test_folding_ranges_simple_region_without_spaces() {
    let css_text = "/*#regionHeader*/\n.header {\n    background: blue;\n}\n/*#endregion*/\n";
    let expected_ranges = vec![
        FoldingRange {
            start_line: 0,
            end_line: 4,
            kind: Some(FoldingRangeKind::Region),
            ..Default::default()
        },
        FoldingRange {
            start_line: 1,
            end_line: 3,
            kind: None,
            ..Default::default()
        },
    ];

    assert_folding_ranges(css_text, expected_ranges);
}

/// Utility function to assert folding ranges for a given CSS document.
///
/// # Parameters
/// - `text`: The CSS document text
/// - `expected_ranges`: The expected folding ranges
///
/// # Panics
/// Panics if the actual folding ranges do not match the expected ranges.
fn assert_folding_ranges(text: &str, expected_ranges: Vec<FoldingRange>) {
    let mut ls = LanguageService::default();

    let document = TextDocumentItem::new(
        Uri::from_str("file:///test.css").unwrap(),
        "css".to_string(),
        1,
        text.to_string(),
    );

    ls.upsert_document(document.clone());

    let mut folding_ranges = ls.get_folding_ranges(document);

    // Sort both actual and expected ranges by start_line for consistent comparison
    folding_ranges.sort_by_key(|fr| fr.start_line);
    let mut expected_sorted = expected_ranges.clone();
    expected_sorted.sort_by_key(|fr| fr.start_line);

    assert_eq!(
        folding_ranges.len(),
        expected_sorted.len(),
        "Unexpected number of folding ranges"
    );

    for (folding_range, expected) in folding_ranges.iter().zip(expected_sorted.iter()) {
        assert_eq!(
            folding_range.start_line, expected.start_line,
            "Start line does not match"
        );
        assert_eq!(
            folding_range.end_line, expected.end_line,
            "End line does not match"
        );
        assert_eq!(
            folding_range.kind, expected.kind,
            "Folding range kind does not match"
        );
    }
}
