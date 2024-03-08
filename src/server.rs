use crate::config::Config;
use dashmap::DashMap;
use std::path::Path;
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};

#[derive(Debug)]
pub struct BridgeServer {
    config: Config,
    client: Client,
    documents: DashMap<Url, String>,
}

impl BridgeServer {
    pub fn new(client: Client, config: Config) -> Self {
        Self {
            client,
            config,
            documents: DashMap::default(),
        }
    }
}

impl BridgeServer {
    pub fn handle_formatting(&self) {}
}

#[tower_lsp::async_trait]
impl LanguageServer for BridgeServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
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
            .log_message(MessageType::INFO, "Server initialized")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let document = params.text_document.text;

        self.documents.insert(uri, document);
        self.client
            .log_message(
                MessageType::INFO,
                format!("File opened: {}", params.text_document.uri),
            )
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = &params.text_document.uri;

        if let Some(mut document) = self.documents.get_mut(uri) {
            *document = params
                .content_changes
                .into_iter()
                .next()
                .map(|c| c.text)
                .unwrap_or_default();
        }
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let file_path = params.text_document.uri.path();
        let file_path = Path::new(file_path);
        let extension = file_path.extension().and_then(|e| e.to_str());
        let document = self.documents.get(&params.text_document.uri);
        let formatter = extension.and_then(|e| self.config.get_formatter(e));

        if let Some((document, formatter)) = document.zip(formatter) {
            let result = formatter.format(document.as_str(), file_path);

            match result {
                Ok(result) => {
                    let end_line = document.lines().count();
                    let start_pos = Position::new(0, 0);
                    let end_pos = Position::new(end_line as u32, 0);
                    let range = Range::new(start_pos, end_pos);
                    let edits = vec![TextEdit::new(range, result)];

                    return Ok(Some(edits));
                }
                Err(err) => {
                    self.client
                        .log_message(MessageType::ERROR, err.to_string())
                        .await
                }
            }
        }

        Ok(None)
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri);
        self.client
            .log_message(
                MessageType::INFO,
                format!("File closed: {}", params.text_document.uri),
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
