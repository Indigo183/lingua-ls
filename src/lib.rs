pub mod rpc {
    use lsp_types::{ClientCapabilities, InitializeParams};
    use serde::{Deserialize, Serialize};

    pub type LSPAny = serde_json::Value;
    pub type LSPObject = std::collections::HashMap<String, LSPAny>;
}
