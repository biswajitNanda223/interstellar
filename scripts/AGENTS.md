# Agent Guide: Scripts

## Scope

Applies to `scripts/`.

## Rules

- Scripts must be idempotent where practical.
- Do not print secret values.
- PowerShell scripts are first-class because this workspace is on Windows.
- Bash scripts may mirror PowerShell behavior for Linux CI/dev machines.
- Commit helper scripts must run checks before calling `git commit`.
- GitHub repo creation scripts must not print tokens.
