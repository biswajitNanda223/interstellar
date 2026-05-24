# Task Manager API

Learning-oriented Rust REST API built with Axum, Tokio, Serde, and Tower HTTP.

This app is intentionally simpler than `careerforge-api`. It is useful for practicing Rust web fundamentals: ownership, shared state, typed errors, async handlers, and container packaging.

## Project Layout

```text
apps/task-manager-api/
  Cargo.toml
  Dockerfile
  src/
    main.rs
    models.rs
    error.rs
    handlers.rs
```

## Run

From monorepo root:

```bash
cargo run -p task_manager_api
```

The API listens on `http://localhost:3000`.

Local env template:

```bash
cp apps/task-manager-api/.env.example apps/task-manager-api/.env
```

## Run with Podman

From monorepo root:

```bash
podman build -f apps/task-manager-api/Dockerfile -t task-manager-api:latest .
podman run -d --rm --name task-manager-api -p 3000:3000 task-manager-api:latest
```

Stop it:

```bash
podman stop task-manager-api
```

## Endpoints

```text
GET    /tasks
POST   /tasks
GET    /tasks/{id}
PATCH  /tasks/{id}
DELETE /tasks/{id}
```

Create task:

```bash
curl -X POST http://localhost:3000/tasks \
  -H "content-type: application/json" \
  -d '{"title":"Learn Podman","description":"Understand daemonless containers"}'
```

## Local Quality Gates

From monorepo root:

```bash
cargo fmt --all --check
cargo clippy -p task_manager_api --all-targets -- -D warnings
cargo test -p task_manager_api
```

## Learning Notes

- `Arc` and `RwLock` allow shared in-memory state across request handlers.
- Axum extractors pull path values, JSON payloads, and state into typed handler parameters.
- `IntoResponse` converts app errors into HTTP responses.
- A real production version should replace in-memory storage with a database and add tests.
