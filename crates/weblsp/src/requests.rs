use lsp_server::Connection;
use std::error::Error;
use std::str::FromStr;

use crate::css;

/// Used by the main loop. Based on the document's language, this function will dispatch the request to the appropriate language handler.
/// Requests are LSP features that the client wants to use, and the server must respond to each request.
pub fn handle_request(
    req: lsp_server::Request,
    css_language_service: &mut csslsrs::service::LanguageService,
    connection: &Connection,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let language_id = get_language_id(&req, css_language_service)?;
    match language_id.as_str() {
        "css" => {
            css::handle_request(css_language_service, connection, req)?;
        }
        _ => {
            eprintln!("unsupported language: {}", language_id);
        }
    }
    Ok(())
}

// TMP: TODO: For now, we use CSSlsrs' store, because we only support CSS. So I can just retrieve the document from this store from its URI.
// Soon, WEBlsp will support multiple languages, and have its own store. The document will be retrieved from this store instead, and the language services for each language will be created with LANGlsrs::new_with_store() instead of LANGlsrs::new().
// See issue -> https://github.com/web-lsp/weblsp/issues/1
/// Since requests only carry the document's URI, we need to extract the language ID from the store.
fn get_language_id(
    req: &lsp_server::Request,
    css_language_service: &mut csslsrs::service::LanguageService,
) -> Result<String, Box<dyn Error + Sync + Send>> {
    let text_document_identifier = req
        .params
        .get("textDocument")
        .and_then(|td| td.get("uri"))
        .and_then(|uri| uri.as_str())
        .ok_or("Missing or invalid 'textDocument.uri' in request parameters")?;

    let text_document_uri = lsp_types::Uri::from_str(text_document_identifier)
        .map_err(|_| "Invalid 'textDocument.uri' in request parameters")?;

    let store_entry = match css_language_service.get_document(&text_document_uri) {
        Some(doc) => doc,
        None => return Err(Box::from("Document not found")),
    };

    // Extract the data you need from store_entry
    let language_id = store_entry.document.language_id.clone();

    // The immutable borrow ends here
    Ok(language_id)
}
