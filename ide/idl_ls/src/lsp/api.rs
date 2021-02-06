use crate::lsp::server::Server;
use tower_lsp::{jsonrpc::Result, lsp_types::*, LanguageServer};

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        log::info!("{:?}", params);

        let capabilities = crate::lsp::capabilities::capabilities();

        let InitializeParams { root_uri, .. } = params;

        self.session.open_analyzer(root_uri).await;

        Ok(InitializeResult {
            capabilities,
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "Server Initialized.")
            .await;
        self.session.start_analyer().await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.session.close_analyzer().await;
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.session.open_document(params).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.session.change_document(params).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        } = &params;
        self.session.close_document(params).await;
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let result = super::formatting::format_document(self.session.clone(), params).await;
        Ok(result)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(super::completion::completion(self.session.clone(), params).await)
    }

    // async fn document_highlight(
    // 		&self,
    // 		params: DocumentHighlightParams,
    // 	) -> Result<Option<Vec<DocumentHighlight>>> {
    // 	let DocumentHighlightParams {
    // 		text_document_position_params: TextDocumentPositionParams {
    // 			text_document,
    // 			position,
    // 		},
    // 		partial_result_params,
    // 		work_done_progress_params
    // 	} = &params;

    // 	self.client.log_message(MessageType::Info, format!("{:?}", params)).await;

    // 	Ok(None)
    // }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let result = super::symbol::get_document_symbol(self.session.clone(), params).await;
        Ok(result)
    }
}
