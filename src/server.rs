use axum::{routing::get, Router};
use std::net::SocketAddr;

use crate::{config::GuardrailConfig, proxy::health_handler};

pub fn build_app(_config: GuardrailConfig) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/status", get(|| async { "guardrail-v1 is running" }))
}

pub async fn run_server(config: GuardrailConfig, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let app = build_app(config);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
