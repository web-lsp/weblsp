use biome_css_parser::{parse_css as biome_parse_css, CssParse, CssParserOptions};

/// Parses CSS code and returns the parse result.
pub fn parse_css(code: &str) -> CssParse {
    // Create parser options
    let options = CssParserOptions::default();

    // Parse the CSS code
    biome_parse_css(code, options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_css() {
        let code = "body { margin: 0; }";
        let parse = parse_css(code);
        assert!(
            !parse.has_errors(),
            "Parsing valid CSS should not have errors"
        );
    }

    #[test]
    fn test_parse_invalid_css() {
        let code = "body { margin }";
        let parse = parse_css(code);
        assert!(parse.has_errors(), "Parsing invalid CSS should have errors");
    }
}
