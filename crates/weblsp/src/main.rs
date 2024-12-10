mod css;
mod notifications;
mod requests;
mod response;
use lsp_server::{Connection, Message};
use lsp_types::{InitializeParams, ServerCapabilities, TextDocumentSyncCapability};
use std::error::Error;

/// Entry point for our WEBlsp server.
/// Heavily inspired by -> https://github.com/rust-lang/rust-analyzer/blob/master/lib/lsp-server/examples/goto_def.rs
fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that we must have our logging only write out to stderr.
    eprintln!("starting server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        color_provider: Some(lsp_types::ColorProviderCapability::Simple(true)),
        folding_range_provider: Some(lsp_types::FoldingRangeProviderCapability::Simple(true)),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::FULL,
        )),
        ..Default::default()
    })
    .unwrap();

    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(params) => params,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };

    // Init language services and start the main loop.
    let css_language_service = css::init_language_service();
    main_loop(connection, initialization_params, css_language_service)?;

    // Joins the IO threads to ensure all communication is properly finished.
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

/// Main loop of our WEBlsp server. Handles all incoming messages, and dispatches them to the appropriate language handler.
fn main_loop(
    connection: Connection,
    init_params: serde_json::Value,
    mut css_language_service: csslsrs::service::LanguageService,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _init_params: InitializeParams = serde_json::from_value(init_params).unwrap();
    for msg in &connection.receiver {
        eprintln!("new msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                requests::handle_request(req, &mut css_language_service, &connection)?;
                continue;
            }
            Message::Response(resp) => {
                response::handle_response(resp)?;
                continue;
            }
            Message::Notification(not) => {
                notifications::handle_notification(not, &mut css_language_service, &connection)?;
                continue;
            }
        }
    }
    Ok(())
}
