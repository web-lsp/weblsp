use biome_css_parser::CssParse;

/// Analyzes the parsed CSS and returns the number of errors.
///
/// # Arguments
///
/// * `parse` - The `CssParse` result from parsing.
///
/// # Returns
///
/// * The number of errors found in the CSS code.
pub fn analyze_css(parse: &CssParse) -> usize {
    parse.diagnostics().len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_css;

    #[test]
    fn test_analyze_css_no_errors() {
        let code = "body { margin: 0; }";
        let parse = parse_css(code);
        let error_count = analyze_css(&parse);
        assert_eq!(error_count, 0, "Should have no errors");
    }

    #[test]
    fn test_analyze_css_with_errors() {
        let code = "body { margin }";
        let parse = parse_css(code);
        let error_count = analyze_css(&parse);
        assert!(error_count > 0, "Should have errors");
    }
}
