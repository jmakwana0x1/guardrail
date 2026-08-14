//! ERC-3009 signing: construct EIP-712 typed-data signatures for payments.
//!
//! This module wraps the alloy library to:
//! - Load a signing key (dev key via env var for MVP)
//! - Build ERC-3009 `transferWithAuthorization` EIP-712 payloads
//! - Sign and serialize the authorization for the X-PAYMENT header
//! - Verify nonce state to prevent replay attacks

/// Load a dev signing key from environment (e.g., GUARDRAIL_PRIVATE_KEY).
/// (Implementation will use alloy::primitives::PrivateKeySigner.)
pub fn load_signer() -> Result<(), Box<dyn std::error::Error>> {
    todo!("load signer from env")
}

/// Build and sign an ERC-3009 authorization payload.
pub fn sign_payment(_destination: &str, _amount: &str, _nonce: u64) -> Result<String, Box<dyn std::error::Error>> {
    todo!("build and sign ERC-3009 payload")
}
