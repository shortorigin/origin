# Harden Wasm Components

Use this guide when you need to validate that a service component still matches the expected WIT world, descriptor metadata, and evidence-aware runtime posture.

## Inputs

- `platform/wasmcloud/bindings/src/lib.rs`
- `schemas/wit/v1/platform.wit`
- `docs/security-rust/examples/distributed-security/Cargo.toml`
- `docs/security-rust/research/threat-models/runtime-plane-threat-model.md`

## Commands

```bash
cargo test -p wasmcloud-smoke-tests -p runtime-security-labs
cargo check -p security-example-distributed-security
```

## Procedure

1. Validate the binding constants and service-world expectations against `schemas/wit/v1/platform.wit`.
2. Confirm the component descriptor exposes the service world and at least one schema reference.
3. Check the distributed-security example to ensure trace and evidence correlation still compile in a public-safe path.
4. Review the threat model to confirm the control assumptions still match the current runtime plane.

## Done Criteria

- service-world bindings remain aligned
- smoke tests still round-trip descriptors
- trace and evidence examples still compile
