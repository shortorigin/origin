# GitHub Governance Rollout

This repository is the pilot adopter for the `shortorigin` GitHub-native Scrumban model.

## Canonical Config

- GitHub governance config: [`.github/governance.toml`](/Users/justinshort/short%20origin/.github/governance.toml)
- Issue forms: [`.github/ISSUE_TEMPLATE`](/Users/justinshort/short%20origin/.github/ISSUE_TEMPLATE)
- PR template: [`.github/PULL_REQUEST_TEMPLATE.md`](/Users/justinshort/short%20origin/.github/PULL_REQUEST_TEMPLATE.md)
- Development policy: [DEVELOPMENT_MODEL.md](/Users/justinshort/short%20origin/DEVELOPMENT_MODEL.md)

## Bootstrap Commands

Dry-run the organization and repository sync plans:

```bash
cargo xtask github sync org --config .github/governance.toml --dry-run
cargo xtask github sync repo --config .github/governance.toml --repository shortorigin/short-origin --dry-run
```

Apply the GitHub settings with `gh` once authenticated:

```bash
cargo xtask github sync org --config .github/governance.toml --apply
cargo xtask github sync repo --config .github/governance.toml --repository shortorigin/short-origin --apply
```

## Organization Setup

1. Create the public `shortorigin/.github` repository.
2. Copy the canonical community-health files, issue forms, PR template, and governance config into that repository.
3. Run `cargo xtask github sync org ... --apply` from the seeded checkout.

The org sync covers:

- ensuring the public `.github` repository exists
- ensuring the `Engineering Flow` organization Project exists
- ensuring the `Status` single-select field exists with the standard options
- linking configured repositories to the project

## Repository Setup

Run the repo sync for each repository that adopts the model.

The repo sync covers:

- repository merge settings
- labels
- milestones
- repository rulesets for `main` protection and branch naming
- pull-request review and thread-resolution requirements sourced from [`.github/governance.toml`](/Users/justinshort/short%20origin/.github/governance.toml)
- auto-merge enablement for protected-branch fallback

Repository workflow standards that should be present after rollout:

- `main` is protected and requires PR-based merges.
- branch names follow the approved issue-driven prefixes and include an issue id.
- required checks are `Governance / validate`, `CI / pr-gate`, and `Security / security-gate`.
- release and environment promotion run only through GitHub Actions workflows.
- issue forms, PR templates, CODEOWNERS, and workflow ownership docs are committed in-repo.

## Manual GitHub UI Steps

The GitHub CLI currently does not cover all project-view and workflow configuration. After the sync runs:

1. Create saved repository views for each linked repository.
2. Create saved milestone views for each standard milestone.
3. Enable built-in Project workflows so new issues land in `Backlog`.
4. Configure status transitions so linked pull requests move issues through `PR Open`, `Review`, and `Done`.
5. Enable merge queue for `main` if the plan supports it.
6. Create GitHub environments `dev`, `stage`, and `production`.
7. Configure `production` to require `@shortorigin/core-maintainers` approval and disable self-approval.
8. Enable secret scanning, push protection, and private vulnerability reporting.
9. Review Actions policy so GitHub-authored actions are allowed and any non-GitHub helper logic is mirrored locally under `.github/actions/`.
10. Confirm environment secrets exist for `AWS_ROLE_TO_ASSUME`, `PULUMI_STATE_BUCKET`, and `CLOUDFLARE_API_TOKEN`.

## Required Checks

Configure the `main` ruleset to require these check names:

- `Governance / validate`
- `CI / pr-gate`
- `Security / security-gate`

## Environment Secrets and Vars

Create the same secret names in each environment where needed:

- `AWS_ROLE_TO_ASSUME`
- `PULUMI_STATE_BUCKET`
- `CLOUDFLARE_API_TOKEN`

`dev` deploys automatically from `main`, `stage` is reserved for release candidates, and
`production` is reserved for approved final releases.

## Ongoing Maintenance

When repository process changes:

1. Update [AGENTS.md](/Users/justinshort/short%20origin/AGENTS.md) with policy or boundary changes.
2. Update [DEVELOPMENT_MODEL.md](/Users/justinshort/short%20origin/DEVELOPMENT_MODEL.md) and [CONTRIBUTING.md](/Users/justinshort/short%20origin/CONTRIBUTING.md) with contributor-facing procedure changes.
3. Update workflow ownership notes in [docs/process/github-actions-supply-chain.md](/Users/justinshort/short%20origin/docs/process/github-actions-supply-chain.md) when Actions dependencies or mirroring strategy changes.
4. Re-run the `cargo xtask github sync ... --dry-run` commands if rulesets or governance config changed.
