# Runtime Policy Lab

## Objective

Learn how Short Origin constrains synthetic guest modules with explicit memory and execution limits while keeping the experiment public-safe and reproducible.

## Safety Boundary

- only synthetic WAT fixtures are used
- no live targets, credentials, or private findings are required
- the lab is tied to repository-owned runtime surfaces rather than unmanaged external infrastructure

## Lab Inputs

The tutorial uses these tracked artifacts:

- `testing/security-labs/runtime-security/fixtures/oversized-state-id.txt`
- `testing/security-labs/runtime-security/fixtures/busy-loop.wat`
- `testing/security-labs/runtime-security/fixtures/guest-invalid-json.wat`
- `docs/security-rust/examples/wasm-sandboxing/Cargo.toml`
- `docs/security-rust/research/experiments/runtime-policy-matrix.md`

## Run the Validation Path

```bash
cargo xtask docs security-book test
cargo nextest run -p secure-patterns -p exploit-mitigation -p runtime-security -p security-instrumentation -p runtime-security-labs
cargo check -p security-example-wasm-sandboxing
```

## Abuse Path

The lab demonstrates three safe failure modes:

1. a guest snapshot that becomes too large for the configured memory budget
2. a guest snapshot that cannot be parsed as JSON
3. a guest call path that burns enough CPU to exceed the configured runtime budget

## Detection and Hardening Path

- keep the policy object small and explicit
- validate service-world bindings before trusting component descriptors
- keep fixtures checked in so regression tests can prove the defensive behavior
- document every lab input in version control so the book and the tests stay aligned

## Short Origin Mapping

This tutorial maps directly to the Wasmtime sandbox and component metadata used by the platform runtime. The runtime lab is intentionally small, but the defensive ideas scale to governed service and workflow deployments.

## Verification Steps

- confirm the oversized fixture triggers a runtime-policy violation during load
- confirm the malformed fixture triggers a parse failure during load
- confirm the busy-loop fixture triggers an execution-budget violation during event handling
