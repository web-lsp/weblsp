use csslsrs::service::LanguageService;
use lsp_server::{Connection, Message, Request, Response};
use lsp_types::request::{
    ColorPresentationRequest, DocumentColor, DocumentSymbolRequest, FoldingRangeRequest,
    HoverRequest,
};
use std::error::Error;

use crate::requests::cast;

/// Initialize our CSS language service (CSSlsrs).
/// Used once at the start of the main loop, so the document store stays alive throughout the server's lifetime.
pub fn init_language_service() -> LanguageService {
    LanguageService::default()
}

/// Handle WEBlsp's CSS requests. This function will be called by the main loop when a CSS request is received,
/// and will dispatch the request to our CSS language service (CSSlsrs).
pub fn handle_request(
    language_service: &mut LanguageService,
    connection: &Connection,
    req: Request,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    match req.method.as_str() {
        "textDocument/documentColor" => {
            let (id, params) = cast::<DocumentColor>(req)?;
            let colors = language_service
                .get_document_colors(get_text_document(params.text_document, language_service)?);
            send_result(connection, id, serde_json::to_value(&colors).unwrap())?;
        }
        "textDocument/colorPresentation" => {
            let (id, params) = cast::<ColorPresentationRequest>(req)?;
            let presentations =
                language_service.get_color_presentations(lsp_types::ColorInformation {
                    color: params.color,
                    range: params.range,
                });
            send_result(
                connection,
                id,
                serde_json::to_value(&presentations).unwrap(),
            )?;
        }
        "textDocument/foldingRange" => {
            let (id, params) = cast::<FoldingRangeRequest>(req)?;
            let ranges = language_service
                .get_folding_ranges(get_text_document(params.text_document, language_service)?);
            send_result(connection, id, serde_json::to_value(&ranges).unwrap())?;
        }
        "textDocument/hover" => {
            let (id, params) = cast::<HoverRequest>(req)?;
            let hover = language_service.get_hover(
                get_text_document(
                    params.text_document_position_params.text_document,
                    language_service,
                )?,
                params.text_document_position_params.position,
            );
            send_result(connection, id, serde_json::to_value(&hover).unwrap())?;
        }
        "textDocument/documentSymbol" => {
            let (id, params) = cast::<DocumentSymbolRequest>(req)?;
            let symbols = language_service
                .get_document_symbols(get_text_document(params.text_document, language_service)?);
            send_result(connection, id, serde_json::to_value(&symbols).unwrap())?;
        }
        _ => {
            eprintln!("handle_request: unsupported request: {}", req.method);
        }
    }
    Ok(())
}

fn get_text_document(
    text_document_identifier: lsp_types::TextDocumentIdentifier,
    language_service: &LanguageService,
) -> Result<lsp_types::TextDocumentItem, Box<dyn Error + Sync + Send>> {
    let text_document = match language_service.get_document(&text_document_identifier.uri) {
        Some(doc) => doc,
        None => return Err(Box::from("Document not found")),
    };

    // TODO: It'd be great to avoid cloning the document here, might need to refactor methods to take a reference instead.
    Ok(text_document.document.clone())
}

fn send_result(
    connection: &Connection,
    id: lsp_server::RequestId,
    result: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let resp = Response {
        id,
        result: Some(result),
        error: None,
    };
    connection.sender.send(Message::Response(resp))?;
    Ok(())
}
