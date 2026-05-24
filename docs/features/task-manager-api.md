# Task Manager API

Learning-focused Axum API for task CRUD and Rust web basics.

## Source

- App: `apps/task-manager-api`
- Package: `task_manager_api`
- Runtime port: `3000`
- Container file: `apps/task-manager-api/Dockerfile`
- Env template: `apps/task-manager-api/.env.example`
- Future database schema: `task_manager`

## Endpoints

```text
GET    /tasks
POST   /tasks
GET    /tasks/{id}
PATCH  /tasks/{id}
DELETE /tasks/{id}
```

## Quality Bar

- Keep teaching comments useful and accurate.
- Avoid adding production complexity unless the lesson needs it.
- Add tests before turning this into a serious app.
- Keep real env values outside Git.
