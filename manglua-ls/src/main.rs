mod protocol;
mod documents;

use tower_lsp_server::{LspService, Server};
use tracing_subscriber::{EnvFilter, fmt};
use crate::protocol::Backend;

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
        .without_time() // time is already present in editor's UI
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
