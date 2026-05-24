# Agent Guide: Task Manager API

## Scope

Applies to `apps/task-manager-api`.

## Service Contract

- Package name: `task_manager_api`.
- Default port: `3000`.
- Main resource: tasks.
- Real `.env` files are local only. Commit examples only.

## Implementation Rules

- Keep learning comments short and correct.
- Keep handlers in `src/handlers.rs`.
- Keep shared state and DTOs in `src/models.rs`.
- Keep error response mapping in `src/error.rs`.
- Prefer small tests before expanding functionality.

## Commands

From repo root:

```bash
cargo fmt --all --check
cargo clippy -p task_manager_api --all-targets -- -D warnings
cargo test -p task_manager_api
podman build -f apps/task-manager-api/Dockerfile -t task-manager-api:latest .
```
