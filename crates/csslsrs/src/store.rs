use biome_css_parser::CssParse;
use lsp_types::{TextDocumentItem, Uri};
use rustc_hash::FxHashMap;

use crate::{converters::line_index::LineIndex, parser::parse_css};

pub struct StoreEntry {
    pub document: TextDocumentItem,
    pub(crate) line_index: LineIndex,
    last_parsed_version: Option<i32>,
    pub css_tree: CssParse,
}

impl StoreEntry {
    pub fn update_css_tree_if_necessary(&mut self) {
        // If the document has been updated, re-parse the CSS tree
        if self.last_parsed_version != Some(self.document.version) {
            self.css_tree = parse_css(&self.document.text);
            self.last_parsed_version = Some(self.document.version);
        }
    }

    pub(crate) fn new(
        document: TextDocumentItem,
        line_index: LineIndex,
        parsed_css: CssParse,
    ) -> Self {
        Self {
            document,
            line_index,
            last_parsed_version: None,
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

    pub fn get_or_update_document(&mut self, document: TextDocumentItem) -> &StoreEntry {
        // TODO: Figure out how to do this without cloning the document on updates
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
        }

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
