use crate::store::DocumentStore;

pub struct LanguageService {
    pub store: DocumentStore,
}

impl LanguageService {
    pub fn new() -> Self {
        LanguageService {
            store: DocumentStore::new(),
        }
    }

    pub fn new_with_store(store: DocumentStore) -> Self {
        LanguageService { store }
    }
}

impl Default for LanguageService {
    fn default() -> Self {
        Self::new()
    }
}
