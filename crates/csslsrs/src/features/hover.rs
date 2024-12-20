use crate::{
    converters::{
        from_proto::offset,
        line_index::LineIndex,
        to_proto::{self, range},
        PositionEncoding,
    },
    css_data::{
        AtDirectiveEntry, CssCustomData, MarkupDescriptionOrString, PropertyEntry, Reference,
        Status,
    },
    service::LanguageService,
};
use biome_css_syntax::{CssLanguage, CssSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode, TextSize};
use lsp_types::{
    Hover, HoverContents, MarkupContent, MarkupKind, Position, Range, TextDocumentItem,
};
use std::fmt::Write;

/// Extracts hover information for the given CSS node and position.
fn extract_hover_information(
    node: &SyntaxNode<CssLanguage>,
    position: Position,
    line_index: &LineIndex,
    encoding: PositionEncoding,
    css_data: &[&CssCustomData],
) -> Option<Hover> {
    let offset = offset(line_index, position, encoding).ok()?;
    let token = node.token_at_offset(offset).right_biased()?;
    // Since the token is a leaf node, we need to find a more meaningful parent to provide hover informations.
    token
        .ancestors()
        .find_map(|ancestor| match ancestor.kind() {
            // Handle CSS selectors, e.g. `.class`, `#id`, `element`, `element.class`, etc.
            CssSyntaxKind::CSS_COMPLEX_SELECTOR | CssSyntaxKind::CSS_SELECTOR_LIST => {
                let name = &ancestor.text_trimmed().to_string();
                let content = format_selector_entry(name, Some(calculate_specificity(name)));
                Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: content,
                    }),
                    range: range(line_index, ancestor.text_trimmed_range(), encoding).ok(),
                })
            }
            // Handle CSS properties, e.g. `color`, `font-size`, etc.
            CssSyntaxKind::CSS_GENERIC_PROPERTY => {
                // We can assume that the token is the IDENT token with the property name.
                let name = token.text_trimmed().to_string();
                css_data.iter().find_map(|data| {
                    data.properties
                        .as_ref()?
                        .iter()
                        .find(|prop| prop.name == name)
                        .map(format_property_entry)
                        .map(|content| Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: content,
                            }),
                            range: range(line_index, token.text_trimmed_range(), encoding).ok(),
                        })
                })
            }
            // Handle CSS at-rules, e.g. `@media`, `@keyframes`, etc.
            CssSyntaxKind::CSS_AT_RULE => {
                // We can't assume that the token is the KW token (with the at-rule name) since it could be the AT token.
                let at_rule_token = ancestor.first_child()?.first_token()?;
                css_data.iter().find_map(|data| {
                    data.at_directives
                        .as_ref()?
                        .iter()
                        .find(|at_rule| {
                            // CSS Custom Data uses `@` prefix for at-rules, so we need to add it back.
                            at_rule.name == format!("@{}", at_rule_token.text_trimmed())
                        })
                        .map(format_at_rule_entry)
                        .map(|content| Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: content,
                            }),
                            range: Some(Range::new(
                                to_proto::position(
                                    line_index,
                                    // We need to include the `@` symbol in the selection range.
                                    at_rule_token.text_trimmed_range().start() - TextSize::from(1),
                                    encoding,
                                )
                                .unwrap(),
                                to_proto::position(
                                    line_index,
                                    at_rule_token.text_trimmed_range().end(),
                                    encoding,
                                )
                                .unwrap(),
                            )),
                        })
                })
            }
            _ => None,
        })
}

/// Formats the CSS property entry into a hover content string.
fn format_property_entry(property: &PropertyEntry) -> String {
    let mut content = String::new();
    write_status(&mut content, &property.status);
    write_description(&mut content, &property.description);
    write_browser_support(&mut content, &property.browsers);
    write_syntax(&mut content, &property.syntax);
    write_references(&mut content, &property.references, &property.name);
    content
}

/// Formats the CSS at-rule entry into a hover content string.
fn format_at_rule_entry(at_property: &AtDirectiveEntry) -> String {
    let mut content = String::new();
    write_status(&mut content, &at_property.status);
    write_description(&mut content, &at_property.description);
    write_references(&mut content, &at_property.references, &at_property.name);
    content
}

