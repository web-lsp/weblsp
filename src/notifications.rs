use lsp_server::Connection;
use lsp_types::{DidChangeTextDocumentParams, DidOpenTextDocumentParams, TextDocumentItem};
use std::error::Error;

/// Used by the main loop to handle notifications. Notifications are messages that the client sends to the server.
/// Notable notifications include `exit`, `textDocument/didOpen`, and `textDocument/didChange`.
pub fn handle_notification(
    notification: lsp_server::Notification,
    css_language_service: &mut csslsrs::service::LanguageService,
    _connection: &Connection,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    match notification.method.as_str() {
        "exit" => {
            eprintln!("exit: shutting down server");
            std::process::exit(0);
        }
        "textDocument/didOpen" => {
            // didOpen notification carry a textDocument item, which contains the document's URI and languageId.
            // We can use this information to determine in wich Language Service's store we should add the document.
            let params: DidOpenTextDocumentParams =
                serde_json::from_value(notification.params).unwrap();
            match params.text_document.language_id.as_str() {
                "css" => {
                    eprintln!(
                        "textDocument/didOpen: adding CSS document to CSS Language Service store"
                    );
                    css_language_service
                        .store
                        .get_or_update_document(params.text_document);
                }
                _ => {
                    eprintln!(
                        "textDocument/didOpen: unsupported language {}",
                        params.text_document.language_id
                    );
                }
            }
        }
        "textDocument/didChange" => {
            // We need to update the document in the store with the new content at each change.
            let params: DidChangeTextDocumentParams =
                serde_json::from_value(notification.params).unwrap();
            css_language_service
                .store
                .get_or_update_document(TextDocumentItem {
                    uri: params.text_document.uri,
                    language_id: "css".to_string(),
                    version: params.text_document.version,
                    // Since we only support full text sync, we can just take the first change.
                    text: params.content_changes[0].text.clone(),
                });
        }
        _ => {
            eprintln!("unknown notification: {}", notification.method);
        }
    }
    Ok(())
}
