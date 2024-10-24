/// VSCode-like object that represents a text document.
pub struct TextDocument {
    pub uri: String,
    pub language_id: String,
    pub version: i64,
    pub text: String,
}

impl TextDocument {
    /// Creates a new `TextDocument` object.
    ///
    /// # Arguments
    ///
    /// * `uri` - A string slice that holds the URI of the text document.
    /// * `language_id` - A string slice that holds the language ID of the text document.
    /// * `version` - An integer that holds the version of the text document.
    /// * `text` - A string slice that holds the text of the document.
    ///
    /// # Returns
    ///
    /// * A `TextDocument` object.
    pub fn new(uri: &str, language_id: &str, version: i64, text: &str) -> TextDocument {
        TextDocument {
            uri: uri.to_string(),
            language_id: language_id.to_string(),
            version,
            text: text.to_string(),
        }
    }
}
