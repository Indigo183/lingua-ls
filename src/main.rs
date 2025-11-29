use tower_lsp::{LspService, Server};
use tracing::info;

use lingua_ls::rpc::*;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // File logger
    let file_appender = tracing_appender::rolling::never("logs", "lingua.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_writer(non_blocking)
        .init();

    info!("Starting LSP server");

    let (service, socket) = LspService::new(|client| Backend {
        client,
        language: Language::English,
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
