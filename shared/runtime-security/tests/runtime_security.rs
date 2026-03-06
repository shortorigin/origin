use std::sync::Arc;

use chrono::TimeZone;
use contracts::{StrategyConfigV1, VerificationStatusV1};
use runtime_security::{load_fixture, oversized_state_wat, validate_service_binding};
use strategy_sandbox::{StrategySandbox, WasmRuntimePolicy};
use trading_core::FixedClock;
use trading_errors::TradingError;
use wasmcloud_bindings::{SignedComponentRefV1, WasmComponentBindingV1};

#[test]
fn oversized_guest_state_is_rejected_by_runtime_policy() {
    let clock = Arc::new(FixedClock::new(
        chrono::Utc
            .with_ymd_and_hms(2026, 3, 5, 0, 0, 0)
            .single()
            .expect("clock"),
    ));
    let mut runtime = StrategySandbox::new(
        WasmRuntimePolicy {
            max_memory_bytes: 160,
            max_execution_ms: 250,
        },
        clock,
    );

    let error = runtime
        .load_wat(
            "oversized",
            &oversized_state_wat().expect("oversized wat"),
            StrategyConfigV1 {
                strategy_id: "oversized".to_string(),
                model_version: "lab".to_string(),
                config_hash: "oversized".to_string(),
                parameters: serde_json::json!({ "verification": VerificationStatusV1::Draft }),
            },
        )
        .expect_err("oversized snapshot should violate policy");

    assert!(matches!(
        error,
        TradingError::RuntimePolicyViolation { details }
        if details.contains("memory policy")
    ));
}

#[test]
fn malformed_guest_snapshot_is_rejected() {
    let clock = Arc::new(FixedClock::new(
        chrono::Utc
            .with_ymd_and_hms(2026, 3, 5, 0, 0, 0)
            .single()
            .expect("clock"),
    ));
    let mut runtime = StrategySandbox::new(WasmRuntimePolicy::default(), clock);

    let error = runtime
        .load_wat(
            "invalid-json",
            &load_fixture("guest-invalid-json.wat").expect("fixture"),
            StrategyConfigV1 {
                strategy_id: "invalid-json".to_string(),
                model_version: "lab".to_string(),
                config_hash: "invalid-json".to_string(),
                parameters: serde_json::json!({}),
            },
        )
        .expect_err("invalid snapshot should fail");

    assert!(matches!(error, TradingError::Parse { .. }));
}

#[test]
fn synthetic_service_binding_stays_runtime_compatible() {
    let binding = WasmComponentBindingV1::service(
        "runtime-lab",
        SignedComponentRefV1 {
            component_ref: "ghcr.io/shortorigin/runtime-lab:wasm".to_string(),
            digest: "sha256:runtime-lab".to_string(),
            signature_ref: None,
        },
        "lab",
        vec!["schemas/wit/v1/platform.wit".to_string()],
        Vec::new(),
    );

    validate_service_binding(&binding).expect("service binding");
}
