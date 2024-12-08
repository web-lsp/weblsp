use crate::store::StoreEntry;
use crate::{
    converters::PositionEncoding,
    css_data::{CssCustomData, BASE_CSS_DATA, EMPTY_CSS_DATA},
    store::DocumentStore,
};
use lsp_types::{TextDocumentItem, Uri};
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// The Language Service is the main entry point for interacting with CSSlsrs.
/// It contains a DocumentStore, a PositionEncoding and a reference to the CSS data.
pub struct LanguageService {
    pub(crate) store: DocumentStore,
    pub(crate) options: LanguageServiceOptions,
    base_css_data: &'static CssCustomData,
    pub(crate) css_data: Vec<CssCustomData>,
}

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

    pub fn get_document(&self, uri: &Uri) -> Option<&StoreEntry> {
        self.store.get(uri)
    }

    pub fn upsert_document(&mut self, document: TextDocumentItem) -> &StoreEntry {
        self.store.upsert_document(document)
    }

    pub(crate) fn get_css_custom_data(&self) -> Vec<&CssCustomData> {
        // Merge the base CSS data with the custom CSS data into a single vector for easier iteration
        std::iter::once(self.base_css_data)
            .chain(self.css_data.iter())
            .collect()
    }
}

impl Default for LanguageService {
    fn default() -> Self {
        LanguageService::new(LanguageServiceOptions::default())
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct LanguageServiceOptions {
    #[serde(default)]
    pub encoding: PositionEncoding,
    #[serde(default)]
    pub include_base_css_custom_data: bool,
}

impl Default for LanguageServiceOptions {
    fn default() -> Self {
        LanguageServiceOptions {
            encoding: PositionEncoding::Utf16,
            include_base_css_custom_data: true,
        }
    }
}

#[cfg(feature = "wasm")]
pub mod wasm_bindings {
    use wasm_bindgen::prelude::wasm_bindgen;
    use wasm_bindgen::JsValue;

    use crate::css_data::CssCustomData;
    use crate::service::{BASE_CSS_DATA, EMPTY_CSS_DATA};
    use crate::store::DocumentStore;
    use crate::wasm_text_document::create_text_document;

    use super::LanguageServiceOptions;
    extern crate console_error_panic_hook;

    // We use a different struct for the WASM bindings because otherwise implementations conflicts with the non-WASM version
    // which works just fine when using a single feature, but makes development harder when using both features at the same time.
    #[wasm_bindgen]
    pub struct WASMLanguageService {
        pub(crate) store: DocumentStore,
        pub(crate) options: LanguageServiceOptions,
        base_css_data: &'static CssCustomData,
        pub(crate) css_data: Vec<CssCustomData>,
    }

    #[wasm_bindgen]
    impl WASMLanguageService {
        #[wasm_bindgen(constructor)]
        pub fn new(options: JsValue) -> Self {
            console_error_panic_hook::set_once();
            let options: LanguageServiceOptions = serde_wasm_bindgen::from_value(options).unwrap();
            let include_base_css_custom_data = options.include_base_css_custom_data;

            WASMLanguageService {
                store: DocumentStore::new(),
                options,
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

        #[wasm_bindgen]
        pub fn upsert_document(&mut self, document: JsValue) {
            let document = create_text_document(document);
            self.store.upsert_document(document);
        }

        pub(crate) fn get_css_custom_data(&self) -> Vec<&CssCustomData> {
            // Merge the base CSS data with the custom CSS data into a single vector for easier iteration
            std::iter::once(self.base_css_data)
                .chain(self.css_data.iter())
                .collect()
        }
    }

    impl Default for WASMLanguageService {
        fn default() -> Self {
            WASMLanguageService::new(JsValue::NULL)
        }
    }
}
