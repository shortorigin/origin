# Execution Plan

## Summary
- Phase 3 upgrades the `ui/` framework stack as one bounded slice: Leptos 0.8, current stable `wasm-bindgen` browser bindings, and any required browser or desktop-host compatibility fixes needed to keep the shared shell runtime coherent across PWA and Tauri targets.

## Task Contract
- Task contract: `plans/130-ui-framework-modernization/task-contract.json`
- GitHub issue: `#130`
- Branch: `refactor/130-ui-framework-modernization`

## Scope Boundaries
- Allowed touchpoints are the root `Cargo.toml` and `Cargo.lock`, `ui/`, `plans/`, and only minimal compatibility touchpoints in `platform/sdk/`, `shared/`, or `docs/` if the UI migration requires them.
- Non-goals are non-UI dependency modernization, workspace-wide async trait refactors, architecture-boundary changes, and the Edition 2024 uplift.

## Implementation Slices
- Add the phase-3 execution artifacts so the UI modernization is locally discoverable and bounded.
- Upgrade the root workspace UI dependency set to the latest stable Leptos/WASM/Tauri-compatible versions available at execution time.
- Fix compile-time and runtime API changes across the shared shell runtime, browser entrypoint, and host adapters while preserving browser/desktop parity.
- Re-run UI validation and hardening to confirm the upgraded stack still satisfies repository build and policy gates.

## Validation Plan
- Run `cargo check -p desktop_runtime`.
- Run `cargo check -p site`.
- Run `cargo check -p desktop_tauri`.
- Run `cargo xtask verify profile ui`.
- Run `cargo xtask ui-hardening`.

## Rollout and Rollback
- Roll out as a dedicated phase-3 PR tied to issue `#130`, isolated from the non-UI modernization slices.
- Roll back the dependency graph update, compatibility fixes, and lockfile changes together so the UI framework stack stays internally consistent.

## Open Questions
- None.
