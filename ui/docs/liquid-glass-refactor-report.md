# Liquid Glass UI Refactor Report

## Current-State Findings

### Inventory and Classification

| Area | Scope | Status | Findings |
| --- | --- | --- | --- |
| Token pipeline | `ui/crates/system_ui/tokens/tokens.toml`, `ui/crates/system_ui/build.rs`, generated token outputs | `compliant` | Replaced the mixed raw/material/surface model with explicit `raw.*` and `semantic.*` layers, updated generator outputs, and regenerated CSS/Tailwind artifacts. |
| Shell primitives | `ui/crates/system_ui/src/origin_primitives/surfaces.rs`, `ui/crates/system_ui/src/origin_primitives/window.rs` | `compliant` | `Viewport`, `Layer`, `WindowSurface`, `TitlebarRegion`, `ResizeHandleRegion`, `WindowTitle`, and `WindowBody` now emit semantic shell roles and stable token-backed attributes instead of neutral wrappers. |
| Shared shell components | `ui/crates/system_ui/src/origin_components/{shell,windowing,navigation,actions}.rs` | `compliant` | Shell-facing components now use canonical `SurfaceRole`, `ElevationRole`, and `ControlTone` metadata; legacy `Dock*`/`GlassWindowFrame` exports were removed. |
| Desktop runtime composition | `ui/crates/desktop_runtime/src/components/{taskbar,window}.rs`, `ui/crates/desktop_runtime/src/components.rs` | `compliant` | Runtime taskbar/window composition now consumes `Taskbar*` and semantic window props, and taskbar layout math derives from generated shell metrics instead of hardcoded chrome widths. |
| Window manager metrics | `ui/crates/desktop_runtime/src/window_manager.rs` | `compliant` | Minimum window geometry now derives from generated token constants (`360x240`). |
| Browser/Tauri persistence and cache naming | `ui/crates/desktop_runtime/src/persistence.rs`, `ui/crates/platform_host_web/src/bridge/interop/wasm.rs`, `ui/crates/desktop_tauri/src/{prefs,cache}.rs`, `ui/crates/platform_host/src/fs/types.rs` | `compliant` | Canonical naming now uses `origin.*`; compatibility fallback migrates legacy `retrodesk*` prefs/cache/layout keys on read and deletes the old key/domain after migration. |
| Browser preview pipeline | `ui/crates/site/Trunk.toml`, `ui/crates/site/tailwind.config.js`, generated CSS | `compliant` | Trunk hashing/SRI behavior remains intact; generated CSS/Tailwind now reflect the new semantic shell model. |
| Verification automation | `xtask/src/main.rs` | `compliant` | Added a shell-style hygiene gate to `verify-ui`/`verify profile ui` to fail on new hardcoded shell styling in source files outside approved generated locations. |
| Manual visual regression capture | browser preview and taskbar/menu/window state visual review artifacts | `partially compliant` | Automated build and runtime checks passed, but this change set did not add a stored visual-regression artifact for the shell states listed in the refactor brief. |
| Legacy shell source artifacts | legacy `Dock*` definitions removed from exports and no longer used by runtime | `legacy / contradictory` | The old dock-first API surface existed before the refactor and conflicted with the target shell language; it has now been removed in place rather than as a standalone file deletion. |

No targeted shell primitive or shared shell component remains in a `placeholder` state after the refactor; the targeted wrappers now emit semantic structure and token-backed behavior.

## Token Model Changes

- Introduced explicit raw token groups: `raw.color`, `raw.space`, `raw.type`, `raw.blur`, `raw.radius`, `raw.motion`, `raw.border`.
- Introduced semantic alias groups: `semantic.surface`, `semantic.control`, `semantic.text`, `semantic.border`, `semantic.state`, `semantic.shell`, `semantic.layer`.
- Normalized shell metrics to the refactor targets:
  - taskbar height `44px`
  - taskbar button height `32px`
  - taskbar clock minimum width `84px`
  - titlebar height `38px`
  - titlebar control size `28px`
  - desktop padding `12px`
  - window content padding `16px`
  - window minimum size `360x240`
  - resize handle edge `8px`
  - resize handle corner `12px`
  - resize handle hit outset `4px`
  - desktop icon tile size `80px`
- Reduced shell typography to one tokenized shell scale (`caption`, `label`, `body`, `title`, `code`) and shell radius to `8`, `12`, `16`, `999`.
- Rebuilt generated Rust constants, CSS custom properties, Tailwind config, and high-contrast/reduced-motion overrides from the new semantic model.

## Primitive and Component Changes

- Added semantic shell enums in `system_ui`:
  - `SurfaceRole::{Shell, Taskbar, WindowActive, WindowInactive, Menu, Modal}`
  - `ElevationRole::{Embedded, Raised, Floating, Modal}`
  - `ControlTone::{Neutral, Accent, Danger}`
- Reworked shell primitives and components to emit semantic role/elevation/tone attributes instead of baseline visual recipes at call sites.
- Updated the shared button layer so shell controls carry `data-ui-control-tone` and uniform interaction state metadata.
- Removed legacy public shell exports:
  - `Dock`
  - `DockButton`
  - `DockSection`
  - `GlassWindowFrame`
- Updated the prelude to expose `Taskbar*` and the new semantic shell enums.

## Runtime Shell Changes

- Replaced runtime `Dock*` usage with `Taskbar`, `TaskbarButton`, `TaskbarOverflowButton`, and `TaskbarSection`.
- Updated managed windows to use semantic window roles, including `ControlTone::Danger` for close controls.
- Rebased taskbar layout calculations on token constants:
  - taskbar button height
  - clock minimum width
  - desktop padding
