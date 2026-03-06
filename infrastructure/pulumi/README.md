# Pulumi Infrastructure Workspace

This workspace provides a modular Pulumi TypeScript codebase for provisioning AWS infrastructure and Cloudflare edge networking with explicit AWS↔Cloudflare wiring.

## Layout

- `bootstrap-state/`: one-time Pulumi stack that provisions S3 state storage and DynamoDB locking.
- `live/`: environment stacks (`dev`, `stage`, `prod`) for AWS + Cloudflare infrastructure.
- `scripts/`: operator-friendly wrappers for preview and apply flows.

## Design Goals

- Reproducible deployments with environment isolation.
- Stage-gated promotion between continuous delivery (`dev`) and manual production approval.
- Strict module boundaries between AWS resources, Cloudflare resources, and cross-cloud wiring.
- Private AWS origin exposure through Cloudflare Tunnel.
- Consistent defaults aligned to:
  - Region: `us-west-2`
  - Environments: `dev`, `stage`, `prod`
  - Instance strategy: `dev=t4g.small`, `stage=m7gd.medium`, `prod=m7gd.medium`

## Quick Start

1. Diagnose local prerequisites from the repository root:
```bash
cargo doctor --domain infra
```

2. Bootstrap state backend:
```bash
./scripts/bootstrap-state.sh
```

3. Configure secrets for live stacks:
```bash
cd live
pulumi stack select dev
pulumi config set --secret short-origin:tunnelSecret "<BASE64_32_BYTE_SECRET>"
pulumi config set --secret short-origin:surrealdbRootPassword "<PASSWORD>"
pulumi stack select stage
pulumi config set --secret short-origin:tunnelSecret "<BASE64_32_BYTE_SECRET>"
pulumi config set --secret short-origin:surrealdbRootPassword "<PASSWORD>"
```

4. Verify, preview, and deploy:
```bash
cd ../..
cargo infra-verify
cargo infra-preview --stack dev
cargo infra-preview --stack stage
./scripts/deploy.sh dev
```

The Cargo aliases provide the short day-to-day DX surface, and `cargo x infra ...` exposes the full underlying `xtask` commands when you need the explicit form. The npm workspace scripts remain the implementation detail used by those wrappers and by deployment-only flows.

## Required Environment Variables

- `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_REGION` (or OIDC in CI)
- `CLOUDFLARE_API_TOKEN`

## CI/CD

GitHub Actions workflows:
- `.github/workflows/ci.yml`
- `.github/workflows/delivery-dev.yml`
- `.github/workflows/release-candidate.yml`
- `.github/workflows/promote-release.yml`

These workflows run stack-specific previews/applies against the S3 Pulumi backend and promote
digest-pinned manifests across `dev`, `stage`, and `production`.
