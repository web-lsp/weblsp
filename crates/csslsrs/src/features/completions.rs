use lsp_types::{CompletionItem, CompletionList, Position, TextDocumentItem};

use crate::{service::LanguageService, store::StoreEntry};

fn compute_completions(store_entry: &StoreEntry, position: Position) -> CompletionList {
    CompletionList {
        is_incomplete: true,
        items: vec![],
    }
}

impl LanguageService {
    pub fn get_completions(
        &self,
        document: TextDocumentItem,
        position: Position,
    ) -> CompletionList {
        let store_document = self.store.get(&document.uri);

        match store_document {
            Some(store_document) => compute_completions(store_document, position),
            None => empty_completion_list(),
        }
    }
}

fn empty_completion_list() -> CompletionList {
    CompletionList {
        is_incomplete: true,
        items: vec![],
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings {
    use std::str::FromStr;

    use crate::{
        features::completions::{compute_completions, empty_completion_list},
        service::wasm_bindings::WASMLanguageService,
    };
    use lsp_types::{Position, Uri};
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"
        declare function get_completions(documentUri: string, position: import("vscode-languageserver-types").Position): import("vscode-languageserver-types").FoldingRange[];
    "#;

    #[wasm_bindgen]
    impl WASMLanguageService {
        #[wasm_bindgen(skip_typescript, js_name = getCompletions)]
        pub fn get_completions(&self, document_uri: String, position: JsValue) -> JsValue {
            let store_document = self.store.get(&Uri::from_str(&document_uri).unwrap());

            let completions = match store_document {
                Some(store_document) => {
                    let position: Position = serde_wasm_bindgen::from_value(position).unwrap();
                    compute_completions(store_document, position)
                }
                None => empty_completion_list(),
            };

            serde_wasm_bindgen::to_value(&completions).unwrap()
        }
    }
}
