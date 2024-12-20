mod css;
mod notifications;
mod requests;
mod response;
mod server;
use lsp_server::{Connection, Message};
use lsp_types::InitializeParams;
use server::get_server_capabilities;
use std::{error::Error, process::ExitCode};

/// Entry point for our WEBlsp server.
/// Heavily inspired by -> https://github.com/rust-lang/rust-analyzer/blob/master/lib/lsp-server/examples/goto_def.rs
fn main() -> Result<ExitCode, Box<dyn Error + Sync + Send>> {
    // Note that we must have our logging only write out to stderr.
    eprintln!("starting server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    let initialization_params = match connection.initialize(get_server_capabilities()) {
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

    // Run the server and wait for the two threads to end (typically by shutdown then exit messages).
    let exit_code = main_loop(connection, initialization_params, css_language_service)?;

    // Joins the IO threads to ensure all communication is properly finished.
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");

    Ok(exit_code)
}

/// Main loop of our WEBlsp server. Handles all incoming messages, and dispatches them to the appropriate language handler.
fn main_loop(
    connection: Connection,
    init_params: serde_json::Value,
    mut css_language_service: csslsrs::service::LanguageService,
) -> Result<ExitCode, Box<dyn Error + Sync + Send>> {
    let _init_params: InitializeParams = serde_json::from_value(init_params).unwrap();

    for msg in &connection.receiver {
        // Handle the rest of the messages.
        match msg {
            Message::Request(req) => {
                match connection.handle_shutdown(&req) {
                    Ok(value) => {
                        if value {
                            return Ok(ExitCode::SUCCESS);
                        }
                    }
                    Err(err) => {
                        eprintln!("Error handling shutdown request: {:?}", err);
                        return Ok(ExitCode::FAILURE);
                    }
                };

                requests::handle_request(req, &mut css_language_service, &connection)?;
            }
            Message::Response(resp) => {
                response::handle_response(resp)?;
            }
            Message::Notification(not) => {
                notifications::handle_notification(not, &mut css_language_service, &connection)?;
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}
