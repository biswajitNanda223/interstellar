# Agent Guide: Interstellar

## Scope

This file applies to the whole monorepo. More specific `AGENTS.md` files inside `apps/` and `docs/` override this guidance for their paths.

## Repository Shape

- Owner GitHub profile: `https://github.com/biswajitNanda223`.
- Intended remote: `https://github.com/biswajitNanda223/interstellar.git`.
- `apps/careerforge-api`: production-style Axum service.
- `apps/task-manager-api`: learning-oriented Axum service.
- `docs`: cross-repo docs and feature notes.
- `infra`: local production-like dependencies.
- `scripts`: repeatable repo commands.
- `Cargo.toml`: workspace membership and shared repo metadata.

## Working Rules

- Run workspace checks from repo root when touching shared config: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- Run package-scoped checks for narrow app-only changes: `cargo test -p careerforge-api` or `cargo test -p task_manager_api`.
- Keep `.githooks/` and `.gitmessage` aligned with `docs/git.md`.
- Keep generated files out of commits, especially `target/`.
- Never commit real `.env`, `podman.env`, database dumps, or secret material.
- Keep GitHub ownership docs and CODEOWNERS aligned with `@biswajitNanda223`.
- Prefer app-local dependencies until at least two apps need the same abstraction.
- Add shared crates under `crates/` only when duplication is real and stable.

## Rust Style

- Keep handlers thin; push domain/state behavior into named functions or modules when it grows.
- Return typed errors instead of stringly HTTP responses.
- Keep route lists easy to scan.
- Use structured logs for production-facing services.

## Container Style

- App images build from app directories.
- Root `docker-compose.yml` is only for multi-service local orchestration.
- Keep Podman Quadlet files near the service they run.
