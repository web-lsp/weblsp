use crate::css_data::CssCustomData;
use biome_css_syntax::{CssLanguage, CssSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode};
use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, TextDocumentItem};

use crate::{
    converters::{from_proto::offset, line_index::LineIndex, to_proto::range, PositionEncoding},
    service::LanguageService,
};

/// Extracts hover information for the given CSS node and position.
fn extract_hover_information(
    node: &SyntaxNode<CssLanguage>,
    position: Position,
    line_index: &LineIndex,
    encoding: PositionEncoding,
    css_data: &Vec<&CssCustomData>,
) -> Option<Hover> {
    let offset = offset(line_index, position, encoding).ok()?;
    let token = node.token_at_offset(offset).right_biased()?;
    let mut selector_node = None;
    for ancestor in token.ancestors() {
        match ancestor.kind() {
            // These nodes represent the full selector, including combinators
            CssSyntaxKind::CSS_COMPLEX_SELECTOR | CssSyntaxKind::CSS_SELECTOR_LIST => {
                selector_node = Some(ancestor.clone());
                break; // We've found the full selector
            }
            // Update selector_node if it's not already set
            CssSyntaxKind::CSS_COMPOUND_SELECTOR => {
                if selector_node.is_none() {
                    selector_node = Some(ancestor.clone());
                }
            }
            CssSyntaxKind::CSS_IDENTIFIER => {
                // Handle identifiers like properties or at-rules
                if let Some(hover_content) =
                    get_css_hover_content(ancestor.kind(), token.text().trim(), css_data)
                {
                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: hover_content,
                        }),
                        range: range(line_index, ancestor.text_trimmed_range(), encoding).ok(),
                    });
                }
            }
            _ => {
                // Not part of a selector; continue traversing
            }
        }
    }

    // Use the identified selector node for hover content
    if let Some(selector_node) = selector_node {
        if let Some(hover_content) = get_css_hover_content(
            selector_node.kind(),
            selector_node.text().to_string().trim(),
            css_data,
        ) {
            return Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: hover_content,
                }),
                range: range(line_index, selector_node.text_trimmed_range(), encoding).ok(),
            });
        }
    }

    None
}

/// Generates hover content for a given CSS entity using the provided CSS custom data.
fn get_css_hover_content(
    kind: CssSyntaxKind,
    name: &str,
    css_data: &Vec<&CssCustomData>,
) -> Option<String> {
    match kind {
        CssSyntaxKind::CSS_IDENTIFIER => {
            for data in css_data {
                if let Some(property) = data
                    .css
                    .properties
                    .entry
                    .iter()
                    .find(|prop| prop.attributes.name == name)
                {
                    return Some(format_css_entry(
                        &property.attributes.name,
                        property.desc.as_deref(),
                        property.attributes.syntax.as_deref(),
                        None,
                        property.attributes.browsers.as_deref(),
                        property.attributes.ref_.as_deref(),
                        property.attributes.restriction.as_deref(),
                    ));
                }
            }
            None
        }
        // Handle at-rules like @media, @supports, etc.
        CssSyntaxKind::CSS_AT_RULE => {
            for data in css_data {
                if let Some(at_directive) = data
                    .css
                    .at_directives
                    .entry
                    .iter()
                    .find(|ad| ad.attributes.name == name)
                {
                    return Some(format_css_entry(
                        &at_directive.attributes.name,
                        at_directive.desc.as_deref(),
                        at_directive.attributes.syntax.as_deref(),
                        None,
                        at_directive.attributes.browsers.as_deref(),
                        at_directive.attributes.ref_.as_deref(),
                        None,
                    ));
                }
            }
            None
        }
        CssSyntaxKind::CSS_SELECTOR_LIST
        | CssSyntaxKind::CSS_COMPLEX_SELECTOR
        | CssSyntaxKind::CSS_COMPOUND_SELECTOR => Some(format_css_entry(
            name,
            None,
            None,
            Some(calculate_specificity(name)),
            None,
            None,
            None,
        )),
        _ => None,
    }
}

/// Formats the CSS entry into a hover content string.
fn format_css_entry(
    name: &str,
    desc: Option<&str>,
    syntax: Option<&str>,
    specificity: Option<(u32, u32, u32)>,
    browsers: Option<&str>,
    reference: Option<&str>,
    restriction: Option<&str>,
) -> String {
    let mut content = String::new();
    content.push_str(&format!("**{}**\n\n", escape_markdown(name)));

    // Add the description if available
    if let Some(description) = desc {
        content.push_str(description);
        content.push_str("\n\n");
    }

    // Add syntax if available
    if let Some(syntax) = syntax {
        content.push_str(&format!("**Syntax**: `{}`\n\n", syntax));
    }

    // Add specificity if available
    if let Some((ids, classes, elements)) = specificity {
        content.push_str(&format!(
            "[Selector Specificity](https://developer.mozilla.org/docs/Web/CSS/Specificity): ({}, {}, {})\n\n",
            ids, classes, elements
        ));
    }

    // Add restriction if available
    if let Some(info) = restriction {
        content.push_str(&format!("**Restriction**: {}\n\n", info));
    }

    // Add browsers if available
    if let Some(browsers) = browsers {
        content.push_str("**Supported Browsers**:\n");
        for browser in browsers.split(',') {
            content.push_str(&format!("- {}\n", browser.trim()));
        }
        content.push('\n');
    }

    // Add reference if available
    if let Some(reference) = reference {
        content.push_str("**Reference**:\n");
        content.push_str(&format!("- [{}]({})\n\n", name, reference));
    }

    content
}

fn escape_markdown(text: &str) -> String {
    text.replace('*', "\\*")
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

impl LanguageService<'_> {
    /// Gets the hover information for the given CSS document and position.
    pub fn get_hover(&mut self, document: TextDocumentItem, position: Position) -> Option<Hover> {
        let store_entry = self.store.get_or_update_document(document);

        extract_hover_information(
            store_entry.css_tree.tree().syntax(),
            position,
            &store_entry.line_index,
            self.encoding,
            &self.css_data,
        )
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use super::extract_hover_information;
    use crate::{
        converters::{line_index::LineIndex, PositionEncoding, WideEncoding},
        css_data_generated::CSS_DATA,
        parser::parse_css,
        wasm_text_document::create_text_document,
    };
    use biome_rowan::AstNode;
    use lsp_types::Position;
    use serde_wasm_bindgen;
    use wasm_bindgen::prelude::*;
    extern crate console_error_panic_hook;

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"export async function get_hover(document: import("vscode-languageserver-textdocument").TextDocument, position: import("vscode-languageserver-types").Position): Promise<import("vscode-languageserver-types").Hover | null>;"#;

    #[wasm_bindgen(skip_typescript)]
    pub fn get_hover(document: JsValue, position: JsValue) -> JsValue {
        let parsed_text_document = create_text_document(document);
        let position: Position = serde_wasm_bindgen::from_value(position).unwrap();
        let css_parse = parse_css(&parsed_text_document.text);
        let line_index = LineIndex::new(&parsed_text_document.text);
        let encoding = PositionEncoding::Wide(WideEncoding::Utf16);

        let hover = extract_hover_information(
            css_parse.tree().syntax(),
            position,
            &line_index,
            encoding,
            &vec![&CSS_DATA],
        );

        serde_wasm_bindgen::to_value(&hover).unwrap()
    }
}
