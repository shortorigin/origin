# Toolchain and Validation

The security book is validated with the Rust toolchain, not a separate docs-only island.

## Primary Commands

```bash
cargo xtask docs security-book build
cargo xtask docs security-book preview
cargo fuzz run parse_policy_line -- -max_total_time=30
```

## Backing Artifacts

- `shared/exploit-mitigation/fuzz/fuzz_targets/parse_policy_line.rs`
- `testing/security-labs/runtime-security/manifest.toml`
- `docs/security-rust/examples/network-security/Cargo.toml`
- `docs/security-rust/examples/infrastructure-hardening/Cargo.toml`

## Expected Tooling

- `cargo`
- `rustfmt`
- `clippy`
- `cargo-audit`
- `cargo-fuzz`
- `cargo-nextest`
- `mdbook`

## Validation Intent

`cargo xtask docs security-book test` builds the book, validates links and chapter references, runs the teaching-crate and lab tests with `cargo nextest`, and preserves the existing targeted runtime checks already used elsewhere in the repository.
