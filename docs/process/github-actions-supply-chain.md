# GitHub Actions Supply Chain Ownership

This repository keeps GitHub Actions policy narrow by owning the implementation for third-party workflow helpers that are not GitHub-authored.

This document complements [AGENTS.md](/Users/justinshort/short%20origin/AGENTS.md), [DEVELOPMENT_MODEL.md](/Users/justinshort/short%20origin/DEVELOPMENT_MODEL.md), and [CONTRIBUTING.md](/Users/justinshort/short%20origin/CONTRIBUTING.md) by defining the operational conventions for workflow ownership.

## Repository-Owned Mirrors

The following third-party Marketplace actions are replaced by local composite actions under [`.github/actions`](/Users/justinshort/short%20origin/.github/actions):

- `dtolnay/rust-toolchain@stable` -> [`.github/actions/install-rust/action.yml`](/Users/justinshort/short%20origin/.github/actions/install-rust/action.yml)
- `Swatinem/rust-cache@v2` -> removed; workflows now run without third-party cargo cache logic
- `oras-project/setup-oras@v1` -> [`.github/actions/install-oras/action.yml`](/Users/justinshort/short%20origin/.github/actions/install-oras/action.yml)
- `aws-actions/configure-aws-credentials@v4` -> [`.github/actions/configure-aws-oidc/action.yml`](/Users/justinshort/short%20origin/.github/actions/configure-aws-oidc/action.yml)
- `pulumi/setup-pulumi@v3` -> [`.github/actions/install-pulumi/action.yml`](/Users/justinshort/short%20origin/.github/actions/install-pulumi/action.yml)

GitHub-authored actions such as `actions/checkout`, `actions/setup-node`, `actions/upload-artifact`, `actions/dependency-review-action`, and `github/codeql-action` remain in use because they are platform-native integrations and do not require repository allow-list exceptions.

## Pinned Tooling

The local mirrors install pinned release artifacts and verify the official SHA-256 checksums before exposing binaries on `PATH`.

- ORAS `1.3.0`
  - asset: `oras_1.3.0_linux_amd64.tar.gz`
  - SHA-256: `6cdc692f929100feb08aa8de584d02f7bcc30ec7d88bc2adc2054d782db57c64`
- Pulumi CLI `3.225.0`
  - asset: `pulumi-v3.225.0-linux-x64.tar.gz`
  - SHA-256: `63d0cb6ed5a6c1d7c398d34f97759bc8538ca6e800adb829d993135d5a1d3aab`

## Audit Procedure

When updating one of the mirrored tools:

1. Review the upstream repository release notes and recent maintenance activity.
2. Download the official checksum file from the tagged release.
3. Update the pinned version and checksum in the local composite action.
4. Run the affected workflow commands locally where practical.
5. Open a PR that documents the upstream version bump and checksum change.

Example commands:

```bash
gh release download v1.3.0 -R oras-project/oras -p 'oras_1.3.0_checksums.txt'
gh release download v3.225.0 -R pulumi/pulumi -p 'pulumi-3.225.0-checksums.txt'
```

## Governance Notes

- Local action changes must trigger CI. [`.github/scripts/detect_changes.sh`](/Users/justinshort/short%20origin/.github/scripts/detect_changes.sh) treats `.github/actions/` as workflow-affecting input.
- AWS credentials are sourced through GitHub OIDC at runtime and are never stored as long-lived static secrets in the repository.
- Third-party action logic should be mirrored only when the repository can audit and maintain the resulting shell logic.
- Workflow changes must be reflected in the contributor-facing process docs when they alter required checks, release flow, or branch protection assumptions.
