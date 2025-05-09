use common::{HOST, HTTPSERVER_PORT, init_tracing};
use reqwest::Client;
use std::net::SocketAddr;

mod cert_utils;
mod handlers;
mod middleware;
mod models;

use cert_utils::{check_cert_exists, get_openssl_config};
use middleware::crypto::sdk_encryption;
use middleware::logging::full_logger;

#[derive(Clone)]
struct AppState {
    http_client: Client,
}

#[tokio::main]
async fn main() {
    init_tracing();
    check_cert_exists();

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

    tracing::info!("Listening on https://{}", addr);
    axum_server::bind_openssl(addr, get_openssl_config())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
