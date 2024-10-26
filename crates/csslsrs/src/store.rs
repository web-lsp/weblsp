// A static store of text documents

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub fn document_store() -> &'static Mutex<DocumentStore> {
    static mut DOCUMENT_STORE: OnceLock<Mutex<DocumentStore>> = OnceLock::new();
    unsafe { DOCUMENT_STORE.get_or_init(|| Mutex::new(DocumentStore::new())) }
}

use lsp_types::TextDocumentItem;

pub struct DocumentStore {
    documents: HashMap<String, TextDocumentItem>,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    pub fn insert(&mut self, document: TextDocumentItem) {
        self.documents.insert(document.uri.to_string(), document);
    }

    pub fn get(&self, uri: &str) -> Option<&TextDocumentItem> {
        self.documents.get(uri)
    }

    pub fn remove(&mut self, uri: &str) {
        self.documents.remove(uri);
    }
}

impl Default for DocumentStore {
    fn default() -> Self {
        Self::new()
    }
}
