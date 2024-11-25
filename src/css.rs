use crate::cast;
use csslsrs::converters::PositionEncoding;
use csslsrs::service::LanguageService;
use lsp_server::{Connection, Message, Request, Response};
use lsp_types::request::HoverRequest;
use std::error::Error;

/// Initialize our CSS Language Service (CSSlsrs).
/// Used once at the start of the main loop, so the document store stays alive throughout the server's lifetime.
pub fn init_language_service() -> LanguageService {
    LanguageService::new(PositionEncoding::Utf8)
}

/// Handle WEBlsp's CSS requests. This function will be called by the main loop when a CSS request is received,
/// and will dispatch the request to our CSS Language Service (CSSlsrs).
pub fn handle_request(
    language_service: &mut LanguageService,
    connection: &Connection,
    req: Request,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    match cast::<HoverRequest>(req) {
        Ok((id, params)) => {
            // TODO: Move this to a separate function
            let position = params.text_document_position_params.position;
            let text_document_identifier = params.text_document_position_params.text_document;
            let text_document = match language_service.store.get(&text_document_identifier.uri) {
                Some(doc) => doc,
                None => return Err(Box::from("Document not found")),
            };
            let hover = language_service.get_hover(
                text_document.document.clone(),
                position,
                language_service.css_data,
            );
            eprintln!("handle_request: got hover -> {:?}", hover);
            let result = serde_json::to_value(&hover).unwrap();
            let resp = Response {
                id,
                result: Some(result),
                error: None,
            };
            connection.sender.send(Message::Response(resp))?;
        }
        Err(e) => {
            eprintln!("handle_request: failed to cast {:?}", e);
        }
    };
    Ok(())
}
