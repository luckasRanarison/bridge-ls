use bridge_ls::{config::read_config, server::BridgeServer};
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let config = read_config();
    let (service, socket) = LspService::new(|client| BridgeServer::new(client, config));

    Server::new(stdin, stdout, socket).serve(service).await;
}
