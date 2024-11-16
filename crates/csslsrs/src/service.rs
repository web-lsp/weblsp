use crate::{converters::PositionEncoding, css_data::CssCustomData, store::DocumentStore};
use serde_json;
use std::sync::LazyLock;

pub struct LanguageService {
    pub store: DocumentStore,
    pub encoding: PositionEncoding,
    pub css_data: &'static CssCustomData,
}

static CSS_DATA: LazyLock<CssCustomData> =
    LazyLock::new(
        || match serde_json::from_str(include_str!("../data/css-schema.json")) {
            Ok(data) => data,
            Err(e) => panic!("Failed to parse CSS data: {}", e),
        },
    );

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
            css_data: &CSS_DATA,
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
        LanguageService {
            store,
            encoding,
            css_data: &CSS_DATA,
        }
    }
}

impl Default for LanguageService {
    fn default() -> Self {
        LanguageService::new(PositionEncoding::Wide(
            crate::converters::WideEncoding::Utf16,
        ))
    }
}
