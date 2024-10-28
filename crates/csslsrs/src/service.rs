use crate::{converters::PositionEncoding, store::DocumentStore};

pub struct LanguageService {
    pub store: DocumentStore,
    pub encoding: PositionEncoding,
}

impl LanguageService {
    pub fn new(encoding: PositionEncoding) -> Self {
        LanguageService {
            store: DocumentStore::new(),
            encoding,
        }
    }

    pub fn new_with_store(store: DocumentStore, encoding: PositionEncoding) -> Self {
        LanguageService { store, encoding }
    }
}

impl Default for LanguageService {
    fn default() -> Self {
        LanguageService::new(PositionEncoding::Wide(
            crate::converters::WideEncoding::Utf16,
        ))
    }
}
