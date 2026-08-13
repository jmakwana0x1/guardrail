//! Configuration loading and management for guardrail-v1 policies.
//!
//! This module defines TOML-deserializable structures for all policy rules:
//! global kill-switch, per-call caps, rolling budget windows, rate limits,
//! and allowlists for networks, assets, and payment destinations.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Global configuration: kill-switch and fail-closed default action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// When true, all payments are blocked until flipped back to false.
    #[serde(default)]
    pub kill_switch: bool,
    /// Default action if no rule matches: "deny" (fail-closed) or "allow".
    #[serde(default = "default_default_action")]
    pub default_action: String,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            kill_switch: false,
            default_action: "deny".to_string(),
        }
    }
}

fn default_default_action() -> String {
    "deny".to_string()
}

/// Per-call payment cap: reject any single payment exceeding this amount.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapsConfig {
    /// Maximum USD amount allowed in a single payment (fail-closed at 0.50 USD by default).
    #[serde(default = "default_max_payment_usd")]
    pub max_payment_usd: Decimal,
}

impl Default for CapsConfig {
    fn default() -> Self {
        Self {
            max_payment_usd: Decimal::new(50, 2),
        }
    }
}

fn default_max_payment_usd() -> Decimal {
    Decimal::new(50, 2)
}

/// Rolling budget windows: enforce cumulative spending caps per time period.
/// Blocks any payment that would exceed the window total.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetConfig {
    /// Maximum USD allowed in a rolling 1-minute window.
    #[serde(default = "default_per_minute_usd")]
    pub per_minute_usd: Decimal,
    /// Maximum USD allowed in a rolling 1-hour window.
    #[serde(default = "default_per_hour_usd")]
    pub per_hour_usd: Decimal,
    /// Maximum USD allowed in a rolling 24-hour window.
    #[serde(default = "default_per_day_usd")]
    pub per_day_usd: Decimal,
}

impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            per_minute_usd: Decimal::new(200, 2),
            per_hour_usd: Decimal::new(2000, 2),
            per_day_usd: Decimal::new(10000, 2),
        }
    }
}

fn default_per_minute_usd() -> Decimal { Decimal::new(200, 2) }
fn default_per_hour_usd() -> Decimal { Decimal::new(2000, 2) }
fn default_per_day_usd() -> Decimal { Decimal::new(10000, 2) }

/// Rate limiting: cap the number of payments per minute to catch runaway loops.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RateConfig {
    /// Maximum number of payments allowed within a rolling 1-minute window.
    #[serde(default)]
    pub max_payments_per_minute: u32,
}

impl Default for RateConfig {
    fn default() -> Self {
        Self { max_payments_per_minute: 30 }
    }
}

/// Allowlists: only allow payments to whitelisted networks, assets, and destinations.
/// By default, fail-closed: anything not explicitly on a list is blocked.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllowConfig {
    /// List of allowed blockchain networks (e.g., "base", "tempo").
    #[serde(default)]
    pub networks: Vec<String>,
    /// List of allowed assets/tokens (e.g., "USDC").
    #[serde(default)]
    pub assets: Vec<String>,
    /// List of allowed payment destinations: blockchain addresses or upstream API hosts.
    #[serde(default)]
    pub destinations: Vec<String>,
}

impl Default for AllowConfig {
    fn default() -> Self {
        Self {
            networks: vec!["base".to_string(), "tempo".to_string()],
            assets: vec!["USDC".to_string()],
            destinations: vec!["0x123".to_string(), "api.example-data.com".to_string()],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuardrailConfig {
    #[serde(default)]
    pub global: GlobalConfig,
    #[serde(default)]
    pub caps: CapsConfig,
    #[serde(default)]
    pub budget: BudgetConfig,
    #[serde(default)]
    pub rate: RateConfig,
    #[serde(default)]
    pub allow: AllowConfig,
}

impl Default for GuardrailConfig {
    fn default() -> Self {
        Self {
            global: GlobalConfig::default(),
            caps: CapsConfig::default(),
            budget: BudgetConfig::default(),
            rate: RateConfig::default(),
            allow: AllowConfig::default(),
        }
    }
}

impl GuardrailConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
