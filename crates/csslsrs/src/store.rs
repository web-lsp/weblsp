use std::collections::hash_map::Entry;

use biome_css_parser::CssParse;
use lsp_types::{TextDocumentItem, Uri};
use rustc_hash::FxHashMap;

use crate::{converters::line_index::LineIndex, parser::parse_css};

pub struct StoreEntry {
    pub document: TextDocumentItem,
    // Calculating the offset of every line in a document is quite expensive, but is required for every conversion from
    // offset to position (and vice versa). For this reason, we cache the line index here, updating it whenever the document is updated.
    pub(crate) line_index: LineIndex,
    pub css_tree: CssParse,
}

impl StoreEntry {
    pub(crate) fn new(
        document: TextDocumentItem,
        line_index: LineIndex,
        parsed_css: CssParse,
    ) -> Self {
        Self {
            document,
            line_index,
            css_tree: parsed_css,
        }
    }
}

pub struct DocumentStore {
    documents: FxHashMap<Uri, StoreEntry>,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self {
            documents: FxHashMap::default(),
        }
    }

    /// Get a document from the store, updating it as well if necessary.
    /// If the document is not yet in the store, it will be added.
    pub fn get_or_update_document(&mut self, document: TextDocumentItem) -> &StoreEntry {
        let uri = document.uri.clone();
        let store_entry = self.documents.entry(uri);

        match store_entry {
            Entry::Vacant(entry) => {
                let line_index = LineIndex::new(&document.text);
                let css_tree = parse_css(&document.text);

                entry.insert(StoreEntry::new(document, line_index, css_tree))
            }
            Entry::Occupied(mut entry) => {
                let mut_entry = entry.get_mut();

                if document.version != mut_entry.document.version {
                    mut_entry.document = document;
                    mut_entry.line_index = LineIndex::new(&mut_entry.document.text);
                    mut_entry.css_tree = parse_css(&mut_entry.document.text);
                }

                entry.into_mut()
            }
        }
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    impl DocumentStore {
        pub fn len(&self) -> usize {
            self.documents.len()
        }

        pub fn is_empty(&self) -> bool {
            self.documents.is_empty()
        }
    }

    #[test]
    fn test_document_store() {
        let mut store = DocumentStore::new();

        let document = TextDocumentItem {
            uri: Uri::from_str("file:///test").unwrap(),
            language_id: "css".to_string(),
            version: 1,
            text: "body { color: red; }".to_string(),
        };

        let store_entry = store.get_or_update_document(document.clone());

        assert_eq!(store_entry.document, document);
        assert_eq!(store.len(), 1);

        // Get a new version of the same document
        let updated_document = TextDocumentItem {
            uri: Uri::from_str("file:///test").unwrap(),
            language_id: "css".to_string(),
            version: 2,
            text: "body { color: blue; }".to_string(),
        };

        let updated_store_entry = store.get_or_update_document(updated_document.clone());

        assert_eq!(updated_store_entry.document, updated_document);

        // We should still only have one document in the store
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_document_store_remove() {
        let mut store = DocumentStore::new();

        let document = TextDocumentItem {
            uri: Uri::from_str("file:///test").unwrap(),
            language_id: "css".to_string(),
            version: 1,
            text: "body { color: red; }".to_string(),
        };

        store.get_or_update_document(document.clone());

        assert!(store.documents.contains_key(&document.uri));
        assert!(!store.is_empty());

        store.remove(&document.uri);

        assert!(!store.documents.contains_key(&document.uri));
        assert!(store.is_empty());
    }
}
