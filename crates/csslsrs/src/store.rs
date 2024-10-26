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
    pub fn get_parsed_css(&mut self) -> Option<&CssParse> {
        // If the CSS tree is not yet parsed, parse it
        if self.css_tree.is_none() {
            self.css_tree = Some(parse_css(&self.document.text));
            self.last_parsed_version = Some(self.document.version);

            return self.css_tree.as_ref();
        }

        // If the document has been updated, re-parse the CSS tree
        if self.document.version != self.last_parsed_version.unwrap() {
            self.css_tree = Some(parse_css(&self.document.text));
            self.last_parsed_version = Some(self.document.version);

            return self.css_tree.as_ref();
        } else {
            // Otherwise, return the cached CSS tree
            return self.css_tree.as_ref();
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

    pub fn insert(&mut self, document: TextDocumentItem) {
        self.documents.insert(
            document.uri.clone(),
            StoreEntry {
                document,
                last_parsed_version: None,
                css_tree: None,
            },
        );
    }

    pub fn insert_or_get(&mut self, document: TextDocumentItem) -> &StoreEntry {
        self.documents
            .entry(document.uri.clone())
            .or_insert(StoreEntry {
                document,
                last_parsed_version: None,
                css_tree: None,
            })
    }

    pub fn get(&self, uri: &Uri) -> Option<&StoreEntry> {
        self.documents.get(uri)
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
