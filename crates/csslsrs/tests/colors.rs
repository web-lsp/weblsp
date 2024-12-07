use csslsrs::service::LanguageService;
use lsp_types::{Color, ColorInformation, Position, Range, TextDocumentItem, Uri};
use std::str::FromStr;

#[test]
fn test_hex_color() {
    let mut ls = LanguageService::default();

    assert_color_symbols(
        &mut ls,
        "body { backgroundColor: #ff9977; }",
        vec![ColorInformation {
            color: Color {
                red: 1.0,
                green: 0.6,
                blue: 0.4666667,
                alpha: 1.0,
            },
            range: Range {
                start: Position {
                    line: 0,
                    character: 24,
                },
                end: Position {
                    line: 0,
                    character: 31,
                },
            },
        }],
    );
}

#[test]
fn test_hsl_color() {
    let mut ls = LanguageService::default();

    assert_color_symbols(
        &mut ls,
        "body { backgroundColor: hsl(0, 0%, 100%); }",
        vec![ColorInformation {
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            },
            range: Range {
                start: Position {
                    line: 0,
                    character: 24,
                },
                end: Position {
                    line: 0,
                    character: 40,
                },
            },
        }],
    );
}

#[test]
fn test_rgb_and_hsl_colors() {
    let mut ls = LanguageService::default();

    assert_color_symbols(
        &mut ls,
        ".oo { color: rgb(1,40,1); borderColor: hsl(120, 75%, 85%) }",
        vec![
            ColorInformation {
                color: Color {
                    red: 0.003921569,
                    green: 0.15686275,
                    blue: 0.003921569,
                    alpha: 1.0,
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 13,
                    },
                    end: Position {
                        line: 0,
                        character: 24,
                    },
                },
            },
            ColorInformation {
                color: Color {
                    red: 0.7375001,
                    green: 0.96250004,
                    blue: 0.7375001,
                    alpha: 1.0,
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 39,
                    },
                    end: Position {
                        line: 0,
                        character: 58,
                    },
                },
            },
        ],
    );
}

#[test]
fn test_rgba_color() {
    let mut ls = LanguageService::default();

    assert_color_symbols(
        &mut ls,
        "body { backgroundColor: rgba(1, 40, 1, 0.3); }",
        vec![ColorInformation {
            color: Color {
                red: 0.003921569,
                green: 0.15686275,
                blue: 0.003921569,
                alpha: 0.3,
            },
            range: Range {
                start: Position {
                    line: 0,
                    character: 24,
                },
                end: Position {
                    line: 0,
                    character: 43,
                },
            },
        }],
    );
}

#[test]
fn test_hwb_color() {
    let mut ls = LanguageService::default();

    assert_color_symbols(
        &mut ls,
        "body { backgroundColor: hwb(194 0% 0% / .5); }",
        vec![ColorInformation {
            color: Color {
                red: 0.0,
                green: 0.76666665,
                blue: 1.0,
                alpha: 0.5,
            },
            range: Range {
                start: Position {
                    line: 0,
                    character: 24,
                },
                end: Position {
                    line: 0,
                    character: 43,
                },
            },
        }],
    );
}

#[test]
fn test_named_color() {
    let mut ls = LanguageService::default();

    assert_color_symbols(
        &mut ls,
        "body { backgroundColor: red; }",
        vec![ColorInformation {
            color: Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            },
            range: Range {
                start: Position {
                    line: 0,
                    character: 24,
                },
                end: Position {
                    line: 0,
                    character: 27,
                },
            },
        }],
    );
}

#[test]
fn test_functions_color() {
    let mut ls = LanguageService::default();

    assert_color_symbols(
        &mut ls,
        "body { color: linear-gradient(to right, red, blue); }",
        vec![
            ColorInformation {
                color: Color {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 40,
                    },
                    end: Position {
                        line: 0,
                        character: 43,
                    },
                },
            },
            ColorInformation {
                color: Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
                    alpha: 1.0,
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 45,
                    },
                    end: Position {
                        line: 0,
                        character: 49,
                    },
                },
            },
        ],
    );
}

#[test]
fn test_color_presentations() {
    let mut ls = LanguageService::default();

    assert_color_presentations(
        &mut ls,
        ColorInformation {
            color: Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
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
        },
        vec![
            "red",
            "rgb(255 0 0)",
            "#ff0000",
            "hsl(0 100% 50%)",
            "hwb(0 0% 0%)",
            "lab(53.24% 80.09 67.20)",
            "lch(53.24% 104.55 40.00)",
        ],
    );

    assert_color_presentations(
        &mut ls,
        ColorInformation {
            color: Color {
                red: 0.3019608,
                green: 0.12941177,
                blue: 0.43529412,
                alpha: 0.5019608,
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
        },
        vec![
            "rgb(77 33 111 / 50%)",
            "#4d216f80",
            "hsl(274 54% 28% / 50%)",
            "hwb(274 13% 56% / 50%)",
            "lab(23.04% 35.89 -36.96 / 50%)",
            "lch(23.04% 51.52 314.16 / 50%)",
        ],
    );
}

fn assert_color_presentations(
    ls: &mut LanguageService,
    color: ColorInformation,
    expected_presentations_texts: Vec<&str>,
) {
    let range = color.range;
    let presentations = ls.get_color_presentations(color, range);

    assert_eq!(
        presentations.len(),
        expected_presentations_texts.len(),
        "Unexpected number of color presentations"
    );

    for (presentation, expected_text) in presentations
        .iter()
        .zip(expected_presentations_texts.iter())
    {
        assert_eq!(
            presentation.label, *expected_text,
            "Unexpected color presentation text"
        );
        assert_eq!(
            presentation.text_edit.as_ref().unwrap().new_text,
            *expected_text,
            "Unexpected color presentation text edit"
        );
        assert!(presentation.text_edit.as_ref().unwrap().range == range);
    }
}

fn assert_color_symbols(
    ls: &mut LanguageService,
    text: &str,
    expected_colors: Vec<ColorInformation>,
) {
    let document = TextDocumentItem {
        uri: Uri::from_str("file:///test.css").unwrap(),
        language_id: "css".to_string(),
        version: 1,
        text: text.to_string(),
    };

    ls.store.upsert_document(document.clone());

    let colors = ls.get_document_colors(document);

    assert_eq!(
        colors.len(),
        expected_colors.len(),
        "Unexpected number of colors"
    );

    for (color, expected) in colors.iter().zip(expected_colors.iter()) {
        assert_eq!(color, expected, "Unexpected color information");
    }
}
