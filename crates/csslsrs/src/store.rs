use biome_css_parser::CssParse;
use lsp_types::{TextDocumentItem, Uri};
use std::collections::HashMap;

use crate::parser::parse_css;

pub struct StoreEntry {
    pub document: TextDocumentItem,
    last_parsed_version: Option<i32>,
    pub css_tree: Option<CssParse>,
}

impl StoreEntry {
    pub fn update_css_tree_if_necessary(&mut self) {
        // If the CSS tree is not yet parsed, parse it
        if self.css_tree.is_none() {
            self.css_tree = Some(parse_css(&self.document.text));
            self.last_parsed_version = Some(self.document.version);
        }

        // If the document has been updated, re-parse the CSS tree
        if self.document.version != self.last_parsed_version.unwrap() {
            self.css_tree = Some(parse_css(&self.document.text));
            self.last_parsed_version = Some(self.document.version);
        }
    }

    // default
    pub fn new(document: TextDocumentItem) -> Self {
        Self {
            document,
            last_parsed_version: None,
            css_tree: None,
        }
    }
}

pub struct DocumentStore {
    documents: HashMap<Uri, StoreEntry>,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    pub fn get_or_update_document(&mut self, document: TextDocumentItem) -> &StoreEntry {
        let store_entry = self
            .documents
            .entry(document.uri.clone())
            .and_modify(|entry| {
                if document.version != entry.document.version {
                    entry.document = document.clone(); // TODO: Figure out how to do this without cloning
                }
            })
            .or_insert_with(|| StoreEntry::new(document));

        store_entry.update_css_tree_if_necessary();

        store_entry
    }

    pub fn remove(&mut self, uri: &Uri) {
        self.documents.remove(uri);
    }
}

impl Default for DocumentStore {
    fn default() -> Self {
        Self::new()
    }
}
