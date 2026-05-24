# Architecture

Interstellar is a Rust workspace with independent applications under `apps/`.

## Current Boundary

- Each app owns its own `Cargo.toml`, source tree, runtime port, and container build file.
- The root workspace owns cross-project commands and CI.
- `infra/` owns local production-like dependencies such as Postgres.
- `scripts/` owns repeatable developer and CI helper commands.
- Shared crates are intentionally absent until real reuse appears.

## Dependency Policy

Apps may use different framework versions while they evolve. Standardize versions only when maintenance cost becomes real.

Current examples:

- `careerforge-api` uses Axum 0.8 and Rust edition 2024.
- `task_manager_api` uses Axum 0.7 and Rust edition 2021.

## Future Growth

Use this shape when adding projects:

```text
apps/new-api/
  Cargo.toml
  src/
  README.md
  AGENTS.md
```

Use this shape when extracting shared code:

```text
crates/api-common/
  Cargo.toml
  src/lib.rs
  README.md
```

Only extract shared code after two projects need it.

## Runtime Config

Configuration comes from environment variables. Secret values must be supplied by local `.env`, deployment secret stores, or CI secrets. Committed files may contain only examples.

Current database scaffold:

- Postgres service name: `postgres`
- Database: `interstellar`
- Default local port: `5432`
- Bootstrap SQL: `infra/postgres/init/001-init.sql`
