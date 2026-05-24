# Production Readiness

This repo is structured for production habits even while the apps are still evolving.

## Current State

- Rust workspace with independent app packages.
- CI for formatting, clippy, tests, and container builds.
- Postgres local dependency with healthcheck.
- Secure env templates and ignored real env files.
- Scripts for repeatable checks, compose lifecycle, and database access.

## Required Before Real Production

- Replace in-memory storage with database-backed repositories.
- Add real database migrations with a migration tool such as `sqlx` or `refinery`.
- Add request tracing correlation IDs.
- Add rate limits and stricter CORS.
- Add OpenAPI docs or checked API examples.
- Add app health checks that verify database connectivity.
- Add deployment-specific secret management.
- Add image vulnerability scanning in CI.
- Add release tags and rollback notes.

## Deployment Baseline

- Build images from monorepo root.
- Run services as non-root where possible.
- Inject secrets at runtime.
- Use managed Postgres or a hardened self-hosted Postgres.
- Protect `main` and deploy only from CI-built artifacts.

