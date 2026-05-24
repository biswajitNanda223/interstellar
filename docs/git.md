# Git And GitHub

This repo treats Git history like production surface area: small commits, clear names, protected `main`, and no secret leaks.

## Identity

| Field | Value |
| --- | --- |
| Owner | [Biswajit Nanda](https://github.com/biswajitNanda223) |
| GitHub handle | `@biswajitNanda223` |
| Repository | `https://github.com/biswajitNanda223/interstellar` |
| Remote | `origin` |
| Default branch | `main` |
| Code owner | `@biswajitNanda223` |

## First Launch

Create the GitHub repository first, then launch the local repo:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/install-git-hooks.ps1
git add .
powershell -ExecutionPolicy Bypass -File ./scripts/commit.ps1 -Message "chore: bootstrap rust monorepo"
git push -u origin main
```

If the GitHub repo name changes:

```bash
git remote set-url origin https://github.com/biswajitNanda223/<repo-name>.git
```

## Branching

`main` is production-aligned. Keep it boring, protected, and always recoverable.

Use short branches:

```text
feat/project-search
fix/task-validation
docs/postgres-ops
chore/ci-cache
security/env-hardening
```

Rules:

- Branch from latest `main`.
- Keep pull requests focused.
- Squash merge when the branch has noisy work-in-progress commits.
- Tag stable deployable points after CI passes.

## Commit Voice

Commits should read like a clean flight log. One line says what changed. Body explains why only when needed.

Good:

```text
feat(careerforge): add project search endpoint
fix(task-manager): validate empty task titles
docs: polish git workflow
chore(ci): build app images in workflow
security(env): block committed local secrets
```

Avoid:

```text
updated stuff
fix
final changes
work in progress
```

Allowed types:

```text
feat, fix, docs, chore, refactor, test, ci, build, perf, security, revert
```

Subject rules:

- 72 characters or less.
- Lowercase type.
- Optional lowercase scope.
- No trailing period.
- Use imperative tone.

## Local Guardrails

Install once:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/install-git-hooks.ps1
```

This sets:

```text
core.hooksPath=.githooks
commit.template=.gitmessage
```

Pre-commit runs:

- secret and database file guard
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`

If local `cargo` is missing, the check script uses Docker as fallback.

Commit message hook enforces Conventional Commits.

Preferred commit path:

```bash
powershell -ExecutionPolicy Bypass -File ./scripts/commit.ps1 -Message "docs: polish git workflow"
```

Emergency bypass:

```bash
git commit --no-verify
```

Use bypass only when the hook itself is broken and the follow-up fix is immediate.

## GitHub Settings

Turn on these before treating the repo as production-grade:

- Branch protection for `main`.
- Required pull request before merge.
- Required CI status checks.
- CODEOWNERS review.
- Secret scanning and push protection.
- Dependabot alerts and security updates.
- Squash merge enabled.
- Direct pushes to `main` disabled.

Suggested topics:

```text
rust, axum, tokio, podman, postgres, monorepo, api
```
