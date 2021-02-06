use tower_lsp::lsp_types::*;

pub(crate) fn capabilities() -> ServerCapabilities {
	let document_symbol_provider = Some(true);
	let document_formatting_provider = Some(true);
	//let document_highlight_provider = Some(true);

	let text_document_sync = Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
		open_close: Some(true),
		change: Some(TextDocumentSyncKind::Full),
		..Default::default()
	}));

	let hover_provider = Some(HoverProviderCapability::Simple(true));

	let completion_provider = Some(CompletionOptions {
		resolve_provider: Some(false),
		trigger_characters: None,
		work_done_progress_options: Default::default(),
	});

	ServerCapabilities {
		document_symbol_provider,
		completion_provider,
		text_document_sync,
		document_formatting_provider,
		hover_provider,
		..Default::default()
	}
}