//! HTTP server setup: axum routing and request handling.
//!
//! This module configures the async HTTP server:
//! - Wire up axum routes for proxy, control API, and health checks
//! - Register middleware (logging, tracing, tower layers)
//! - Bind to the configured listen address and port
//! - Handle graceful shutdown on signals

use axum::{routing::get, Router};
use std::net::SocketAddr;

use crate::{config::GuardrailConfig, proxy::health_handler};

/// Build the axum router with all proxy and control plane routes.
pub fn build_app(_config: GuardrailConfig) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/status", get(|| async { "guardrail-v1 is running" }))
}

/// Start the HTTP proxy and control plane server on the given address.
/// Blocks indefinitely until shutdown signal or error.
pub async fn run_server(config: GuardrailConfig, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let app = build_app(config);
    // Bind to the address and serve; axum will handle connections indefinitely.
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
