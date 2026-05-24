# Interstellar

Personal Rust monorepo for API experiments, portfolio-grade services, and learning projects.

The name is borrowed from the movie because this repo is for serious Rust systems with room to grow.

## Repository

| Field | Value |
| --- | --- |
| Owner | [Biswajit Nanda](https://github.com/biswajitNanda223) |
| GitHub | [`@biswajitNanda223`](https://github.com/biswajitNanda223) |
| Remote | `https://github.com/biswajitNanda223/interstellar` |
| Branch | `main` |
| Style | Small commits, protected history, no secrets |

## Projects

| Project | Path | Purpose | Port |
| --- | --- | --- | --- |
| CareerForge API | `apps/careerforge-api` | Resume/project portfolio REST API with validation, structured errors, tracing, tests, and Podman deployment files. | `8080` |
| Task Manager API | `apps/task-manager-api` | Learning-oriented Axum REST API for task CRUD, shared state, and container basics. | `3000` |

## Layout

```text
interstellar/
  apps/
    careerforge-api/
    task-manager-api/
  infra/
    postgres/
  scripts/
  docs/
    features/
  .github/workflows/
  Cargo.toml
  docker-compose.yml
  AGENTS.md
```

## Quick Start

Create local env from templates:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/bootstrap-env.ps1
powershell -ExecutionPolicy Bypass -File ./scripts/install-git-hooks.ps1
```

Run checks for every workspace crate:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/check.ps1
```

Run one app locally:

```bash
cargo run -p careerforge-api
cargo run -p task_manager_api
```

Run both apps with containers:

```bash
podman compose --env-file .env up --build
```

If using Docker Compose instead:

```bash
docker compose --env-file .env up --build
```

## Production Hygiene

- Never commit `.env`, `podman.env`, secret files, database data, or generated `target/`.
- Use `.env.example` and app `*.env.example` files as templates only.
- Rotate `POSTGRES_PASSWORD` and `DATABASE_URL` outside Git.
- Use `scripts/check.ps1` before pushing.
- Use `infra/postgres/init/` only for safe bootstrap SQL, not destructive migrations.

## Workspace Rules

- Keep runnable services in `apps/`.
- Add shared Rust libraries in `crates/` when two apps need the same code.
- Keep app-specific deployment files inside the app folder unless they orchestrate multiple apps.
- Keep root docs focused on cross-repo guidance; app READMEs explain app-specific behavior.
- Do not commit generated `target/` output.

## Docs

- [Architecture](docs/architecture.md)
- [Operations](docs/operations.md)
- [Security](docs/security.md)
- [Git and GitHub](docs/git.md)
- [Production readiness](docs/production.md)
- [CareerForge API feature doc](docs/features/careerforge-api.md)
- [Task Manager API feature doc](docs/features/task-manager-api.md)
