# Agent Guide: CareerForge API

## Scope

Applies to `apps/careerforge-api`.

## Service Contract

- Package name: `careerforge-api`.
- Default port: `8080`.
- Health endpoint: `GET /health`.
- Container image should run as non-root user.
- Real `podman.env` and `.env` files are local only. Commit examples only.

## Implementation Rules

- Keep Axum routes in `src/routes.rs`.
- Keep shared app state in `src/state.rs`.
- Keep request/response/domain structs in `src/models.rs`.
- Keep HTTP error mapping in `src/error.rs`.
- Add route tests for changed CRUD behavior.

## Commands

From repo root:

```bash
cargo fmt --all --check
cargo clippy -p careerforge-api --all-targets -- -D warnings
cargo test -p careerforge-api
podman build -f apps/careerforge-api/Containerfile -t careerforge-api:latest .
```
