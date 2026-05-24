# CareerForge API

Resume-ready Rust REST API showing production habits without hiding behind a framework. It includes typed routing, request validation, structured errors, async state, JSON tracing, tests, Podman packaging, and CI.

## Stack

- Rust 1.95, edition 2024
- Axum 0.8 and Tokio
- Serde, Validator, UUID v7, Chrono
- Tower HTTP middleware for tracing, CORS, and timeouts
- Multi-stage Podman build with a small Debian runtime

## Run

From monorepo root:

```bash
cargo run -p careerforge-api
```

Local env template:

```bash
cp apps/careerforge-api/.env.example apps/careerforge-api/.env
cp apps/careerforge-api/podman.env.example apps/careerforge-api/podman.env
```

On Windows, start Podman machine first if needed:

```bash
podman machine init
podman machine start
```

From monorepo root:

```bash
podman build -f apps/careerforge-api/Containerfile -t careerforge-api:latest .
podman run -d --rm --name careerforge-api --env-file apps/careerforge-api/podman.env -p 8080:8080 careerforge-api:latest
```

API listens on `http://localhost:8080`.

Stop container:

```bash
podman stop careerforge-api
```

## Endpoints

```text
GET    /health
GET    /metrics
GET    /projects
GET    /projects?status=active
POST   /projects
GET    /projects/{id}
PATCH  /projects/{id}
DELETE /projects/{id}
```

Create project:

```bash
curl -X POST http://localhost:8080/projects \
  -H "content-type: application/json" \
  -d '{
    "title": "Rust Deployment API",
    "summary": "Shows validated JSON workflows and production-friendly Podman packaging.",
    "stack": ["Rust", "Axum", "Podman"],
    "status": "active",
    "impact_score": 88
  }'
```

## Local Quality Gates

From monorepo root:

```bash
cargo fmt --all --check
cargo clippy -p careerforge-api --all-targets -- -D warnings
cargo test -p careerforge-api
```

Without local Rust, use Podman:

```bash
podman build -f apps/careerforge-api/Containerfile -t careerforge-api:local .
podman run --rm -p 8080:8080 careerforge-api:local
```

## Future Linux Deployment

This repo includes a Quadlet unit at `deploy/podman/careerforge-api.container`.

```bash
mkdir -p ~/.config/containers/systemd ~/.config/careerforge-api
cp deploy/podman/careerforge-api.container ~/.config/containers/systemd/
cp apps/careerforge-api/podman.env.example ~/.config/careerforge-api/podman.env
systemctl --user daemon-reload
systemctl --user enable --now careerforge-api
```

## Resume Talking Points

- Built async REST API in Rust using Axum and Tokio.
- Added typed domain models, validation, structured error responses, and health/metrics endpoints.
- Packaged service with multi-stage Podman build and non-root runtime user.
- Added CI workflow for formatting, linting, tests, and container build.
