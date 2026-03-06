# Trust Boundaries and Evidence

Short Origin already defines the runtime and data trust boundaries in `docs/adr/0002-trust-zones-and-evidence.md`. The security book builds on that rather than inventing a parallel model.

## Inputs

- `docs/adr/0002-trust-zones-and-evidence.md`
- `enterprise/policies/control_catalog.toml`
- `docs/security-rust/research/threat-models/runtime-plane-threat-model.md`
- `docs/security-rust/research/architecture-notes/public-safety-and-disclosure.md`

## Why It Matters

Runtime security is not just about catching malformed guest code. It is also about preserving the evidence chain for every decision and side effect that matters operationally.

That is why the v1 deep exemplar combines:

- runtime policy limits
- component descriptor validation
- trace correlation
- evidence manifests

## Security Engineering Lens

The trust-zone model keeps the public book honest. It forces every example to answer:

- which boundary is under study?
- which control should detect or constrain the abuse path?
- what evidence would prove the control worked?
- is the example safe to publish before remediation?
