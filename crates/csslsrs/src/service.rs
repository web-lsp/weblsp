use crate::{converters::PositionEncoding, store::DocumentStore};

pub struct LanguageService {
    pub store: DocumentStore,
    pub encoding: PositionEncoding,
}

impl LanguageService {
    /// Create a new LanguageService with a default DocumentStore and a custom PositionEncoding.
    ///
    /// # Arguments
    ///
    /// * `encoding` - A custom PositionEncoding, in most cases LSP clients will expect Utf-16 encoding.
    ///
    /// # Example
    ///
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
        }
    }

    /// Create a new LanguageService with an already existing DocumentStore. This can be useful to share the same DocumentStore between multiple LanguageServices. If you do not need to share the DocumentStore, you can use the LanguageService::new() method instead.
    ///
    /// # Arguments
    ///
    /// * `store` - An existing instance of DocumentStore.
    /// * `encoding` - A custom PositionEncoding, in most cases LSP clients will expect Utf-16 encoding.
    ///
    /// # Example
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
    ///
    pub fn new_with_store(store: DocumentStore, encoding: PositionEncoding) -> Self {
        LanguageService { store, encoding }
    }
}

impl Default for LanguageService {
    fn default() -> Self {
        LanguageService::new(PositionEncoding::Wide(
            crate::converters::WideEncoding::Utf16,
        ))
    }
}
