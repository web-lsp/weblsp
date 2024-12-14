use lsp_types::{ServerCapabilities, TextDocumentSyncCapability};

pub(crate) fn get_server_capabilities() -> serde_json::Value {
    let capabilities = ServerCapabilities {
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        color_provider: Some(lsp_types::ColorProviderCapability::Simple(true)),
        document_symbol_provider: Some(lsp_types::OneOf::Left(true)),
        folding_range_provider: Some(lsp_types::FoldingRangeProviderCapability::Simple(true)),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::FULL,
        )),
        ..Default::default()
    };

    serde_json::to_value(capabilities).unwrap()
}
