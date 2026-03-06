# Contributing to Systems Security Engineering with Rust

## Working Rules

- Start with a GitHub issue using the repository's existing issue forms.
- Prefer public-safe content that uses synthetic fixtures, deterministic builds, and controlled experiments.
- Keep new material inside the repository boundaries: book content under `docs/security-rust/book/`, public examples under `docs/security-rust/examples/`, labs under `testing/security-labs/`, and reusable teaching code in `shared/`.
- Do not add new canonical contracts or schema versions in support of the book unless the platform itself requires them.

## Chapter Requirements

Deep exemplar chapters should include:

- objective
- safety boundary
- runnable lab or example
- exploit or abuse path
- detection or hardening path
- Short Origin mapping
- verification steps

Placeholder chapters should include:

- learning goals
- intended lab type
- required Rust tooling
- an issue seed for deeper follow-up work

## Validation

Run the canonical book checks from the repository root:

```bash
cargo xtask docs security-book build
cargo xtask docs security-book test
```
