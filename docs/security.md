# Security

Owner: [Biswajit Nanda](https://github.com/biswajitNanda223)

## Secrets

- Commit templates only: `.env.example`, `*.env.example`, `podman.env.example`.
- Never commit `.env`, `podman.env`, database dumps, backups, or generated secret files.
- Generate local secrets with `powershell -ExecutionPolicy Bypass -File ./scripts/bootstrap-env.ps1`.
- Store production secrets in the deployment platform, CI secret store, or OS secret manager.

## Database

- Local Postgres runs through Compose with a named volume.
- Bootstrap SQL creates schemas and migration marker tables only.
- Real migrations live under `infra/postgres/migrations/` and are applied with `scripts/db-migrate.ps1`.
- Migrations should be reviewed and reversible where possible.
- Database credentials must be rotated per environment.

## Containers

- Build from the monorepo root so images use the workspace lockfile.
- Keep runtime images small.
- Run app containers as non-root when possible. `careerforge-api` already does this.
- Do not bake secrets into images.

## CI

CI must keep running:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Container builds should not require local secret files.
