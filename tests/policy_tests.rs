use rust_decimal::Decimal;
use std::str::FromStr;

use guardrail_v1::{
    config::GuardrailConfig,
    policy::{BudgetSnapshot, PolicyEngine},
    types::PaymentIntent,
};

fn config() -> GuardrailConfig {
    GuardrailConfig::default()
}

#[test]
fn rejects_over_single_payment_cap() {
    let engine = PolicyEngine::new(config());
    let intent = PaymentIntent {
        destination: "0x123".to_string(),
        network: "base".to_string(),
        asset: "USDC".to_string(),
        amount_usd: Decimal::from_str("1.00").unwrap(),
    };

    let decision = engine.evaluate(&intent, &BudgetSnapshot::default());
    assert!(decision.is_blocked());
    assert_eq!(decision.rule(), Some("per_call_cap"));
}

#[test]
fn rejects_budget_window_overrun() {
    let engine = PolicyEngine::new(GuardrailConfig {
        caps: Default::default(),
        budget: Default::default(),
        rate: Default::default(),
        allow: Default::default(),
        global: Default::default(),
    });

    let intent = PaymentIntent {
        destination: "0x123".to_string(),
        network: "base".to_string(),
        asset: "USDC".to_string(),
        amount_usd: Decimal::from_str("0.25").unwrap(),
    };

    let snapshot = BudgetSnapshot {
        spent_per_minute: Decimal::from_str("1.75").unwrap(),
        spent_per_hour: Decimal::from_str("2.00").unwrap(),
        spent_per_day: Decimal::from_str("2.00").unwrap(),
        payments_per_minute: 8,
    };

    let decision = engine.evaluate(&intent, &snapshot);
    assert!(decision.is_blocked());
    assert_eq!(decision.rule(), Some("budget_window"));
}

#[test]
fn blocks_unknown_destination() {
    let engine = PolicyEngine::new(GuardrailConfig {
        allow: Default::default(),
        caps: Default::default(),
        budget: Default::default(),
        rate: Default::default(),
        global: Default::default(),
    });

    let intent = PaymentIntent {
        destination: "0xAttacker".to_string(),
        network: "base".to_string(),
        asset: "USDC".to_string(),
        amount_usd: Decimal::from_str("0.10").unwrap(),
    };

    let decision = engine.evaluate(&intent, &BudgetSnapshot::default());
    assert!(decision.is_blocked());
    assert_eq!(decision.rule(), Some("destination_not_allowed"));
}

#[test]
fn blocks_when_kill_switch_is_on() {
    let mut config = config();
    config.global.kill_switch = true;
    let engine = PolicyEngine::new(config);

    let intent = PaymentIntent {
        destination: "0x123".to_string(),
        network: "base".to_string(),
        asset: "USDC".to_string(),
        amount_usd: Decimal::from_str("0.01").unwrap(),
    };

    let decision = engine.evaluate(&intent, &BudgetSnapshot::default());
    assert!(decision.is_blocked());
    assert_eq!(decision.rule(), Some("kill_switch"));
}
