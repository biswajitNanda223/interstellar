# Agent Guide: Infra

## Scope

Applies to `infra/`.

## Rules

- Commit examples and bootstrap definitions only.
- Do not commit database data, dumps, credentials, or generated secret files.
- Keep local services close to production defaults without hardcoding production secrets.
- SQL under `postgres/init/` must be safe for first boot only.
- SQL under `postgres/migrations/` must be idempotent unless a migration tool with tracking is added.