- Rebased window minimum constraints on generated token constants in `window_manager`.
- Implemented compatibility migration for:
  - `origin.shell.layout.v1` with fallback from `retrodesk.layout.v1`
  - `origin.shell.theme.v1` with fallback from `system.desktop_theme.v2` and `retrodesk.theme.v1`
  - `origin.shell.wallpaper.v1` with fallback from `system.desktop_wallpaper.v1`
  - `origin.shell.terminal-history.v1` with fallback from `retrodesk.terminal_history.v1`
  - `origin.explorer.prefs.v1` with fallback from `retrodesk.explorer.prefs.v1`
  - `origin-explorer-cache-v1` with fallback from `retrodesk-explorer-cache-v1`
  - browser IndexedDB name `origin_os`

## Removed Legacy Patterns

- Removed dock-first shell API exports from `system_ui`.
- Removed `GlassWindowFrame` from the shared public API surface.
- Removed runtime taskbar dependence on `Dock*` component names.
- Removed repository-facing “retro desktop shell” wording from `desktop_runtime` crate documentation.
- Removed generated reliance on the old monolithic token groups (`material`, old `surface`/`shadow` layering split) in favor of semantic alias tokens.

## Verification Results

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --workspace --all-targets` | PASS |
| `cargo check -p system_ui` | PASS |
| `cargo check -p desktop_runtime` | PASS |
| `cargo check -p site` | PASS |
| `cargo check -p desktop_tauri` | PASS |
| `cargo test -p system_ui` | PASS |
| `cargo test -p desktop_runtime` | PASS |
| `cargo test -p xtask` | PASS |
| `cargo ui-build` | PASS |
| `cargo verify-ui` | PASS |

### Notes

- `cargo verify-ui` completed after the new shell-style hygiene gate was added to `xtask`.
- Browser preview mount verification was exercised through the existing Trunk preview smoke path inside `verify-ui`.
- Shell interaction behavior remains covered by existing reducer/runtime tests for taskbar focus, minimize/restore, modal focus rules, and boot/persistence flows.
- Pixel-perfect manual visual review was not separately captured in an artifact during this run; the automated browser smoke/build pipeline passed.

## Remaining Follow-Up Work

- Add dedicated browser automation for taskbar overflow, clock menu, and notification/control-center interaction flows if the team wants visual regression coverage beyond reducer/runtime tests.
- If cross-version browser IndexedDB migration for `retrodesk_os` must preserve preexisting browser-only app state, add an explicit old-database copy step; this refactor renamed the browser database and migrated pref/cache compatibility keys at read time.

## Final Checklist

- `DONE` Replaced the token source with explicit raw and semantic layers.
- `DONE` Normalized shell metrics to the required compact chrome values.
- `DONE` Rebuilt generated Rust constants, CSS variables, and Tailwind config from the new token model.
- `DONE` Rewrote the targeted shell primitives to emit semantic shell roles and token-backed structure.
- `DONE` Reworked shared shell components to consume semantic roles and canonical shell defaults.
- `DONE` Removed `Dock*` and `GlassWindowFrame` from public `system_ui` exports and prelude.
- `DONE` Updated desktop runtime taskbar/window composition to consume `Taskbar*` and semantic window APIs.
- `DONE` Replaced hardcoded taskbar layout math inputs with token-derived shell metrics.
- `DONE` Moved managed window minimum sizing to generated token constants.
- `DONE` Renamed shell/cache/prefs persistence keys to `origin.*` and implemented compatibility migration for legacy `retrodesk*` keys.
- `DONE` Added automated source hygiene checks for forbidden hardcoded shell styling patterns.
- `DONE` Preserved browser preview and Trunk/SRI build behavior.
- `DONE` Regenerated browser CSS artifacts and Tailwind config.
- `DONE` Ran formatting, clippy, workspace tests, UI checks, UI build, and UI verification.
- `DONE` Produced this implementation report with findings, verification, checklist, and diff summary.

## File-Level Diff Summary

### Modified Files

- `ui/crates/system_ui/tokens/tokens.toml`
- `ui/crates/system_ui/build.rs`
- `ui/crates/system_ui/src/foundation.rs`
- `ui/crates/system_ui/src/lib.rs`
- `ui/crates/system_ui/src/origin_tokens/{mod.rs,schema.rs}`
- `ui/crates/system_ui/src/origin_primitives/{mod.rs,surfaces.rs,window.rs}`
- `ui/crates/system_ui/src/origin_components/{actions.rs,mod.rs,navigation.rs,shell.rs,windowing.rs}`
- `ui/crates/desktop_runtime/src/{components.rs,lib.rs,persistence.rs,window_manager.rs}`
- `ui/crates/desktop_runtime/src/components/{taskbar.rs,window.rs}`
- `ui/crates/platform_host/src/fs/types.rs`
- `ui/crates/platform_host_web/src/bridge/{mod.rs,interop/wasm.rs}`
- `ui/crates/platform_host_web/src/storage/tauri_prefs.rs`
- `ui/crates/desktop_tauri/src/{cache.rs,prefs.rs}`
- `xtask/src/main.rs`

### Generated Files

- `ui/crates/site/src/generated/tokens.css`
- `ui/crates/site/src/generated/tailwind.css`
- `ui/crates/site/tailwind.config.js`

### Deleted Files

- None

### Legacy Files Removed

- None at the filesystem level
- Legacy APIs removed in place:
  - `Dock`
  - `DockButton`
  - `DockSection`
  - `GlassWindowFrame`
