# Agent Guide: CareerForge Source

## Scope

Applies to Rust source for CareerForge API.

## Feature Notes

- Project CRUD lives in `routes.rs`.
- In-memory seeded storage lives in `state.rs`.
- Validation uses `validator` derive attributes in `models.rs`.
- Public behavior should be covered by handler/router tests.

