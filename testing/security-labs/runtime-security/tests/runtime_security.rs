use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::TimeZone;
use contracts::{AssetClassV1, MarketEventV1, OhlcvBarV1, StrategyConfigV1, SymbolV1, VenueV1};
use finance_service::component::component_binding;
use runtime_security::{load_fixture, validate_service_binding};
use strategy_sandbox::{StrategySandbox, WasmRuntimePolicy};
use trading_core::FixedClock;
use trading_errors::TradingError;
use wasmcloud_bindings::InterfaceBindingV1;
use wit_parser::Resolve;

#[test]
fn busy_loop_fixture_triggers_runtime_budget_rejection() {
    let clock = Arc::new(FixedClock::new(
        chrono::Utc
            .with_ymd_and_hms(2026, 3, 5, 0, 0, 0)
            .single()
            .expect("clock"),
    ));
    let mut runtime = StrategySandbox::new(
        WasmRuntimePolicy {
            max_memory_bytes: 1024,
            max_execution_ms: 0,
        },
        clock,
    );
    runtime
        .load_wat(
            "busy-loop",
            &load_fixture("busy-loop.wat").expect("busy loop fixture"),
            StrategyConfigV1 {
                strategy_id: "slow".to_string(),
                model_version: "lab".to_string(),
                config_hash: "slow".to_string(),
                parameters: serde_json::json!({ "iterations": 5_000_000 }),
            },
        )
        .expect("load fixture");

    let error = runtime
        .on_market_event(
            "busy-loop",
            &MarketEventV1::Bar(OhlcvBarV1 {
                symbol: SymbolV1::new(VenueV1::Coinbase, AssetClassV1::Crypto, "BTC", "USD"),
                open_time: chrono::Utc
                    .with_ymd_and_hms(2026, 3, 5, 0, 0, 0)
                    .single()
                    .expect("open"),
                close_time: chrono::Utc
                    .with_ymd_and_hms(2026, 3, 5, 0, 1, 0)
                    .single()
                    .expect("close"),
                open: 100.0,
                high: 101.0,
                low: 99.0,
                close: 100.2,
                volume: 100.0,
            }),
            contracts::DeterminismKeyV1::new("lab-event", "v1", "busy"),
        )
        .expect_err("busy loop should exceed runtime policy");

    assert!(matches!(
        error,
        TradingError::RuntimePolicyViolation { details }
        if details.contains("execution time")
    ));
}

#[test]
fn finance_component_descriptor_matches_service_world() {
    let binding = component_binding();
    validate_service_binding(&binding).expect("service binding");

    let wit_dir = workspace_root().join("schemas/wit/v1");
    let mut resolve = Resolve::default();
    let (package_id, _) = resolve.push_dir(&wit_dir).expect("parse wit");
    let world = resolve
        .select_world(package_id, Some("service-component"))
        .expect("service world");
    let world_name = resolve.worlds.get(world).expect("world").name.clone();

    assert_eq!(world_name, "service-component");
    assert!(binding
        .interfaces
        .iter()
        .any(|interface| interface == &InterfaceBindingV1::service_world()));
}

#[test]
fn scaffolded_lab_families_have_manifests_and_readmes() {
    for relative in [
        "testing/security-labs/exploit-labs",
        "testing/security-labs/reverse-engineering",
        "testing/security-labs/fuzzing",
        "testing/security-labs/binary-analysis",
    ] {
        let root = workspace_root().join(relative);
        assert!(
            root.join("README.md").exists(),
            "{relative} is missing README.md"
        );
        assert!(
            root.join("manifest.toml").exists(),
            "{relative} is missing manifest.toml"
        );
        let manifest = fs::read_to_string(root.join("manifest.toml")).expect("manifest");
        assert!(manifest.contains("status = \"scaffold\""));
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}
