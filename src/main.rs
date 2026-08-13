//! guardrail-v1 CLI entry point.
//!
//! This binary starts the HTTP proxy server on localhost with a configurable port.
//! It loads policy configuration from a TOML file, initializes structured logging,
//! and boots the proxy server that sits between agents and paid APIs.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use clap::Parser;

use guardrail_v1::{config::GuardrailConfig, server::run_server};

/// CLI arguments for guardrail-v1 server.
#[derive(Parser, Debug)]
#[command(name = "guardrail-v1", version, about = "x402 spending firewall")]
struct Cli {
    /// Path to the TOML configuration file containing policy rules, budgets, and allowlists.
    #[arg(long, default_value = "guardrail-v1.toml")]
    config: String,
    /// Port on which to listen for incoming proxy requests (default: 8402).
    #[arg(long, default_value = "8402")]
    port: u16,
}

/// Main entry point: parse CLI args, load config, initialize logging, and start the server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging via tracing with env-filter support.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .init();

    // Parse command-line arguments.
    let cli = Cli::parse();
    // Load TOML config; fall back to defaults if file is missing or invalid.
    let config = GuardrailConfig::from_file(&cli.config).unwrap_or_default();

    // Bind to localhost on the specified port.
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), cli.port);
    tracing::info!(%addr, "starting guardrail-v1");
    // Boot the proxy server; this blocks until shutdown.
    run_server(config, addr).await?;
    Ok(())
}
