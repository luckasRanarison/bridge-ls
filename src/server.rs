use crate::config::Config;
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};

#[derive(Debug)]
pub struct BridgeServer {
    config: Config,
    client: Client,
}

impl BridgeServer {
    pub fn new(client: Client, config: Config) -> Self {
        Self { client, config }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for BridgeServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions::default(),
                )),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "bridge_ls".to_owned(),
                version: option_env!("CARGO_PKG_VERSION").map(str::to_owned),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized")
            .await;
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        todo!()
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        todo!()
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
