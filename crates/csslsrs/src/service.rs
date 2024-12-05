use crate::{
    converters::PositionEncoding, css_data::CssCustomData, css_data_generated::CSS_DATA,
    store::DocumentStore,
};

/// The Language Service is the main entry point for interacting with CSSlsrs.
/// It contains a DocumentStore, a PositionEncoding and a reference to the CSS data.
pub struct LanguageService<'a> {
    pub store: DocumentStore,
    pub encoding: PositionEncoding,
    pub css_data: Vec<&'a CssCustomData<'a>>,
}

impl LanguageService<'_> {
    /// Create a new LanguageService with a default DocumentStore and a custom PositionEncoding.
    ///
    /// ## Example
    /// ```rust
    /// use csslsrs::service::LanguageService;
    /// use csslsrs::converters::PositionEncoding;
    ///
    /// let language_service = LanguageService::new(PositionEncoding::Utf8);
    /// ```
    pub fn new(encoding: PositionEncoding) -> Self {
        LanguageService {
            store: DocumentStore::new(),
            encoding,
            css_data: vec![&CSS_DATA],
        }
    }

    /// Create a new LanguageService with an already existing DocumentStore. This can be useful to share the same DocumentStore between multiple LanguageServices. If you do not need to share the DocumentStore, you can use the LanguageService::new() method instead.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use csslsrs::service::LanguageService;
    /// use csslsrs::store::DocumentStore;
    /// use csslsrs::converters::PositionEncoding;
    ///
    /// let store = DocumentStore::new();
    ///
    /// let language_service = LanguageService::new_with_store(store, PositionEncoding::Utf8);
    /// ```
    pub fn new_with_store(store: DocumentStore, encoding: PositionEncoding) -> Self {
        LanguageService {
            store,
            encoding,
            css_data: vec![&CSS_DATA],
        }
    }
}

impl Default for LanguageService<'_> {
    fn default() -> Self {
        LanguageService::new(PositionEncoding::Wide(
            crate::converters::WideEncoding::Utf16,
        ))
    }
}
