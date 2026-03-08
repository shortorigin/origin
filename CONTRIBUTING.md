# Contributing

## GitHub Workflow Protocol

All material changes are issue-driven and must follow the same GitHub workflow:

1. Create or refine a same-repository GitHub issue before writing code.
2. Ensure the issue captures:
   - the change, defect, or enhancement being proposed,
   - enough background and scope for another contributor to understand the request,
   - explicit acceptance criteria,
   - supporting technical notes, constraints, and related links where needed.
3. Move the issue onto the organization Project in the correct `Status` column.
4. Fetch the current remote branch tips before creating a working branch: `git fetch origin`.
5. Create a dedicated short-lived branch from `origin/main`, not from a stale local `main`.
6. Name the branch with the issue identifier using:
   - `feature/<issue-id>-description`
   - `fix/<issue-id>-description`
   - `infra/<issue-id>-description`
   - `docs/<issue-id>-description`
   - `refactor/<issue-id>-description`
   - `research/<issue-id>-description`
7. Implement the change on that issue branch and keep the change set small enough to review and merge quickly.
8. If the work is stacked, create the child branch from its parent branch and open the child PR against the parent branch until the base PR lands.
9. Rebase on the current target branch before requesting merge, and rebase again if the target branch moves while the PR is open.
10. Open a pull request targeting `main` or the parent branch in a stack that:
   - references the originating issue,
   - explains the technical change,
   - documents the testing strategy,
   - includes a closing directive such as `Closes #<issue-id>`.
11. Merge only after review and required checks pass so GitHub automatically closes the linked issue.

Example commands:

```bash
gh issue create
git fetch origin
git switch -c fix/123-runtime-error-contract origin/main
gh pr create --fill
```

## Pull Request Requirements

- No direct commits to `main`.
- PR titles must use conventional commits: `type(scope): description`.
- The PR body must reference a same-repository issue with `Closes #<issue-id>` or an equivalent issue URL.
- Source files, docs, UI strings, fixtures, generated assets, commit messages, PR text, and issue text must use repository/product terminology only; do not leak Codex, OpenAI, ChatGPT, or other assistant/vendor branding unless a third-party reference or legal attribution requires it.
- Required checks must pass before merge.
- At least one reviewer approval is required before merge.
- Squash merge is the default merge strategy.
- Keep the `Closes #<issue-id>` directive in the PR body through merge so the issue closes automatically.
- If the PR is stacked, target the parent branch until the base PR merges.
- If the PR touches `ui/crates/desktop_runtime`, `ui/crates/system_ui`, or `ui/crates/site/src/generated`, rebase on the current target branch immediately before requesting merge.
- Regenerate derived assets after the last rebase and before review whenever the PR changes token, shell, or generated CSS inputs.
- Do not mix unrelated shell refactors, generated asset churn, and behavioral fixes into one PR when they can be reviewed separately.

## Labels and Milestones

Use the standard label taxonomy:

- `type:feature`
- `type:bug`
- `type:refactor`
- `type:docs`
- `type:infra`
- `type:research`
- `priority:low`
- `priority:medium`
- `priority:high`
- `priority:critical`

Use milestones to group issues into release objectives such as `v0.1 - MVP` and `v1.0 - Production`.

## Verification

Run the baseline checks from the repository root:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo xtask github audit-process
```

If your change affects dependency security posture, validate `cargo audit` locally or rely on the `Security Scan` workflow in CI.

The required GitHub checks are:

- `Governance / validate`
- `CI / pr-gate`
- `Security / security-gate`

During self-review, also remove assistant-generated branding, boilerplate provenance notes, and tool-specific placeholder text before requesting review.
