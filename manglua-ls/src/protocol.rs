use crate::documents::DOCUMENTS;
use tower_lsp_server::ls_types::{
    CompletionItem, CompletionOptions, CompletionParams, CompletionResponse,
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, Hover,
    HoverContents, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult,
    InitializedParams, MarkedString, MessageType, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind,
};
use tower_lsp_server::{Client, LanguageServer};

pub const MANGLUA_LANGUAGE_ID: &str = "manglua";

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
}

impl LanguageServer for Backend {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> tower_lsp_server::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_owned()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let document = params.text_document;
        if document.language_id == MANGLUA_LANGUAGE_ID {
            DOCUMENTS
                .add(document.uri, document.text, document.version)
                .await;
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        DOCUMENTS
            .update_partially(
                &params.text_document.uri,
                params.text_document.version,
                params.content_changes,
            )
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        DOCUMENTS.remove(&params.text_document.uri).await;
    }

    async fn completion(
        &self,
        _: CompletionParams,
    ) -> tower_lsp_server::jsonrpc::Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Example completion 1".to_string(), "Details 1".to_string()),
            CompletionItem::new_simple("Example completion 2".to_string(), "Details 2".to_string()),
        ])))
    }

    async fn hover(&self, _: HoverParams) -> tower_lsp_server::jsonrpc::Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("Hover information".to_string())),
            range: None,
        }))
    }

    async fn shutdown(&self) -> tower_lsp_server::jsonrpc::Result<()> {
        self.client
            .log_message(MessageType::INFO, "server is down")
            .await;
        Ok(())
    }
}
