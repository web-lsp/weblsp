use lsp_types::{FoldingRange, FoldingRangeKind, TextDocumentItem};

#[cfg(feature = "wasm")]
use serde_wasm_bindgen;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Computes the folding ranges for the given CSS source code.
///
/// # Arguments
///
/// * `source` - The original CSS source code as a string slice.
///
/// # Returns
///
/// * A vector of `FoldingRange` indicating the foldable regions in the CSS code.
#[cfg(not(feature = "wasm"))]
pub fn get_folding_ranges(document: TextDocumentItem) -> Vec<FoldingRange> {
    compute_folding_ranges(document)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export async function get_folding_ranges(source: import("vscode-languageserver-textdocument").TextDocument): Promise<import("vscode-languageserver-types").FoldingRange[]>;
"#;

#[cfg(feature = "wasm")]
#[wasm_bindgen(skip_typescript)]
pub fn get_folding_ranges(source: JsValue) -> JsValue {
    let doc = crate::text_document::wasm_bindings::create_text_document(source);
    let folding_ranges = compute_folding_ranges(doc);

    serde_wasm_bindgen::to_value(&folding_ranges).unwrap()
}

fn compute_folding_ranges(document: TextDocumentItem) -> Vec<FoldingRange> {
    let mut folding_ranges = Vec::new();
    let mut stack = Vec::new();

    let source = document.text;

    // Precompute line start offsets
    let line_starts: Vec<usize> = std::iter::once(0)
        .chain(source.match_indices('\n').map(|(idx, _)| idx + 1))
        .collect();

    for (offset, c) in source.char_indices() {
        if c == '{' {
            // Determine line number based on offset
            let line_number = line_starts.partition_point(|&line_start| line_start <= offset) - 1;
            stack.push((offset, line_number));
        } else if c == '}' {
            if let Some((_start_offset, start_line)) = stack.pop() {
                let end_line = line_starts.partition_point(|&line_start| line_start <= offset) - 1;
                if start_line != end_line {
                    folding_ranges.push(FoldingRange {
                        start_line: start_line as u32,
                        start_character: None,
                        end_line: end_line as u32,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Region),
                        collapsed_text: None,
                    });
                }
            }
        }
    }

    folding_ranges
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use lsp_types::Uri;

    use super::*;

    #[test]
    fn test_compute_folding_ranges_empty() {
        let document = TextDocumentItem::new(
            Uri::from_str("file:///test.css").unwrap(),
            "css".to_string(),
            1,
            "".to_string(),
        );

        let folding_ranges = compute_folding_ranges(document);

        assert!(
            folding_ranges.is_empty(),
            "No folding ranges should be returned for empty input"
        );
    }

    #[test]
    fn test_compute_folding_ranges_single_rule() {
        let document = TextDocumentItem::new(
            Uri::from_str("file:///test.css").unwrap(),
            "css".to_string(),
            1,
            "body {\n    margin: 0;\n    padding: 0;\n}\n".to_string(),
        );

        let folding_ranges = compute_folding_ranges(document);

        assert_eq!(folding_ranges.len(), 1, "Expected one folding range");
        let range = &folding_ranges[0];
        assert_eq!(range.start_line, 0, "Folding should start at line 0");
        assert_eq!(range.end_line, 3, "Folding should end at line 3");
    }

    #[test]
    fn test_compute_folding_ranges_multiple_rules() {
        let document = TextDocumentItem::new(
            Uri::from_str("file:///test.css").unwrap(),
            "css".to_string(),
            1,
            "body {\n    margin: 0;\n}\n\nh1 {\n    color: red;\n}\n".to_string(),
        );
        let mut folding_ranges = compute_folding_ranges(document);

        assert_eq!(folding_ranges.len(), 2, "Expected two folding ranges");

        folding_ranges.sort_by_key(|fr| fr.start_line);

        let range1 = &folding_ranges[0];
        assert_eq!(range1.start_line, 0, "First folding should start at line 0");
        assert_eq!(range1.end_line, 2, "First folding should end at line 2");

        let range2 = &folding_ranges[1];
        assert_eq!(
            range2.start_line, 4,
            "Second folding should start at line 4"
        );
        assert_eq!(range2.end_line, 6, "Second folding should end at line 6");
    }

    #[test]
    fn test_compute_folding_ranges_nested_rules() {
        let document = TextDocumentItem::new(
            Uri::from_str("file:///test.css").unwrap(),
            "css".to_string(),
            1,
            "@media screen {\n    body {\n        margin: 0;\n    }\n}\n".to_string(),
        );
        let mut folding_ranges = compute_folding_ranges(document);

        assert_eq!(folding_ranges.len(), 2, "Expected two folding ranges");

        // Sort folding ranges by start_line
        folding_ranges.sort_by_key(|fr| fr.start_line);

        let outer_range = &folding_ranges[0];
        assert_eq!(
            outer_range.start_line, 0,
            "Outer folding should start at line 0"
        );
        assert_eq!(
            outer_range.end_line, 4,
            "Outer folding should end at line 4"
        );

        let inner_range = &folding_ranges[1];
        assert_eq!(
            inner_range.start_line, 1,
            "Inner folding should start at line 1"
        );
        assert_eq!(
            inner_range.end_line, 3,
            "Inner folding should end at line 3"
        );
    }

    #[test]
    fn test_compute_folding_ranges_single_line_rule() {
        let document = TextDocumentItem::new(
            Uri::from_str("file:///test.css").unwrap(),
            "css".to_string(),
            1,
            "h1 { color: blue; }\n".to_string(),
        );
        let folding_ranges = compute_folding_ranges(document);

        // Since the rule is on a single line, there should be no folding range
        assert!(
            folding_ranges.is_empty(),
            "No folding ranges expected for single-line rules"
        );
    }

    #[test]
    fn test_compute_folding_ranges_unmatched_braces() {
        let document = TextDocumentItem::new(
            Uri::from_str("file:///test.css").unwrap(),
            "css".to_string(),
            1,
            "body {\n    margin: 0;\n    padding: 0;\n\n".to_string(),
        );
        let folding_ranges = compute_folding_ranges(document);

        // The opening brace does not have a matching closing brace
        // So the folding range should not be added
        assert!(
            folding_ranges.is_empty(),
            "No folding ranges expected when braces are unmatched"
        );
    }

    #[test]
    fn test_compute_folding_ranges_with_comments() {
        let document = TextDocumentItem::new(
            Uri::from_str("file:///test.css").unwrap(),
            "css".to_string(),
            1,
            "/* Comment block\nspanning multiple lines\n*/\nbody {\n    margin: 0;\n}\n"
                .to_string(),
        );
        let folding_ranges = compute_folding_ranges(document);

        assert_eq!(folding_ranges.len(), 1, "Expected one folding range");

        let range = &folding_ranges[0];
        assert_eq!(range.start_line, 3, "Folding should start at line 3");
        assert_eq!(range.end_line, 5, "Folding should end at line 5");
    }

    #[test]
    fn test_compute_folding_ranges_complex() {
        let document = TextDocumentItem::new(
            Uri::from_str("file:///test.css").unwrap(),
            "css".to_string(),
            1,
            "@media screen {\n    @supports (display: grid) {\n        .container {\n            display: grid;\n        }\n    }\n}\n".to_string(),
        );
        let mut folding_ranges = compute_folding_ranges(document);

        assert_eq!(folding_ranges.len(), 3, "Expected three folding ranges");

        // Sort folding ranges by start_line
        folding_ranges.sort_by_key(|fr| fr.start_line);

        let range1 = &folding_ranges[0];
        assert_eq!(range1.start_line, 0, "First folding should start at line 0");
        assert_eq!(range1.end_line, 6, "First folding should end at line 6");

        let range2 = &folding_ranges[1];
        assert_eq!(
            range2.start_line, 1,
            "Second folding should start at line 1"
        );
        assert_eq!(range2.end_line, 5, "Second folding should end at line 5");

        let range3 = &folding_ranges[2];
        assert_eq!(range3.start_line, 2, "Third folding should start at line 2");
        assert_eq!(range3.end_line, 4, "Third folding should end at line 4");
    }
}
