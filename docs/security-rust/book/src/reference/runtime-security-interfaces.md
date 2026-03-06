# Runtime Security Interfaces

The runtime-security slice is anchored to existing repository interfaces.

## Core Repository Surfaces

- `platform/runtime/strategy-sandbox/src/lib.rs`
- `platform/wasmcloud/bindings/src/lib.rs`
- `schemas/wit/v1/platform.wit`
- `shared/runtime-security/src/lib.rs`
- `testing/security-labs/runtime-security/manifest.toml`

## Important Types

- `WasmRuntimePolicy` defines memory and execution limits for guest code.
- `WasmComponentBindingV1` captures component metadata, config schema references, and required capabilities.
- `TraceContext` and `EvidenceManifestV1` connect runtime activity to audit-relevant context.

## Validation Rule

For v1, a public teaching artifact is runtime-compatible only if it:

- models a service-world binding correctly
- uses tracked fixtures from `testing/security-labs/runtime-security/`
- stays synthetic and deterministic enough for CI
