# Contributing

This is a personal Rust monorepo owned by [Biswajit Nanda](https://github.com/biswajitNanda223).

Keep changes small, named well, and easy to roll back. The Git history should explain the project without archaeology.

## Local Setup

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/bootstrap-env.ps1
powershell -ExecutionPolicy Bypass -File ./scripts/install-git-hooks.ps1
```

## Required Checks

Run before opening a pull request or pushing to `main`:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/check.ps1
```

Commit through the helper when possible:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/commit.ps1 -Message "docs: update git workflow"
```

The script runs:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`

## Change Rules

- Keep app code inside `apps/`.
- Keep shared Rust crates inside `crates/` only after real reuse exists.
- Keep production-like infra inside `infra/`.
- Keep repeatable commands inside `scripts/`.
- Never commit real `.env`, `podman.env`, dumps, generated secrets, or `target/`.
- Commit messages must follow Conventional Commits.

## Pull Request Checklist

- Docs updated when commands, ports, routes, env vars, or deployment behavior change.
- CI passes.
- New behavior has tests where practical.
- Secret values are not present in diff.
- Rollback path is obvious for production-impacting changes.
