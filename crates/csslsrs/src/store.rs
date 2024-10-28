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
    /// If the document is not in the store, it will be added.
    pub fn get_or_update_document(&mut self, document: TextDocumentItem) -> &StoreEntry {
        // PERF: Figure out how to do this without cloning the document on updates
        let store_entry = self
            .documents
            .entry(document.uri.clone())
            .or_insert_with(|| {
                StoreEntry::new(
                    document.clone(),
                    LineIndex::new(&document.text),
                    parse_css(&document.text),
                )
            });

        if document.version != store_entry.document.version {
            store_entry.document = document.clone();
            store_entry.line_index = LineIndex::new(&document.text);
            store_entry.css_tree = parse_css(&document.text);
        }

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
