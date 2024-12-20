use csslsrs::service::LanguageService;
use lsp_types::{
    Hover, HoverContents, MarkupContent, MarkupKind, Position, Range, TextDocumentItem, Uri,
};
use pretty_assertions::assert_eq;
use std::str::FromStr;

/// Utility function to convert an offset to a position
fn offset_to_position(text: &str, offset: usize) -> Position {
    let mut line = 0;
    let mut character = 0;
    let mut current_offset = 0;

    for (idx, ch) in text.char_indices() {
        if current_offset == offset {
            return Position::new(line, character);
        }
        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }
        current_offset = idx + ch.len_utf8();
    }

    if current_offset == offset {
        return Position::new(line, character);
    }

    panic!("Offset out of bounds");
}

/// Utility function to test hover
fn assert_hover(text_with_cursor: &str, expected_hover: Hover) {
    let offset = text_with_cursor
        .find('|')
        .expect("Cursor position '|' not found in text");
    let text = text_with_cursor.replacen('|', "", 1);
    let mut ls = LanguageService::default();

    let document = TextDocumentItem::new(
        Uri::from_str("file:///test.css").unwrap(),
        "css".to_string(),
        1,
        text.clone(),
    );

    ls.upsert_document(document.clone());

    let position = offset_to_position(&text, offset);
    let hover = ls.get_hover(document, position);

    assert_eq!(
        hover,
        Some(expected_hover),
        "Hover did not match expected hover."
    );
}

#[test]
fn test_hover_over_color_property_inline() {
    let css_text = ".test { |color: blue; }";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "Sets the color of an element's text\n\nSupported by Edge 12, Firefox 1, Safari 1, Chrome 1, Internet Explorer 3, Opera 3.5.\n\nSyntax: `<color>`\n\n[MDN Reference](https://developer.mozilla.org/docs/Web/CSS/color), [Can I Use](https://caniuse.com/?search=color)\n\n".to_owned()
        }),
        range: Some(Range {
            start: Position { line: 0, character: 8 },
            end: Position { line: 0, character: 13 }
        }) };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_over_experimental_property() {
    let css_text = ".test { |align-tracks: center; }";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "🧪 *Experimental, use with caution.*\n\nThe align-tracks CSS property sets the alignment in the masonry axis for grid containers that have masonry in their block axis.\n\nSyntax: `[ normal | <baseline-position> | <content-distribution> | <overflow-position>? <content-position> ]#`\n\n".to_owned()
        }),
        range: Some(Range {
            start: Position { line: 0, character: 8 },
            end: Position { line: 0, character: 20 }
        }) };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_over_obsolete_property() {
    let css_text = ".test { |box-align: center; }";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "🚧 *Obsolete, consider using alternatives.*\n\nThe box-align CSS property specifies how an element aligns its contents across its layout in a perpendicular direction. The effect of the property is only visible if there is extra space in the box.\n\nSupported by Edge 12, Firefox 49, Safari 3, Chrome 1, Opera 15.\n\nSyntax: `start | center | end | baseline | stretch`\n\n[MDN Reference](https://developer.mozilla.org/docs/Web/CSS/box-align), [Can I Use](https://caniuse.com/?search=box-align)\n\n".to_owned()
        }),
        range: Some(Range {
            start: Position { line: 0, character: 8 },
            end: Position { line: 0, character: 17 }
        }) };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_over_non_standard_property() {
    let css_text = ".test { |-moz-border-bottom-colors: red; }";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "🚨 *Non-standard, avoid using it.*\n\nSets a list of colors for the bottom border.\n\nSupported by Firefox 1.\n\nSyntax: `<color>+ | none`\n\n".to_owned()
        }),
        range: Some(Range {
            start: Position { line: 0, character: 8 },
            end: Position { line: 0, character: 33 }
        }) };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_over_color_property() {
    let css_text = ".test {\n  col|or: blue;\n}\n";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "Sets the color of an element's text\n\nSupported by Edge 12, Firefox 1, Safari 1, Chrome 1, Internet Explorer 3, Opera 3.5.\n\nSyntax: `<color>`\n\n[MDN Reference](https://developer.mozilla.org/docs/Web/CSS/color), [Can I Use](https://caniuse.com/?search=color)\n\n".to_owned()
        }),
        range: Some(Range {
            start: Position { line: 1, character: 2 },
            end: Position { line: 1, character: 7 }
        }) };

    assert_hover(css_text, expected_hover);
}

// TODO: Selectors' hovers should have a HTML preview, but for now it's just the selector itself
#[test]
fn test_hover_over_universal_selector() {
    let css_text = "|* {}";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            // Universal selector does not have a specificity
            value: "**\\***\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (0, 0, 0)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 1 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_over_element_selector() {
    let css_text = ".|foo {}";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**.foo**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (0, 1, 0)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 4 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_over_compound_selector() {
    let css_text = "h1.|foo {}";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**h1.foo**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (0, 1, 1)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 6 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_over_descendant_selector() {
    let css_text = "main h1.|foo {}";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**main h1.foo**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (0, 1, 2)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 11 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_over_child_selector_in_list() {
    let css_text = "main > h1.|foo, main#bar > button:hover {}";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**main > h1.foo**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (0, 1, 2)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 13 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_over_complex_selector_in_list() {
    let css_text = "main > h1.foo, main#bar > button:|hover {}";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**main#bar > button:hover**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (1, 1, 2)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 15 },
            end: Position { line: 0, character: 38 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_with_escaped_characters() {
    let css_text = "#\\31| 23 {}"; // Id "123"
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**#\\31 23**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (1, 0, 0)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 7 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_with_unicode_characters() {
    let css_text = ".f\\00F|6o {}"; // Class "föo"
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**.f\\00F6o**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (0, 1, 0)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 8 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_with_escaped_brackets() {
    // Classes with [] are common in CSS, e.g. in TailwindCSS classes, so this test ensures they are handled correctly
    let css_text = ".color-\\[red\\]| {}"; // Class "color-[red]"
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**.color-\\\\[red\\\\]**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (0, 1, 0)\n\n" .to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 14 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_with_escaped_colon() {
    // Class with : are common in CSS, e.g. in TailwindCSS classes, so this test ensures they are handled correctly
    let css_text = ".sm\\:|text-red {}"; // Class "sm:text-red"
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**.sm\\:text-red**\n\n[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): (0, 1, 0)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 13 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_at_rule() {
    let css_text = "@med|ia screen and (min-width: 900px) {}";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "Defines a stylesheet for a particular media type.\n\n[MDN Reference](https://developer.mozilla.org/docs/Web/CSS/@media), [Can I Use](https://caniuse.com/?search=@media)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 6 },
        }),
    };

    assert_hover(css_text, expected_hover);
}

#[test]
fn test_hover_at_rule_on_at_token() {
    let css_text = "|@media screen and (min-width: 900px) {}";
    let expected_hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "Defines a stylesheet for a particular media type.\n\n[MDN Reference](https://developer.mozilla.org/docs/Web/CSS/@media), [Can I Use](https://caniuse.com/?search=@media)\n\n".to_string(),
        }),
        range: Some(Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 6 },
        }),
    };

    assert_hover(css_text, expected_hover);
}
