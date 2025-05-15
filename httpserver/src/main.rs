use common::{HOST, HTTPSERVER_PORT, init_tracing};
use reqwest::Client;
use std::net::SocketAddr;

mod handlers;
mod middleware;
mod models;

use middleware::crypto::sdk_encryption;
use middleware::logging::full_logger;

#[derive(Clone)]
struct AppState {
    http_client: Client,
}

#[tokio::main]
async fn main() {
    init_tracing();

    let addr: SocketAddr = format!("{}:{}", HOST, HTTPSERVER_PORT).parse().unwrap();

    let state = AppState {
        http_client: Client::new(),
    };

    let with_encryption = {
        handlers::router::account_router()
            .layer(axum::middleware::from_fn(full_logger))
            .layer(axum::middleware::from_fn(sdk_encryption))
    };

    let without_encryption = {
        handlers::router::game_router()
            .merge(handlers::router::jsp_router())
            .merge(handlers::router::index_router())
            .layer(axum::middleware::from_fn(full_logger))
    };

    let app = with_encryption.merge(without_encryption).with_state(state);

    tracing::info!("Listening on http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
