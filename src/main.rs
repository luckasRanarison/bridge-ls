use bridge_ls::{config::Config, server::BridgeServer};
use std::{env, fs};
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let file_path = env::var("BRIDGE_LS_CONFIG").expect("BRIDGE_LS_CONFIG is not set");
    let buffer = fs::read_to_string(file_path).expect("Configuration file not found");
    let config = serde_json::from_str::<Config>(&buffer)
        .expect("Failed to parse configuration file")
        .register_builtins();
    let (service, socket) = LspService::new(|client| BridgeServer::new(client, config));

    Server::new(stdin, stdout, socket).serve(service).await;
}
