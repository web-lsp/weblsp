use crate::store::StoreEntry;
use crate::{
    converters::PositionEncoding,
    css_data::{CssCustomData, BASE_CSS_DATA, EMPTY_CSS_DATA},
    store::DocumentStore,
};
use lsp_types::{TextDocumentItem, Uri};
use serde::{Deserialize, Serialize};

/// The Language Service is the main entry point for interacting with CSSlsrs.
/// It contains a DocumentStore, a PositionEncoding and a reference to the CSS data.
pub struct LanguageService {
    pub(crate) store: DocumentStore,
    pub(crate) options: LanguageServiceOptions,
    base_css_data: &'static CssCustomData,
    pub(crate) css_data: Vec<CssCustomData>,
}

impl LanguageService {
    /// Create a new LanguageService. This will create a `DocumentStore`` internally. See `LanguageServiceOptions` for more information on the options available or `LanguageService::new_with_store` if you want to use an existing `DocumentStore`.
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

    /// Create a new `LanguageService` with an already existing `DocumentStore`. This can be useful to share the same `DocumentStore` between multiple `LanguageService`s. If you do not need to share the `DocumentStore`, you can use the `LanguageService::new()` method instead.
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

    /// Add custom CSS data to the `LanguageService`. This can be useful to add custom CSS properties, at-rules, or pseudo-classes to the `LanguageService`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use csslsrs::service::{LanguageService, LanguageServiceOptions};
    /// use csslsrs::css_data::CssCustomData;
    /// use csslsrs::converters::PositionEncoding;
    /// use csslsrs::css_data::{PropertyEntry};
    ///
    /// let mut language_service = LanguageService::new(LanguageServiceOptions::default());
    ///
    /// let custom_data = CssCustomData {
    ///         version: 1.1,
    ///         at_directives: None,
    ///         pseudo_classes: None,
    ///         pseudo_elements: None,
    ///         properties: Some(vec![
    ///             PropertyEntry {
    ///                 name: "my-custom-property".to_string(),
    ///                 restrictions: None,
    ///                 browsers: None,
    ///                 references: None,
    ///                 at_rule: None,
    ///                 relevance: None,
    ///                 syntax: None,
    ///                 description: None,
    ///                 status: None,
    ///                 values: None,
    ///             }
    ///         ])
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
    #[wasm_bindgen(skip_typescript)]
    pub struct WASMLanguageService {
        pub(crate) store: DocumentStore,
        pub(crate) options: LanguageServiceOptions,
        base_css_data: &'static CssCustomData,
        pub(crate) css_data: Vec<CssCustomData>,
    }

    #[wasm_bindgen(typescript_custom_section)]
    const TS_TYPE: &'static str = r#"
    export interface LanguageServiceOptions {
        encoding?: PositionEncoding;
        include_base_css_custom_data?: boolean;
    }

    export class WASMLanguageService {
        constructor(options: LanguageServiceOptions);
        upsertDocument(document: import("vscode-languageserver-textdocument").TextDocument): void;
        getHover: typeof get_hover;
        getDocumentColors: typeof get_document_colors;
        getColorPresentations: typeof get_color_presentations;
        getFoldingRanges: typeof get_folding_ranges;
        free(): void;
    }
    "#;

    #[wasm_bindgen]
    impl WASMLanguageService {
        #[wasm_bindgen(constructor, skip_typescript)]
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

        #[wasm_bindgen(skip_typescript, js_name = upsertDocument)]
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
