//! guardrail-v1: An x402 spending firewall for autonomous AI agents.
//!
//! This library provides a transparent HTTP proxy that sits between an AI agent
//! and paid APIs. When an upstream service returns a 402 Payment Required response,
//! the firewall intercepts the payment requirement, evaluates it against configured
//! policies (budgets, caps, allowlists, rate limits), and either signs + replays
//! the request with payment authorization, or blocks it with a clear reason.
//!
//! # Architecture
//!
//! - **config**: Load and manage TOML configuration (budgets, caps, allowlists)
//! - **types**: Core domain types (PaymentIntent, Decision, BudgetSnapshot)
//! - **policy**: PolicyEngine that evaluates payment intents against rules
//! - **proxy**: HTTP proxy handler that detects 402 and coordinates policy + signing
//! - **signer**: ERC-3009 EIP-712 authorization signing via alloy
//! - **ledger**: SQLite-backed spend history and audit trail
//! - **x402**: Parse and construct x402 payment types from headers
//! - **control**: REST API for live monitoring (spend, audit, metrics, kill-switch)
//! - **server**: HTTP server setup (axum) and request routing

pub mod config;
pub mod control;
pub mod ledger;
pub mod policy;
pub mod proxy;
pub mod server;
pub mod signer;
pub mod types;
pub mod x402;
