//! Policy engine: evaluates payment intents against configured rules and constraints.
//!
//! The PolicyEngine is the core decision-maker: given a payment intent (destination,
//! network, asset, amount) and current budget snapshot (spending state), it checks
//! all rules in order and returns an Allow or Block decision.
//!
//! Rules are checked in fail-closed order:
//! 1. Global kill-switch
//! 2. Per-call cap (single payment max)
//! 3. Budget windows (rolling minute/hour/day totals)
//! 4. Rate limit (payments per minute)
//! 5. Network allowlist
//! 6. Asset allowlist
//! 7. Destination allowlist

use rust_decimal::Decimal;

use crate::{
    config::GuardrailConfig,
    types::{BudgetSnapshot, Decision, PaymentIntent},
};

/// The policy engine: holds configuration and evaluates payments against rules.
pub struct PolicyEngine {
    config: GuardrailConfig,
}

impl PolicyEngine {
    /// Create a new policy engine with the given configuration.
    pub fn new(config: GuardrailConfig) -> Self {
        Self { config }
    }

    /// Evaluate a payment intent against all policy rules.
    ///
    /// Takes the current budget snapshot (spending state) and checks all rules
    /// in fail-closed order. Returns the first blocking rule, or Allow if all pass.
    pub fn evaluate(&self, intent: &PaymentIntent, snapshot: &BudgetSnapshot) -> Decision {
        // Rule 1: Check global kill-switch (fail-closed emergency stop).
        if self.config.global.kill_switch {
            return Decision::block("kill_switch", "global spending is disabled");
        }

        // Rule 2: Check per-call cap (reject oversized single payments).
        if intent.amount_usd > self.config.caps.max_payment_usd {
            return Decision::block("per_call_cap", "single payment exceeds max_payment_usd");
        }

        // Rule 3a: Check rolling per-minute budget.
        let minute_total = snapshot.spent_per_minute + intent.amount_usd;
        if minute_total > self.config.budget.per_minute_usd {
            return Decision::block("budget_window", "per-minute budget would be exceeded");
        }

        // Rule 3b: Check rolling per-hour budget.
        let hour_total = snapshot.spent_per_hour + intent.amount_usd;
        if hour_total > self.config.budget.per_hour_usd {
            return Decision::block("budget_window", "per-hour budget would be exceeded");
        }

        // Rule 3c: Check rolling per-day budget.
        let day_total = snapshot.spent_per_day + intent.amount_usd;
        if day_total > self.config.budget.per_day_usd {
            return Decision::block("budget_window", "per-day budget would be exceeded");
        }

        // Rule 4: Check rate limit (catch runaway payment loops).
        if snapshot.payments_per_minute >= self.config.rate.max_payments_per_minute {
            return Decision::block("rate_limit", "rate limit reached for this minute");
        }

        // Rule 5: Check network allowlist (only whitelisted chains).
        if !self.config.allow.networks.iter().any(|net| net == &intent.network) {
            return Decision::block("network_not_allowed", "destination network is not allowed");
        }

        // Rule 6: Check asset allowlist (only whitelisted tokens).
        if !self.config.allow.assets.iter().any(|asset| asset == &intent.asset) {
            return Decision::block("asset_not_allowed", "asset is not allowed");
        }

        // Rule 7: Check destination allowlist (only whitelisted payees).
        if !self.config.allow.destinations.iter().any(|dest| dest == &intent.destination) {
            return Decision::block("destination_not_allowed", "destination is not on the allowlist");
        }

        // All rules passed: allow the payment.
        Decision::allow()
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new(GuardrailConfig::default())
    }
}