/// Formats the CSS selector entry into a hover content string.
fn format_selector_entry(name: &str, specificity: Option<(u32, u32, u32)>) -> String {
    let mut content = String::new();
    // TODO: this is a placeholder, we should render an HTML preview of the selector
    writeln!(content, "**{}**\n", escape_markdown(name)).unwrap();
    // Add specificity if available
    if let Some((ids, classes, elements)) = specificity {
        writeln!(content,
            "[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): ({}, {}, {})\n",
            ids, classes, elements
        ).unwrap();
    }
    content
}

/// Escapes Markdown characters in the selector name.
fn escape_markdown(text: &str) -> String {
    text.replace('*', "\\*")
        .replace('_', "\\_")
        .replace('`', "\\`")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

/// Parses the status of a CSS Custom Data to a human-readable string, and writes it to the hover content.
fn write_status(content: &mut String, status: &Option<Status>) {
    let status_str = match status {
        Some(Status::Experimental) => "🧪 *Experimental, use with caution.*\n",
        Some(Status::Obsolete) => "🚧 *Obsolete, consider using alternatives.*\n",
        Some(Status::Nonstandard) => "🚨 *Non-standard, avoid using it.*\n",
        _ => return,
    };
    writeln!(content, "{}", status_str).unwrap();
}

/// Writes the description of a CSS Custom Data to the hover content.
fn write_description(content: &mut String, description: &Option<MarkupDescriptionOrString>) {
    if let Some(desc) = description {
        let desc_str = match desc {
            MarkupDescriptionOrString::MarkupDescription(md) => &md.value,
            MarkupDescriptionOrString::String(s) => s,
        };
        writeln!(content, "{}\n", desc_str).unwrap();
    }
}

/// Writes the syntax of a CSS Custom Data to the hover content.
fn write_syntax(content: &mut String, syntax: &Option<String>) {
    if let Some(syntax) = syntax {
        writeln!(content, "Syntax: `{}`\n", syntax).unwrap();
    }
}

/// Writes the references of a CSS Custom Data to the hover content.
fn write_references(
    content: &mut String,
    references: &Option<Vec<Reference>>,
    name_for_caniuse: &str,
) {
    if let Some(references) = references {
        let reference_str = references
            .iter()
            .map(|reference| format!("[{}]({})", reference.name, reference.url))
            .collect::<Vec<_>>()
            .join(", ");
        // Add Can I Use as a bonus 🎁
        let can_i_use_url = format!("https://caniuse.com/?search={}", name_for_caniuse);
        writeln!(
            content,
            "{}, [Can I Use]({})\n",
            reference_str, can_i_use_url
        )
        .unwrap();
    }
}

/// Writes the browser support information of a CSS Custom Data to the hover content.
fn write_browser_support(content: &mut String, browsers: &Option<Vec<String>>) {
    if let Some(browsers) = browsers {
        let browsers_str = browsers
            .iter()
            .filter_map(|s| parse_browser_name(s.trim()))
            .collect::<Vec<_>>()
            .join(", ");
        if !browsers_str.is_empty() {
            writeln!(content, "Supported by {}.\n", browsers_str).unwrap();
        }
    }
}

/// Parses browser code (letter + version) to a human-readable string.
///
/// Example: "FF78" -> "Firefox 78"
fn parse_browser_name(name: &str) -> Option<String> {
    let (browser_code, version) = name.split_at(name.find(|c: char| c.is_ascii_digit())?);
    let browser = match browser_code {
        "E" => "Edge",
        "FF" => "Firefox",
        "S" => "Safari",
        "C" => "Chrome",
        "IE" => "Internet Explorer",
        "O" => "Opera",
        _ => return None,
    };
    Some(format!("{} {}", browser, version))
}

/// Given a CSS selector, calculates the specificity of the selector.
/// The specificity is returned as a tuple of (ids, classes, elements).
/// Refer to the MDN documentation for more information:
/// https://developer.mozilla.org/docs/Web/CSS/Specificity
fn calculate_specificity(selector: &str) -> (u32, u32, u32) {
    let (mut ids, mut classes, mut elements) = (0, 0, 0);

    // Segregate the selector into tokens
    let tokens = selector.split([' ', '>', '+', '~', ',']);

    for token in tokens {
        let mut chars = token.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '#' => {
                    ids += 1;
                    consume_identifier(&mut chars);
                }

                '.' => {
                    classes += 1;
                    consume_identifier(&mut chars);
                }

                '[' => {
                    classes += 1;
                    consume_until(&mut chars, ']');
                }

                ':' => {
                    if let Some(&next_c) = chars.peek() {
                        if next_c == ':' {
                            chars.next(); // Consume the second colon
                            elements += 1;
                        } else {
                            classes += 1;
                        }
                        consume_identifier(&mut chars);
                    }
                }

                c if is_identifier_start(c) => {
                    elements += 1;
                    consume_identifier(&mut chars);
                }

                '*' => {
                    // Universal selector does not contribute to specificity
                }

                _ => {
                    // Consume any other characters (e.g., combinators)
                }
            }
        }
    }

    (ids, classes, elements)
}

