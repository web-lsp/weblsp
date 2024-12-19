use crate::css_data::{CssCustomData, MarkupDescriptionOrString, Reference, Status};
use biome_css_syntax::{CssLanguage, CssSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode};
use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, TextDocumentItem};
use std::fmt::Write;

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
        // Handle CSS properties like "color", "font-size", etc.
        CssSyntaxKind::CSS_IDENTIFIER => {
            for data in css_data {
                if let Some(property) = data
                    .properties
                    .as_ref()
                    .iter()
                    .flat_map(|props| props.iter())
                    .find(|prop| prop.name == name)
                {
                    return Some(format_css_property_entry(
                        &property.name,
                        &property.status,
                        &property.description,
                        property.syntax.as_deref(),
                        property.browsers.as_deref(),
                        property.references.as_deref(),
                    ));
                }
            }
            None
        }
        // Handle at-rules like @media, @supports, etc.
        CssSyntaxKind::CSS_AT_RULE => {
            for data in css_data {
                if let Some(at_directive) = data
                    .at_directives
                    .as_ref()
                    .iter()
                    .flat_map(|ats| ats.iter())
                    .find(|at| at.name == name)
                {
                    return Some(format_css_at_rule_entry(
                        &at_directive.name,
                        &at_directive.status,
                        &at_directive.description,
                        at_directive.references.as_deref(),
                    ));
                }
            }
            None
        }
        // Handle CSS selectors like ".class", "#id", "element", etc.
        CssSyntaxKind::CSS_SELECTOR_LIST
        | CssSyntaxKind::CSS_COMPLEX_SELECTOR
        | CssSyntaxKind::CSS_COMPOUND_SELECTOR => Some(format_css_selector_entry(
            name,
            Some(calculate_specificity(name)),
        )),
        _ => None,
    }
}

/// Formats the CSS property entry into a hover content string.
fn format_css_property_entry(
    name: &str,
    status: &Option<Status>,
    description: &Option<MarkupDescriptionOrString>,
    syntax: Option<&str>,
    browsers: Option<&[String]>,
    references: Option<&[Reference]>,
) -> String {
    let mut content = String::new();
    write_status(&mut content, status);
    write_description(&mut content, description);
    write_browser_support(&mut content, browsers);
    write_syntax(&mut content, syntax);
    write_references(&mut content, references, name);
    content
}

/// Formats the CSS at-rule entry into a hover content string.
fn format_css_at_rule_entry(
    name: &str,
    status: &Option<Status>,
    description: &Option<MarkupDescriptionOrString>,
    references: Option<&[Reference]>,
) -> String {
    let mut content = String::new();
    write_status(&mut content, status);
    write_description(&mut content, description);
    write_references(&mut content, references, name);
    content
}

/// Formats the CSS selector entry into a hover content string.
fn format_css_selector_entry(name: &str, specificity: Option<(u32, u32, u32)>) -> String {
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
    match status {
        Some(Status::Experimental) => {
            writeln!(content, "🧪 *Experimental, use with caution.*\n").unwrap()
        }
        Some(Status::Obsolete) => {
            writeln!(content, "🚧 *Obsolete, consider using alternatives.*\n").unwrap()
        }
        Some(Status::Nonstandard) => {
            writeln!(content, "🚨 *Non-standard, avoid using it.*\n").unwrap()
        }
        _ => {}
    }
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
fn write_syntax(content: &mut String, syntax: Option<&str>) {
    if let Some(syntax) = syntax {
        writeln!(content, "Syntax: `{}`\n", syntax).unwrap();
    }
}

/// Writes the references of a CSS Custom Data to the hover content.
fn write_references(
    content: &mut String,
    references: Option<&[Reference]>,
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
fn write_browser_support(content: &mut String, browsers: Option<&[String]>) {
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
