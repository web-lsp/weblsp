use crate::cast;
use csslsrs::converters::PositionEncoding;
use csslsrs::service::LanguageService;
use lsp_server::{Connection, Message, Request, Response};
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
    match req.method.as_str() {
        "textDocument/documentColor" => {
            let (id, params) = cast::<lsp_types::request::DocumentColor>(req)?;
            let colors = language_service
                .get_document_colors(get_text_document(params.text_document, language_service)?);
            send_result(connection, id, serde_json::to_value(&colors).unwrap())?;
        }
        "textDocument/colorPresentation" => {
            let (id, params) = cast::<lsp_types::request::ColorPresentationRequest>(req)?;
            let presentations = language_service.get_color_presentations(
                lsp_types::ColorInformation {
                    color: params.color,
                    range: params.range,
                },
                // Erika se fout de ma gueule
                params.range,
            );
            send_result(
                connection,
                id,
                serde_json::to_value(&presentations).unwrap(),
            )?;
        }
        "textDocument/foldingRange" => {
            let (id, params) = cast::<lsp_types::request::FoldingRangeRequest>(req)?;
            let ranges = language_service
                .get_folding_ranges(get_text_document(params.text_document, language_service)?);
            send_result(connection, id, serde_json::to_value(&ranges).unwrap())?;
        }
        "textDocument/hover" => {
            let (id, params) = cast::<lsp_types::request::HoverRequest>(req)?;
            let hover = language_service.get_hover(
                get_text_document(
                    params.text_document_position_params.text_document,
                    language_service,
                )?,
                params.text_document_position_params.position,
                language_service.css_data,
            );
            send_result(connection, id, serde_json::to_value(&hover).unwrap())?;
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
    let text_document = match language_service.store.get(&text_document_identifier.uri) {
        Some(doc) => doc,
        None => return Err(Box::from("Document not found")),
    };

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