fn consume_identifier<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) {
    while let Some(&c) = chars.peek() {
        if c == '\\' {
            chars.next(); // Consume the backslash
            consume_escape(chars);
        } else if is_identifier_char(c) {
            chars.next(); // Consume the character
        } else {
            break;
        }
    }
}

fn consume_escape<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) {
    if let Some(&c) = chars.peek() {
        if c.is_ascii_hexdigit() {
            // Consume up to 6 hex digits
            for _ in 0..6 {
                if let Some(&hex_c) = chars.peek() {
                    if hex_c.is_ascii_hexdigit() {
                        chars.next(); // Consume hex digit
                    } else {
                        break;
                    }
                }
            }
            // Optional whitespace after hex digits
            if let Some(&ws_c) = chars.peek() {
                if ws_c.is_whitespace() {
                    chars.next(); // Consume whitespace
                }
            }
        } else if c == '\n' || c == '\r' || c == '\u{000C}' {
            // Invalid escape; do not consume
        } else {
            chars.next(); // Consume the escaped character
        }
    }
}

fn consume_until<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>, end_char: char) {
    while let Some(c) = chars.next() {
        if c == end_char {
            break;
        } else if c == '\\' {
            // Handle escapes inside attribute selectors
            consume_escape(chars);
        }
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '-' || c == '\\'
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-' || c == '\\'
}

impl LanguageService {
    /// Gets the hover information for the given CSS document and position.
    pub fn get_hover(&self, document: TextDocumentItem, position: Position) -> Option<Hover> {
        let store_entry = self.store.get(&document.uri);

        match store_entry {
            Some(store_entry) => Some(extract_hover_information(
                store_entry.css_tree.tree().syntax(),
                position,
                &store_entry.line_index,
                self.options.encoding,
                &self.get_css_custom_data(),
            ))?,
            None => None,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use std::str::FromStr;

    use super::extract_hover_information;
    use crate::service::wasm_bindings::WASMLanguageService;
    use biome_rowan::AstNode;
    use lsp_types::{Position, Uri};
    use serde_wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"
        declare function get_hover(documentUri: string, position:  import("vscode-languageserver-types").Position): import("vscode-languageserver-types").FoldingRange[];
    "#;

    #[wasm_bindgen]
    impl WASMLanguageService {
        #[wasm_bindgen(skip_typescript, js_name = getHover)]
        pub fn get_hover(&self, document_uri: String, position: JsValue) -> JsValue {
            let store_document = self.store.get(&Uri::from_str(&document_uri).unwrap());

            let hover_info = match store_document {
                Some(store_document) => {
                    let position: Position = serde_wasm_bindgen::from_value(position).unwrap();

                    extract_hover_information(
                        store_document.css_tree.tree().syntax(),
                        position,
                        &store_document.line_index,
                        self.options.encoding,
                        &self.get_css_custom_data(),
                    )
                }
                None => None,
            };

            serde_wasm_bindgen::to_value(&hover_info).unwrap()
        }
    }
}
