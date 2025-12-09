pub mod rpc {
    // use lsp_types::{ClientCapabilities, InitializeParams};
    // use serde::{Deserialize, Serialize};

    pub type LSPAny = serde_json::Value;
    // pub type LSPObject = std::collections::HashMap<String, LSPAny>;

    use tokio::sync::Mutex;
    pub use tower_lsp::jsonrpc::Result;
    pub use tower_lsp::lsp_types::*;
    pub use tower_lsp::{Client, LanguageServer, LspService, Server};
    pub use tracing::{debug, error, info, warn};
    pub use tracing_subscriber::{EnvFilter, fmt, prelude::*};

    #[derive(Debug)]
    pub enum Language {
        English,
        Deutsch,
    }

    #[derive(Debug)]
    pub struct Backend {
        pub client: Client,
        pub language: Language,
        pub content: Mutex<Vec<String>>, // lines
    }

    #[tower_lsp::async_trait]
    impl LanguageServer for Backend {
        async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
            info!("initialising...");
            Ok(InitializeResult {
                server_info: Some(ServerInfo {
                    name: String::from("Lingua LS"),
                    version: Some(String::from("pre-alpha")),
                }),
                capabilities: ServerCapabilities {
                    position_encoding: None,
                    text_document_sync: Some(TextDocumentSyncCapability::Kind(
                        TextDocumentSyncKind::FULL,
                    )),
                    selection_range_provider: None,
                    hover_provider: None,
                    completion_provider: None,
                    signature_help_provider: None,
                    definition_provider: None,
                    type_definition_provider: None,
                    implementation_provider: None,
                    references_provider: None,
                    document_highlight_provider: None,
                    document_symbol_provider: None,
                    workspace_symbol_provider: None,
                    code_action_provider: None,
                    code_lens_provider: None,
                    document_formatting_provider: None,
                    document_range_formatting_provider: None,
                    document_on_type_formatting_provider: None,
                    rename_provider: None,
                    document_link_provider: None,
                    color_provider: None,
                    folding_range_provider: None,
                    declaration_provider: None,
                    execute_command_provider: None,
                    workspace: None,
                    call_hierarchy_provider: None,
                    semantic_tokens_provider: None,
                    moniker_provider: None,
                    linked_editing_range_provider: None,
                    inline_value_provider: None,
                    inlay_hint_provider: None,
                    diagnostic_provider: None,
                    experimental: None,
                },
            })
        }

        async fn initialized(&self, _: InitializedParams) {
            info!("server initialised!");
            self.client
                .log_message(MessageType::INFO, "server initialised!")
                .await;
        }

        async fn shutdown(&self) -> Result<()> {
            Ok(())
        }

        async fn did_open(&self, params: DidOpenTextDocumentParams) {
            info!(
                "file opened: \n{}",
                params
                    .text_document
                    .text
                    .lines()
                    .map(|line| format!("| {line}\n"))
                    .collect::<String>()
            );

            *self.content.lock().await = params
                .text_document
                .text
                .lines()
                .map(String::from)
                .collect();
        }

        async fn did_change(&self, params: DidChangeTextDocumentParams) {
            info!(
                "file changed: \n{}",
                params.content_changes[0]
                    .text
                    .lines()
                    .map(|line| format!("| {line}\n"))
                    .collect::<String>()
            );

            *self.content.lock().await = params.content_changes[0]
                .text
                .lines()
                .map(String::from)
                .collect();
        }

        async fn did_save(&self, params: DidSaveTextDocumentParams) {
            info!("file saved to path {}", params.text_document.uri);
        }

        async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<LSPAny>> {
            info!("got command: {:#?}", params);
            Ok(None)
        }
    }
}
