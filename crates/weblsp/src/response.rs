use std::error::Error;

/// TMP: log the response.
pub fn handle_response(resp: lsp_server::Response) -> Result<(), Box<dyn Error + Sync + Send>> {
    eprintln!("handle_response: got {resp:?}");
    Ok(())
}
