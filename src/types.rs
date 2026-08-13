//! Core domain types for the spending firewall.
//!
//! Represents payment intents, policy decisions, and budget snapshots
//! that flow through the policy evaluation pipeline.

use rust_decimal::Decimal;

/// A single payment request: destination, network, asset, and amount.
/// This is extracted from the x402 PaymentRequirements header and
/// passed to the policy engine for evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentIntent {
    /// Target address or upstream API host that will receive the payment.
    pub destination: String,
    /// Blockchain network (e.g., "base", "tempo").
    pub network: String,
    /// Asset/token being paid (e.g., "USDC").
    pub asset: String,
    /// Amount in USD (always precise via Decimal, never float).
    pub amount_usd: Decimal,
}

/// Current spending state: cumulative spend in rolling time windows and payment count.
/// Fetched from the ledger before policy evaluation to determine if the new payment
/// would breach any budget constraints.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BudgetSnapshot {
    /// Total USD spent in the rolling 1-minute window.
    pub spent_per_minute: Decimal,
    /// Total USD spent in the rolling 1-hour window.
    pub spent_per_hour: Decimal,
    /// Total USD spent in the rolling 24-hour window.
    pub spent_per_day: Decimal,
    /// Number of payments (authorized, not blocked) in the rolling 1-minute window.
    pub payments_per_minute: u32,
}

/// Policy evaluation outcome: whether a payment is allowed or blocked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecisionKind {
    Allow,
    Block,
}

/// Full policy decision with optional rule name and reason for blocking.
/// Returned by PolicyEngine::evaluate() and logged to the audit trail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decision {
    kind: DecisionKind,
    /// Name of the policy rule that triggered a block (e.g., "per_call_cap").
    rule: Option<String>,
    /// Human-readable explanation of why the decision was made.
    reason: Option<String>,
}

impl Decision {
    /// Construct an Allow decision (no rule or reason needed).
    pub fn allow() -> Self {
        Self {
            kind: DecisionKind::Allow,
            rule: None,
            reason: None,
        }
    }

    /// Construct a Block decision with the rule name and reason.
    pub fn block(rule: &str, reason: &str) -> Self {
        Self {
            kind: DecisionKind::Block,
            rule: Some(rule.to_string()),
            reason: Some(reason.to_string()),
        }
    }

    /// Check if this decision allows the payment.
    pub fn is_allowed(&self) -> bool {
        matches!(self.kind, DecisionKind::Allow)
    }

    /// Check if this decision blocks the payment.
    pub fn is_blocked(&self) -> bool {
        matches!(self.kind, DecisionKind::Block)
    }

    /// Get the blocking rule name, if blocked.
    pub fn rule(&self) -> Option<&str> {
        self.rule.as_deref()
    }

    /// Get the reason for blocking, if blocked.
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}
