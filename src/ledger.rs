//! Ledger: SQLite-backed spend tracking and audit trail.
//!
//! This module manages persistent state:
//! - Record every payment decision (allowed and blocked) with timestamp, amount, rule
//! - Query spend totals within rolling time windows (last minute, hour, day)
//! - Provide audit log for operator inspection and compliance
//! - All writes are append-only (immutable history)
//!
//! Schema will include:
//! - payments: (id, ts, destination, network, asset, amount_usd, status, rule, reason)
//! - Window queries computed via SQL aggregate functions (SUM, COUNT)

use crate::types::BudgetSnapshot;

/// Get current spending state for policy evaluation.
/// Queries rolling window totals and payment count from ledger.
pub fn get_budget_snapshot() -> Result<BudgetSnapshot, Box<dyn std::error::Error>> {
    todo!("fetch budget snapshot from ledger")
}

/// Record a payment decision (allowed or blocked) to the audit trail.
pub fn record_payment(_destination: &str, _amount: &str, _decision: &str) -> Result<(), Box<dyn std::error::Error>> {
    todo!("write payment record to ledger")
}
