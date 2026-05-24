# CareerForge API

Production-leaning portfolio API for resume/project data.

## Source

- App: `apps/careerforge-api`
- Package: `careerforge-api`
- Runtime port: `8080`
- Container file: `apps/careerforge-api/Containerfile`
- Env template: `apps/careerforge-api/.env.example`
- Future database schema: `careerforge`

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

## Quality Bar

- Keep request validation explicit.
- Keep error responses typed.
- Keep route tests for main CRUD flows.
- Keep deployment docs current with Podman files.
- Keep real env values outside Git.
