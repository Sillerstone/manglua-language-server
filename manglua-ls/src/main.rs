use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::{
    InitializeParams, InitializeResult, InitializedParams, MessageType,
};
use tower_lsp_server::{Client, LanguageServer, LspService, Server};
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Debug)]
struct Backend {
    client: Client,
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

fn init_logging() {
    let directives = if cfg!(debug_assertions) {
        "trace"
    } else {
        "warn"
    };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(directives));
    fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .without_time()
        .init();
}

#[tokio::main]
async fn main() {
    init_logging();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
}
