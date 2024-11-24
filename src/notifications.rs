use lsp_server::Connection;
use lsp_types::DidOpenTextDocumentParams;
use std::error::Error;

/// Used by the main loop to handle notifications. Notifications are messages that the client sends to the server.
/// Notable notifications include `exit`, `textDocument/didOpen`, and `textDocument/didChange`.
pub fn handle_notification(
    notification: lsp_server::Notification,
    css_language_service: &mut csslsrs::service::LanguageService,
    _connection: &Connection,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    eprintln!("got notification: {notification:?}");
    let params: DidOpenTextDocumentParams =
        serde_json::from_value(notification.params).map_err(|e| {
            eprintln!("Failed to deserialize didOpen params: {:?}", e);
            Box::<dyn Error + Sync + Send>::from("Invalid didOpen parameters")
        })?;
    match notification.method.as_str() {
        "exit" => {
            eprintln!("Exiting server");
            std::process::exit(0);
        }
        // didOpen notification carry a textDocument item, which contains the document's URI and languageId.
        // We can use this information to determine in wich Language Service's store we should add the document.
        "textDocument/didOpen" => {
            eprintln!("Handling didOpen notification");
            match params.text_document.language_id.as_str() {
                "css" => {
                    eprintln!("Adding CSS document to CSS Language Service store");
                    css_language_service
                        .store
                        .get_or_update_document(params.text_document);
                }
                _ => {
                    eprintln!("Unsupported language: {}", params.text_document.language_id);
                }
            }
        }
        _ => {
            eprintln!("Unknown notification: {}", notification.method);
        }
    }
    Ok(())
}
