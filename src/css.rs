use crate::cast;
use csslsrs::converters::PositionEncoding;
use csslsrs::service::LanguageService;
use lsp_server::{Connection, ExtractError, Message, Request, Response};
use lsp_types::request::HoverRequest;
use std::error::Error;

pub fn init_language_service() -> LanguageService {
    LanguageService::new(PositionEncoding::Utf8)
}

/// Handle WEBlsp's CSS requests. This function will be called by the main loop when a CSS request is received,
/// and will dispatch the request to our CSS Language Service (CSSlsrs).
///
/// # Arguments
/// - `language_service` - The CSS Language Service.
/// - `connection` - The connection to the client.
/// - `req` - The request to be handled.
///
/// # Returns
/// - `Result<(), Box<dyn Error + Sync + Send>>` - A result that contains either `Ok(())` if the request was handled successfully or an error if the request failed to be handled.
pub fn handle_request(
    language_service: &mut LanguageService,
    connection: &Connection,
    req: Request,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    match cast::<HoverRequest>(req) {
        Ok((id, params)) => {
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
            let response = Response::new_ok(id, hover);
            connection.sender.send(Message::Response(response))?;
        }
        // Handle JSON error
        Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
        Err(ExtractError::MethodMismatch(_req)) => {
            // Handle method mismatch error
            return Err(Box::from("Method mismatch error"));
        }
    };
    // Handle other requests here
    Ok(())
}
