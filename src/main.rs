use tokio::sync::Mutex;
use tower_lsp::{LspService, Server};
use tracing::{debug, error, info, warn};

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

    debug!("debug mode active");
    info!("starting LSP server");

    let (service, socket) = LspService::new(|client| Backend {
        client,
        language: Language::English,
        content: Mutex::new(Vec::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
