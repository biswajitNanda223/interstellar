# Security Policy

Owner: [Biswajit Nanda](https://github.com/biswajitNanda223)

## Reporting

For private security issues, do not open a public issue with exploit details. Contact the repository owner through GitHub profile links or a private channel.

## Scope

Security-sensitive areas:

- environment variable handling
- database credentials and migrations
- container images
- CI/CD configuration
- dependency upgrades
- API input validation and error responses

## Baseline Rules

- Commit templates only: `.env.example`, `*.env.example`, `podman.env.example`.
- Never commit real `.env`, `podman.env`, database dumps, backups, generated secret files, or `target/`.
- Do not bake secrets into container images.
- Use deployment or CI secret stores for production values.
- Review database migrations before applying them outside local development.

More detail lives in [docs/security.md](docs/security.md).

