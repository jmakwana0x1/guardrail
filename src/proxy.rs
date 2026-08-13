//! HTTP proxy handler: forwards requests upstream and intercepts 402 Payment Required responses.
//!
//! This module implements the core request flow:
//! 1. Agent sends HTTP request to the proxy.
//! 2. Proxy forwards to upstream API.
//! 3. If upstream returns 402 Payment Required, proxy parses the payment requirement.
//! 4. Policy engine evaluates the payment intent against rules.
//! 5. If allowed: signer constructs ERC-3009 authorization, proxy replays with payment header.
//! 6. If blocked: proxy returns structured error explaining the blocking rule.
//! 7. All decisions are logged to the audit trail.

use axum::response::IntoResponse;

/// Health check endpoint: returns 200 OK to verify the proxy is running.
pub async fn health_handler() -> impl IntoResponse {
    "ok"
}
