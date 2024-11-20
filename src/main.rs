pub mod css;
use lsp_server::{Connection, ExtractError, Message, Request, RequestId};
use lsp_types::{HoverProviderCapability, InitializeParams, ServerCapabilities};
use std::error::Error;

/// Entry point for our WEBlsp server.
/// Heavily inspired by -> https://github.com/rust-lang/rust-analyzer/blob/master/lib/lsp-server/examples/goto_def.rs
/// This is a generic LSP server that can handle any LSP request.
///
/// # Returns
/// - `Result<(), Box<dyn Error + Sync + Send>>` - A result that contains either `Ok(())` if the server was able to start and run successfully or an error if the server failed to start or run.
fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that we must have our logging only write out to stderr.
    eprintln!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    })
    .unwrap();
    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
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
///
/// # Arguments
/// - `connection` - The connection to the client.
/// - `params` - The initialization parameters.
///
/// # Returns
/// - `Result<(), Box<dyn Error + Sync + Send>>` - A result that contains either `Ok(())` if the server was able to start and run successfully or an error if the server failed to start or run.
fn main_loop(
    connection: Connection,
    params: serde_json::Value,
    mut css_language_service: csslsrs::service::LanguageService,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    eprintln!("starting example main loop");
    for msg in &connection.receiver {
        eprintln!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                eprintln!("got request: {req:?}");
                // Segregate by language and dispatch to the appropriate handler.
                // TODO: req.params.get("languageId") doesn't exist, I was just dreaming about it 😴
                if let Some(language_id) = req.params.get("languageId") {
                    match language_id.as_str() {
                        Some("html") => {
                            eprintln!("HTML is not supported yet");
                        }
                        Some("css") => {
                            // Handle CSS request
                            eprintln!("Handling CSS request");
                            css::handle_request(&mut css_language_service, &connection, req)?;
                        }
                        _ => {
                            eprintln!("Unknown or unsupported language: {language_id}");
                        }
                    }
                } else {
                    eprintln!("No languageId found in request");
                }
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                eprintln!("got notification: {not:?}");
            }
        }
    }
    Ok(())
}

/// Casts a request to a specific LSP request type.
/// This function will attempt to cast the request to the specified LSP request type.
/// If the request is not of the specified type, an error will be returned.
/// If the request is of the specified type, the request ID and parameters will be returned.
pub fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
