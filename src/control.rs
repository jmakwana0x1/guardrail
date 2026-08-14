//! Control plane API: REST endpoints for monitoring and management.
//!
//! This module provides axum route handlers for operator control:
//! - `GET /spend`: Current spend totals and remaining budgets per window
//! - `GET /audit`: Recent payment decisions (allowed and blocked)
//! - `POST /killswitch`: Toggle the global kill-switch on/off
//! - `GET /metrics`: Prometheus metrics endpoint (spend, decision counts, etc.)
//! - `GET /dashboard`: Minimal HTML dashboard with live gauges (optional)
//!
//! All responses are JSON except `/metrics` (Prometheus text format) and `/dashboard` (HTML).

/// Handler: GET /health — returns 200 if the server is running.
pub async fn health() -> impl axum::response::IntoResponse {
    "ok"
}

/// Handler: GET /spend — return current budget totals and remaining capacity.
pub async fn get_spend() -> impl axum::response::IntoResponse {
    "todo: return spend snapshot"
}

/// Handler: GET /audit — return recent payment decisions from ledger.
pub async fn get_audit() -> impl axum::response::IntoResponse {
    "todo: return audit log"
}

/// Handler: POST /killswitch — toggle global kill-switch via JSON body.
pub async fn toggle_killswitch() -> impl axum::response::IntoResponse {
    "todo: toggle kill-switch"
}

/// Handler: GET /metrics — return Prometheus-formatted metrics.
pub async fn get_metrics() -> impl axum::response::IntoResponse {
    "todo: return prometheus metrics"
}
