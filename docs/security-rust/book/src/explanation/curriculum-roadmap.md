# Curriculum Roadmap

Runtime security is the deep v1 slice. The rest of the curriculum is scaffolded here with contribution-ready placeholders.

## Secure Rust Programming

- Learning goals: model least authority, capability-style APIs, and reviewable safe abstractions.
- Intended lab type: API-boundary design and compile-time constraint exercises.
- Required tooling: `cargo`, `clippy`, `cargo-nextest`.
- Issue seed: `[Docs]: add safe abstraction and capability-based API chapters`.

## Unsafe Rust and Memory Analysis

- Learning goals: explain where `unsafe` appears, how to audit it, and how to document invariants.
- Intended lab type: synthetic FFI and memory-model audits.
- Required tooling: `cargo`, `clippy`, `cargo-nextest`.
- Issue seed: `[Research]: add unsafe Rust audit exercises with explicit invariants`.

## Exploit Development and Mitigation

- Learning goals: reason about bounded parser abuse, control-flow constraints, and mitigation-first examples.
- Intended lab type: safe parser and memory-model simulations backed by `cargo-fuzz`.
- Required tooling: `cargo`, `cargo-fuzz`, `cargo-nextest`.
- Issue seed: `[Research]: deepen exploit-mitigation labs and parser-hardening notes`.

## Reverse Engineering and Binary Analysis

- Learning goals: inspect Wasm and component metadata without shipping harmful payloads.
- Intended lab type: synthetic module inspection and descriptor analysis.
- Required tooling: `cargo`, `rustfmt`, `cargo-nextest`.
- Issue seed: `[Docs]: add reverse-engineering walkthroughs for synthetic components`.

## Network and Protocol Security

- Learning goals: model secure protocol boundaries and capability-limited network surfaces.
- Intended lab type: pure-Rust protocol parsers and least-authority network examples.
- Required tooling: `cargo`, `clippy`, `cargo-nextest`.
- Issue seed: `[Docs]: add network security examples built around bounded interfaces`.

## Secure Distributed Systems

- Learning goals: reason about trust boundaries, identity propagation, and evidence-aware workflows.
- Intended lab type: distributed trace and evidence correlation examples.
- Required tooling: `cargo`, `cargo-nextest`, `cargo-audit`.
- Issue seed: `[Research]: add distributed trust-boundary threat models and examples`.

## Infrastructure Hardening

- Learning goals: document Wasm isolation, zero-trust assumptions, and deployment guardrails.
- Intended lab type: configuration validation and hardened descriptor examples.
- Required tooling: `cargo`, `cargo-audit`, `cargo-nextest`.
- Issue seed: `[Docs]: expand infrastructure hardening guides for Wasm-first workloads`.

## Observability and Security Telemetry

- Learning goals: correlate traces, policy decisions, and evidence manifests.
- Intended lab type: telemetry pipelines with synthetic evidence records.
- Required tooling: `cargo`, `cargo-nextest`, `mdbook`.
- Issue seed: `[Feature]: extend security instrumentation examples and telemetry reference pages`.

## Threat Modeling and Adversarial Simulation

- Learning goals: build attack-surface and trust-boundary models that map to repository controls.
- Intended lab type: threat-model documents and adversarial simulation matrices.
- Required tooling: `mdbook`, `cargo`, `cargo-nextest`.
- Issue seed: `[Research]: expand adversarial simulation notes for runtime and workflow boundaries`.
