use crate::{
    converters::PositionEncoding,
    css_data::{CssCustomData, BASE_CSS_DATA, EMPTY_CSS_DATA},
    store::{DocumentStore, StoreEntry},
};
use lsp_types::{TextDocumentItem, Uri};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// The Language Service is the main entry point for interacting with CSSlsrs.
/// It contains a DocumentStore, a PositionEncoding and a reference to the CSS data.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct LanguageService {
    pub(crate) store: DocumentStore,
    pub(crate) options: LanguageServiceOptions,
    base_css_data: &'static CssCustomData,
    pub(crate) css_data: Vec<CssCustomData>,
}

#[cfg(not(feature = "wasm"))]
impl LanguageService {
    /// Create a new LanguageService. This will create a {DocumentStore} internally. See {LanguageServiceOptions} for more information on the options available or {LanguageService::new_with_store} if you want to use an existing {DocumentStore}.
    ///
    /// ## Example
    /// ```rust
    /// use csslsrs::service::{LanguageService, LanguageServiceOptions};
    /// use csslsrs::converters::PositionEncoding;
    ///
    /// let language_service = LanguageService::new(LanguageServiceOptions {
    ///    encoding: PositionEncoding::Utf8,
    ///    ..Default::default()
    /// });
    /// ```
    pub fn new(options: LanguageServiceOptions) -> Self {
        LanguageService {
            store: DocumentStore::new(),
            options,
            base_css_data: {
                if options.include_base_css_custom_data {
                    &BASE_CSS_DATA
                } else {
                    &EMPTY_CSS_DATA
                }
            },
            css_data: vec![],
        }
    }

    /// Create a new LanguageService with an already existing DocumentStore. This can be useful to share the same DocumentStore between multiple LanguageServices. If you do not need to share the DocumentStore, you can use the LanguageService::new() method instead.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use csslsrs::service::{LanguageService, LanguageServiceOptions};
    /// use csslsrs::store::DocumentStore;
    /// use csslsrs::converters::PositionEncoding;
    ///
    /// let store = DocumentStore::new();
    ///
    /// let language_service = LanguageService::new_with_store(LanguageServiceOptions::default(), store);
    /// ```
    pub fn new_with_store(options: LanguageServiceOptions, store: DocumentStore) -> Self {
        LanguageService {
            store,
            options,
            base_css_data: {
                if options.include_base_css_custom_data {
                    &BASE_CSS_DATA
                } else {
                    &EMPTY_CSS_DATA
                }
            },
            css_data: vec![],
        }
    }

    /// Add custom CSS data to the LanguageService. This can be useful to add custom CSS properties, at-rules, or pseudo-classes to the LanguageService.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use csslsrs::service::{LanguageService, LanguageServiceOptions};
    /// use csslsrs::css_data::CssCustomData;
    /// use csslsrs::converters::PositionEncoding;
    /// use csslsrs::css_data::{CssSection, AtDirectives, PseudoClasses, PseudoElements, Properties, PropertyEntry, PropertyAttributes};
    ///
    /// let mut language_service = LanguageService::new(LanguageServiceOptions::default());
    ///
    /// let custom_data = CssCustomData {
    ///     css: CssSection {
    ///         at_directives: AtDirectives { entry: vec![] },
    ///         pseudo_classes: PseudoClasses { entry: vec![] },
    ///         pseudo_elements: PseudoElements { entry: vec![] },
    ///         properties: Properties { entry: vec![
    ///             PropertyEntry {
    ///                 attributes: PropertyAttributes {
    ///                     name: "my-custom-property".to_string(),
    ///                     restriction: None,
    ///                     version: None,
    ///                     browsers: None,
    ///                     ref_: None,
    ///                     syntax: None,
    ///                 },
    ///                 desc: None,
    ///             }
    ///         ] }
    ///     }
    /// };
    ///
    /// language_service.add_css_custom_data(custom_data);
    /// ```
    pub fn add_css_custom_data(&mut self, data: CssCustomData) {
        self.css_data.push(data);
    }
}

impl LanguageService {
    pub(crate) fn get_css_custom_data(&self) -> Vec<&CssCustomData> {
        // Merge the base CSS data with the custom CSS data into a single vector for easier iteration
        std::iter::once(self.base_css_data)
            .chain(self.css_data.iter())
            .collect()
    }

    pub fn get_document(&self, uri: &Uri) -> Option<&StoreEntry> {
        self.store.get(uri)
    }

    pub fn upsert_document(&mut self, document: TextDocumentItem) -> &StoreEntry {
        self.store.upsert_document(document)
    }
}

#[cfg(not(feature = "wasm"))]
impl Default for LanguageService {
    fn default() -> Self {
        LanguageService::new(LanguageServiceOptions::default())
    }
}

#[derive(Clone, Copy)]
pub struct LanguageServiceOptions {
    pub encoding: PositionEncoding,
    pub include_base_css_custom_data: bool,
}

impl Default for LanguageServiceOptions {
    fn default() -> Self {
        LanguageServiceOptions {
            encoding: PositionEncoding::Wide(crate::converters::WideEncoding::Utf16),
            include_base_css_custom_data: true,
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::prelude::wasm_bindgen;

    use crate::converters::PositionEncoding;
    use crate::service::{BASE_CSS_DATA, EMPTY_CSS_DATA};
    use crate::store::DocumentStore;

    use super::{LanguageService, LanguageServiceOptions};

    #[wasm_bindgen]
    impl LanguageService {
        #[wasm_bindgen(constructor)]
        pub fn new(options: JSLanguageServiceOptions) -> Self {
            let include_base_css_custom_data = options.include_base_css_custom_data;

            LanguageService {
                store: DocumentStore::new(),
                options: options.into(),
                base_css_data: {
                    if include_base_css_custom_data {
                        &BASE_CSS_DATA
                    } else {
                        &EMPTY_CSS_DATA
                    }
                },
                css_data: vec![],
            }
        }
    }

    impl Default for LanguageService {
        fn default() -> Self {
            LanguageService::new(JSLanguageServiceOptions::default())
        }
    }

    #[wasm_bindgen]
    #[derive(Deserialize, Serialize)]
    pub enum JSPositionEncoding {
        Utf8,
        Utf16,
        Utf32,
    }

    impl From<JSPositionEncoding> for PositionEncoding {
        fn from(js_encoding: JSPositionEncoding) -> Self {
            match js_encoding {
                JSPositionEncoding::Utf8 => PositionEncoding::Utf8,
                JSPositionEncoding::Utf16 => {
                    PositionEncoding::Wide(crate::converters::WideEncoding::Utf16)
                }
                JSPositionEncoding::Utf32 => {
                    PositionEncoding::Wide(crate::converters::WideEncoding::Utf32)
                }
            }
        }
    }

    #[derive(Deserialize, Serialize)]
    #[wasm_bindgen]
    pub struct JSLanguageServiceOptions {
        encoding: JSPositionEncoding,
        include_base_css_custom_data: bool,
    }

    impl Default for JSLanguageServiceOptions {
        fn default() -> Self {
            JSLanguageServiceOptions {
                encoding: JSPositionEncoding::Utf16,
                include_base_css_custom_data: true,
            }
        }
    }

    impl From<JSLanguageServiceOptions> for LanguageServiceOptions {
        fn from(js_options: JSLanguageServiceOptions) -> Self {
            LanguageServiceOptions {
                encoding: js_options.encoding.into(),
                include_base_css_custom_data: js_options.include_base_css_custom_data,
            }
        }
    }
}
