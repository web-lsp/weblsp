use crate::{converters::line_index::LineIndex, service::LanguageService};
use biome_rowan::TextSize;
use lsp_types::{FoldingRange, FoldingRangeKind, TextDocumentItem};

/// Compute the folding ranges for the given CSS source code. It supports CSS blocks enclosed in
/// braces, multi-line comments, and regions marked with `#region` and `#endregion` comments.
///
/// # Arguments
/// `document` - The original CSS source code as a `TextDocumentItem`.
/// `line_index` - The precomputed line index from StoreEntry.
///
/// # Returns
/// A vector of `FoldingRange` indicating the foldable regions in the CSS code.
fn compute_folding_ranges(
    document: &TextDocumentItem,
    line_index: &LineIndex,
) -> Vec<FoldingRange> {
    let mut folding_ranges = Vec::new();
    let mut brace_stack = Vec::new();
    let mut comment_stack = Vec::new();
    let mut region_stack = Vec::new();

    let source = &document.text;

    let line_starts = &line_index.newlines;

    let mut chars = source.char_indices().peekable();
    while let Some((offset, c)) = chars.next() {
        match c {
            '{' => {
                // Convert offset to TextSize and unwrap, assuming offset <= u32::MAX
                let text_size_offset = TextSize::try_from(offset).unwrap();
                let line_number = line_starts
                    .binary_search(&text_size_offset)
                    .unwrap_or_else(|x| x.saturating_sub(1))
                    as u32;
                brace_stack.push(line_number);
            }
            '}' => {
                if let Some(start_line) = brace_stack.pop() {
                    let text_size_offset = TextSize::try_from(offset).unwrap();
                    let end_line = line_starts
                        .binary_search(&text_size_offset)
                        .unwrap_or_else(|x| x.saturating_sub(1))
                        as u32;
                    if start_line != end_line {
                        folding_ranges.push(FoldingRange {
                            start_line,
                            start_character: None,
                            end_line,
                            end_character: None,
                            kind: None, // CSS blocks have no specific kind
                            collapsed_text: None,
                        });
                    }
                }
            }
            '/' => {
                if let Some(&(_, next_char)) = chars.peek() {
                    if next_char == '*' {
                        chars.next(); // Consume the '*'
                        let text_size_offset = TextSize::try_from(offset).unwrap();
                        let line_number = line_starts
                            .binary_search(&text_size_offset)
                            .unwrap_or_else(|x| x.saturating_sub(1))
                            as u32;
                        comment_stack.push(line_number);
                    }
                }
            }
            '*' => {
                if let Some(&(_, next_char)) = chars.peek() {
                    if next_char == '/' {
                        chars.next(); // Consume the '/'
                        if let Some(start_line) = comment_stack.pop() {
                            let text_size_offset = TextSize::try_from(offset).unwrap();
                            let end_line = line_starts
                                .binary_search(&text_size_offset)
                                .unwrap_or_else(|x| x.saturating_sub(1))
                                as u32;

                            let end_offset = if (end_line as usize) + 1 < line_starts.len() {
                                usize::from(line_starts[(end_line as usize) + 1])
                            } else {
                                source.len()
                            };

                            let comment_content =
                                &source[usize::from(line_starts[start_line as usize])..end_offset];

                            if comment_content.contains("#region") {
                                region_stack.push(start_line);
                            } else if comment_content.contains("#endregion") {
                                if let Some(region_start) = region_stack.pop() {
                                    folding_ranges.push(FoldingRange {
                                        start_line: region_start,
                                        start_character: None,
                                        end_line,
                                        end_character: None,
                                        kind: Some(FoldingRangeKind::Region),
                                        collapsed_text: None,
                                    });
                                }
                            } else if start_line != end_line {
                                folding_ranges.push(FoldingRange {
                                    start_line,
                                    start_character: None,
                                    end_line,
                                    end_character: None,
                                    kind: Some(FoldingRangeKind::Comment),
                                    collapsed_text: None,
                                });
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Determine the last line with content, so the final newline is always
    // clickable in the editor, even if the last folding range is collapsed
    let mut total_lines = line_starts.len() as u32 - 1;
    if source.ends_with('\n') && total_lines > 0 {
        total_lines -= 1;
    }

    // Handle any unclosed blocks
    while let Some(start_line) = brace_stack.pop() {
        if (start_line as usize) < total_lines as usize {
            folding_ranges.push(FoldingRange {
                start_line,
                start_character: None,
                end_line: total_lines,
                end_character: None,
                kind: None, // CSS blocks have no specific kind
                collapsed_text: None,
            });
        }
    }

    // Handle any unclosed comments
    while let Some(start_line) = comment_stack.pop() {
        if (start_line as usize) < total_lines as usize {
            folding_ranges.push(FoldingRange {
                start_line,
                start_character: None,
                end_line: total_lines,
                end_character: None,
                kind: Some(FoldingRangeKind::Comment),
                collapsed_text: None,
            });
        }
    }

    // Handle any unclosed regions
    while let Some(region_start) = region_stack.pop() {
        if region_start < total_lines {
            folding_ranges.push(FoldingRange {
                start_line: region_start,
                start_character: None,
                end_line: total_lines,
                end_character: None,
                kind: Some(FoldingRangeKind::Region),
                collapsed_text: None,
            });
        }
    }
    folding_ranges
}

impl LanguageService {
    /// Get the folding ranges for the given CSS source code. It supports CSS blocks enclosed in
    /// braces, multi-line comments, and regions marked with `#region` and `#endregion` comments.
    ///
    /// # Arguments
    /// `document` - The original CSS source code as a `TextDocumentItem`.
    ///
    /// # Returns
    /// A vector of `FoldingRange` indicating the foldable regions in the CSS code.
    pub fn get_folding_ranges(&mut self, document: TextDocumentItem) -> Vec<FoldingRange> {
        let store_document = self.store.get_or_update_document(document);
        compute_folding_ranges(&store_document.document, &store_document.line_index)
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use super::compute_folding_ranges;
    use crate::converters::line_index::LineIndex;
    use serde_wasm_bindgen;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"
/**
 * Get the folding ranges for the given CSS source code. It supports CSS blocks enclosed in
 * braces, multi-line comments, and regions marked with `#region` and `#endregion` comments.
 * 
 * @param source The CSS source code as a `TextDocument`.
 * @returns A list of `FoldingRange` objects indicating the foldable regions in the CSS code.
 */
export async function get_folding_ranges(source: import("vscode-languageserver-textdocument").TextDocument): Promise<import("vscode-languageserver-types").FoldingRange[]>;
"#;

    #[wasm_bindgen(skip_typescript)]
    pub fn get_folding_ranges(document: JsValue) -> JsValue {
        let parsed_text_document = crate::wasm_text_document::create_text_document(document);
        let folding_ranges = compute_folding_ranges(
            &parsed_text_document,
            &LineIndex::new(&parsed_text_document.text),
        );

        serde_wasm_bindgen::to_value(&folding_ranges).unwrap()
    }
}
