# Operations

Run commands from the monorepo root unless noted.

## Environment

Create `.env` from the committed template:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/bootstrap-env.ps1
```

The generated `.env` contains a local random Postgres password and is ignored by Git.

## Rust Checks

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/check.ps1
```

If local `cargo` is missing, the script falls back to Docker with the pinned Rust toolchain.

## Local Runs

```bash
cargo run -p careerforge-api
cargo run -p task_manager_api
```

## Containers

Start local stack with Postgres:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/compose-up.ps1
```

Stop local stack:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/compose-down.ps1
```

Build one image manually:

```bash
podman build -f apps/careerforge-api/Containerfile -t careerforge-api:latest .
podman build -f apps/task-manager-api/Dockerfile -t task-manager-api:latest .
```

Open Postgres shell:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/db-shell.ps1
```

Apply idempotent SQL migrations:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/db-migrate.ps1
```

## Ports

- CareerForge API: `http://localhost:8080`
- Task Manager API: `http://localhost:3000`
- Postgres: `localhost:5432`

## Secrets

- `.env` is local only and ignored.
- `.env.example` is safe to commit.
- Use CI/deployment secret stores for production values.
- Do not commit database dumps or generated secret files.

## CI

CI runs:

- format check
- clippy
- workspace tests
- Podman image builds for both apps
