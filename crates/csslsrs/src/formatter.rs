use biome_css_parser::CssParse;

/// Formats the parsed CSS and returns the formatted code.
///
/// # Arguments
///
/// * `parse` - The `CssParse` result from parsing.
///
/// # Returns
///
/// * A `String` containing the original CSS code.
pub fn format_css(parse: &CssParse) -> String {
    parse.tree().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_css;

    #[test]
    fn test_format_css() {
        let code = "body{margin:0;}";
        let parse = parse_css(code);
        let formatted_code = format_css(&parse);
        // Pour cet exemple simple, nous comparons avec le code original
        // Dans un formateur réel, vous vous attendez à ce que formatted_code soit correctement formaté
        assert_eq!(
            formatted_code, code,
            "Formatted code should match original code"
        );
    }
}
